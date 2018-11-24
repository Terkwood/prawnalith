# Prawnalith Heads-Up Display

It's a small view of statuses for all the tanks.

## Overview

We use [yew framework](https://github.com/DenisKolodin/yew) to create a simple frontend which polls
the [pond service](/cloud_images/pond) for temp & pH data for all of the prawn tanks.

### Acknowledgements

We really appreciate the help from [this article, which shows how to properly wait on DOM elements coming into existence](https://swizec.com/blog/how-to-properly-wait-for-dom-elements-to-show-up-in-modern-browsers/swizec/6663).
