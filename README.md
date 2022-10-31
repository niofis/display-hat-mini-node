# display-hat-mini-node

This library is a wrapper for the [display-hat-mini-driver](https://github.com/niofis/display-hat-min-driver), that allows the use of the [Pimoroni Display HAT Mini](https://shop.pimoroni.com/products/display-hat-mini?variant=39496084717651).

To install and build, please install the latest [Rust](https://www.rust-lang.org) language version. Which is required by [Neon](https://github.com/neon-bindings/neon), the bindings library used to enable access to `display-hat-mini-driver` from node.

## Usage

This is a simple example on how to use this library:

```js
const dhtm = require('display-hat-mini-node');
const width = dhtm.width();
const height = dhtm.height();
const data = new Uint8Array(width * height * 3);

dhtm.init(); //this is always needed

for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
        let offset = (i * width + j) * 3;
        data[offset] = i; //red
        data[offset + 1] = j; //green
        data[offset + 2] = 0; //blue
    }
}

const buffer = Buffer.from(data.buffer, data.byteOffset, data.byteLength);
dhtm.display(buffer);
```

## Features

### Bitmap drawing

```js
display(Buffer)
```

Call this function to draw a bitmap in the display. This bitmap should have 320x240 pixels where each pixel is 3 bytes, i.e. RGB24.



### VSync

```rust
setVSync(Boolean)
```

Instructs the driver to wait for the tear effect pin to become high before writing the bitmap data. It does not totally eliminate image tearing while drawing.

### Button states

```rust
readButtons()
```

The return value of this function is a flags byte indicating the state of the hardward buttons. For more information refer to this [explanation](https://github.com/niofis/display-hat-min-driver#button-states).

### RGB LED

```rust
setLED(red, green, blue)
```

Simply call this function to show the RGB in the integrated RGB LED.
