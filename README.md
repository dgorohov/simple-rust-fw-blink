### Pre-requisites:
 * Install any IDE which supports Rust. I recommend using Visual Studio Code,
   since it's easier to setup debugging using gdb/openocd. I am using CLion,
   I am just a big fan of JetBrains products, and it has very nice Rust plugin.
   Please note: CLion is a commercial product.

   Please follow this link: https://medium.com/coinmonks/coding-the-stm32-blue-pill-with-rust-and-visual-studio-code-b21615d8a20 if you wish using VSC.

### Rusty things:
   * Download nRF SDK 15.2 from https://www.nordicsemi.com/-/media/Software-and-other-downloads/SDKs/nRF5/Binaries/nRF5SDK15209412b96.zip
   and point the link in `includes/nrf/nRF5-sdk` to extracted folder
   ```
   # cd includes/nrf
   # ln -sf <extracted source> nRF5-sdk
   ```
   * Update rust or install it if not installed yet. To do so follow the single step on: https://rustup.rs/    
   * Install nightly toolchain:
   ```
   # rustup default nightly
   ```
   * Install non-std components for the ARM-Cortex targets:
   ```
   # rustup target add thumbv6m-none-eabi \
     thumbv7m-none-eabi \
     thumbv7em-none-eabi \
     thumbv7em-none-eabihf
   ```    
   * Install openocd / gdb

        - OpenOCD version 0.10.0 does not have support for nRF52840 device, at least the one I tried to install using brew.
          To fix it, if we under Mac, and you're brew user, please follow these steps:
            ```
            # brew edit open-ocd
            ```

        - And update the following lines to:
            ```
             def install
                ENV["CCACHE"] = "none"
            
                system "./bootstrap", "nosubmodule" if build.head?
                system "./configure", "--disable-dependency-tracking",
                                   "--prefix=#{prefix}",
                                   "--enable-buspirate",
                                   "--enable-stlink",
                                   "--enable-dummy",
                                   "--enable-bcm2835gpio",
                                   "--enable-jtag_vpi",
                                   "--enable-remote-bitbang"
                system "make", "install"
             end
            ```
        **NOTE:** It is important to have this flag set: **--enable-bcm2835gpio**
    
        - Under the Linux, grab the sources, and configure it using the following flags
            ```
            # ./configure --prefix=/usr/local \
                    --enable-jlink \
                    --enable-oocd_trace \
                    --enable-bcm2835gpio
            ```    
   * There're no such limitations/issues with **GDB** at the moment, so install it from any source you could.

### Happy coding!

If you found any error, or configuration missed you're welcome to open a PR. Thank you!

### Debugging from terminal

```
# make openocd
# cargo run
```

`cargo run` in this case would run a project in debug mode.

### Things TODO

- [ ] Change serial interface for the Simcom module, use the generic one, `embeded_hal` provides.
- [ ] Setup interrupt handler for the RX line to receive bytes from the simcom UART line.
- [ ] Create an abstraction(s) to handle AT commands.
- [ ] Would be nice to have a Finite State Machine abstractions.
      How? For example, having one state machine executor which would receive
      a state machine to invoke. The clock source is device configured clock.

