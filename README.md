# Connect4 AI Website
The front end to my connect 4 AI website. Integrates Rust web assembly with the website GUI. 

<img width="598" alt="Screenshot 2025-03-19 at 9 34 17 pm" src="https://github.com/user-attachments/assets/f06909f6-9cfc-4e45-8a5e-cf8ba7a0b301" />

The AI itself is not very bright, and even after 46 hours of non-stop training it is hardly capable of placing pieces intelligently. It was developed for an Inteligent Systems unit at my university (ICT206). There were no restrictions on the kind of Artificial Inteligence you could submit, some people were submitting decision trees, but I chose to challenge myself by making something that I found interesting.

See [connect4_nn_trainer](https://github.com/Dot32Dev/connect4_nn_trainer) for the training of the AI, and [barebones_connectfour](https://github.com/Dot32Dev/barebones_connectfour) for the connect four board library I wrote to interact with my neural network.

The training program is a gigantic mess, and involves manually commenting out different sections depending on whether you wish to train or to export a model, for example, or whether you want to print output of games as they are played (which considerably slows training speed). The board library is very neat code however, and I am proud of it.

The Rust side of this website uses my Connect Four board library and can import an exported AI from the training program from a string. Deserialising that file with Serde meant copying the struct fields and implementations from the training program, which is a very quick and dirty way of doing it. I should have abstracted my training program and this website to both use some sort of Connect Four neural network library, but I was running out of time to complete the project. 

I built the Rust half of this website and the part of the JavaScript that interacts with the user/Rust backend in just one day. I had previously designed the UI of the website when I chose to do this project, so it was quick to integrate it.

Here is a screenshot from my Assignment submission about the failure of the AI to learn

<img width="857" alt="Screenshot 2025-03-19 at 9 37 33 pm" src="https://github.com/user-attachments/assets/c6cbc0c5-61bd-4777-9fab-aef68d22ad5f" />

I guess i'm not majoring in AI for a reason lmao
