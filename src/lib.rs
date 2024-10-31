use connect4_board_library::Bitboard;
use nalgebra::{DMatrix, DVector};
use serde::{Deserialize, Serialize};
// use std::any::type_name;
// use std::any::TypeId;
use wasm_bindgen::prelude::*;

// JS functions that rust can call
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    pub fn alert(s: &str);
    pub fn win();
    pub fn ai_loaded_in_rust();
}

#[wasm_bindgen]
pub struct State {
    bitboard: Bitboard,
    ai: Option<NeuralNetwork>,
}

#[wasm_bindgen]
impl State {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let bitboard = Bitboard::new();
        return Self { bitboard, ai: None };
    }

    pub fn place(&mut self, column: usize) {
        self.bitboard.drop_piece(column);

        log(&format!("{}", self.bitboard));

        if self.bitboard.check_win() {
            // alert(&format!(
            //     "Player {} won",
            //     (self.bitboard.move_counter & 0) + 1
            // ));
            win();
        }
    }

    pub fn set_ai(&mut self, data: &str) {
        // let ai: NeuralNetwork =
        //     ron::from_str(data).expect("Failed to deserialize ai");
        // self.ai = Some(ai);

        log(&format!("Attempting to parse AI data: {}", &data[..100]));

        // match ron::from_str(data) {
        //     Ok(ai) => {
        //         self.ai = Some(ai);
        //         log(&"AI successfully parsed and set");
        //     }
        //     Err(e) => {
        //         log(&format!("Failed to parse AI data: {}", e));
        //         panic!("Failed to parse AI data: {}", e);
        //     }
        // }
        // Change from ron::from_str to serde_json::from_str
        match serde_json::from_str(data) {
            Ok(ai) => {
                self.ai = Some(ai);
                log("AI successfully parsed and set");
                ai_loaded_in_rust();
            }
            Err(e) => {
                log(&format!("Failed to parse AI data: {}", e));
                panic!("Failed to parse AI data: {}", e);
            }
        }
    }

    pub fn ai_place(&mut self) -> usize {
        // let ai = self.ai.unwrap();

        // ai.make_move(
        //     &mut self.bitboard,
        //     self.bitboard.move_counter & 1,
        //     1 - (self.bitboard.move_counter & 1),
        // )

        let move_counter = self.bitboard.move_counter;

        if let Some(ai) = self.ai.as_ref() {
            let column = ai.make_move(
                &mut self.bitboard,
                move_counter & 1,
                1 - (move_counter & 1),
            );

            log(&format!("{}", self.bitboard));
            log(&format!("ai placed in {}", column));

            if self.bitboard.check_win() {
                // alert(&format!(
                //     "Player {} won",
                //     (self.bitboard.move_counter & 0) + 1
                // ));
                win();
            }

            column
        } else {
            log("AI not initialised");
            panic!("AI not initialised");
        }
    }
}

//----------------------------------------------------------------------------\\
//----------------------------------------------------------------------------\\
//----------------------------------------------------------------------------\\

// #[typetag::serde(tag = "type")]
trait Participant {
    fn make_move(
        &self,
        game: &mut Bitboard,
        you: usize,
        opponent: usize,
    ) -> usize;
    // fn type_id(&self) -> TypeId;
    // fn adjust(&mut self, range: f64);
}

#[derive(Serialize, Deserialize)]
struct NeuralNetwork {
    #[serde(rename = "type")]
    type_name: String, // Add this to handle the "type" field in JSON
    id: u8,
    layers: Vec<DMatrix<f64>>, // Weights for each layer
    biases: Vec<DVector<f64>>, // Biases for each layer
}

impl NeuralNetwork {
    // fn new(layer_sizes: &[usize]) -> Self {
    //     let mut layers = Vec::new();
    //     let mut biases = Vec::new();

    //     let mut rng = rand::thread_rng();
    //     let uniform = Uniform::new(-1.0, 1.0);

    //     for i in 0..layer_sizes.len() - 1 {
    //         // // Randomly initialize weights and biases
    //         // let layer =
    //         //     DMatrix::<f64>::new_random(layer_sizes[i + 1], layer_sizes[i]);

    //         // Randomly initialize weights and biases
    //         let layer =
    //             DMatrix::from_fn(layer_sizes[i + 1], layer_sizes[i], |_, _| {
    //                 uniform.sample(&mut rng)
    //             });

    //         // let bias = DVector::<f64>::new_random(layer_sizes[i + 1]);

    //         let bias = DVector::from_fn(layer_sizes[i + 1], |_, _| {
    //             uniform.sample(&mut rng)
    //         });
    //         layers.push(layer);
    //         biases.push(bias);
    //     }

    //     NeuralNetwork {
    //         id: rng.gen::<u8>(),
    //         layers,
    //         biases,
    //     }
    // }

    fn forward(&self, input: &DVector<f64>) -> DVector<f64> {
        let mut output = input.clone();
        for (layer, bias) in self.layers.iter().zip(self.biases.iter()) {
            output = layer * output + bias; // Matrix multiplication and bias addition
            output = output.map(|x| x.max(0.0)); // Apply ReLU activation
        }
        output
    }
}

fn bitboard_to_input(player1: u64, player2: u64) -> DVector<f64> {
    let mut input = DVector::zeros(42); // 42 playable positions (6x7 grid)

    for i in 0..42 {
        let mask = 1u64 << i;
        if player1 & mask != 0 {
            input[i] = -1.0;
        } else if player2 & mask != 0 {
            input[i] = 1.0;
        }
        // If neither player has a piece here, it remains 0.0
    }

    input
}

fn sort_indices_by_values(arr: &DVector<f64>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..arr.len()).collect();

    // Sort the indices based on the corresponding values in the DVector, in
    // reverse for largest first
    indices.sort_by(|&i, &j| arr[j].partial_cmp(&arr[i]).unwrap());

    indices
}

// #[typetag::serde]
impl Participant for NeuralNetwork {
    fn make_move(
        &self,
        game: &mut Bitboard,
        you: usize,
        opponent: usize,
    ) -> usize {
        let input =
            bitboard_to_input(game.players[you], game.players[opponent]);
        let output = sort_indices_by_values(&self.forward(&input));
        for i in 0..7 {
            if game.drop_piece(output[i]) {
                return output[i];
            }
        }
        log("Found no columns with space");
        return 0;
    }

    // fn type_id(&self) -> TypeId {
    //     TypeId::of::<Self>()
    // }

    // fn adjust(&mut self, range: f64) {
    //     for (layer, bias) in self.layers.iter_mut().zip(self.biases.iter_mut())
    //     {
    //         layer.apply(|w| *w += range * (2.0 * rand::random::<f64>() - 1.0));
    //         bias.apply(|b| *b += range * (2.0 * rand::random::<f64>() - 1.0));
    //     }
    // }
}

// fn type_name_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// impl fmt::Debug for NeuralNetwork {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // All I really want it to print is the type name
//         write!(
//             f,
//             "{}, id: {}",
//             type_name_of(NeuralNetwork {
//                 id: 0,
//                 layers: Vec::new(),
//                 biases: Vec::new()
//             }),
//             self.id
//         )
//     }
// }
