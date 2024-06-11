# RAT Controlling Command
## Get Sensor Data from All Boards
### Print Fromatted
```rust
rat-control -g/--get
```
### Print JSON
```rust
rat-control -g/--get --json
```

# RB (Readout Board) Controlling Command
## Get Sensor Data
### Print Fromatted
```rust
rat-control -b/--board rb -g/--get
```
### Print JSON
```rust
rat-control -b/--board rb -g/--get --json
```

# LTB (Local Trigger Board) Controlling Command
## Get Sensor Data
### Print Fromatted
```rust
rat-control -b/--board ltb -g/--get
```
### Print JSON
```rust
rat-control -b/--board ltb -g/--get --json
```

# PB (Power Board) Controlling Command
## Get Sensor Data
### Print Fromatted
```rust
rat-control -b/--board pb -g/--get
```
### Print JSON
```rust
rat-control -b/--board pb -g/--get --json
```

# PA (Preamp Board) Controlling Command
## Get Sensor Data
### Print Fromatted
```rust
rat-control -b/--board pa -g/--get
```
### Print JSON
```rust
rat-control -b/--board pa -g/--get --json
```
## Set SiPM Bias Voltage
### Set Same Voltage for All 16 Preamp Boards
```rust
rat-control -b/--board pa -s/--set <Voltage>
```
### Set Different Voltages for Each Preamp Boards
```rust
rat-control -b/--board pa -s/--set <PA1Vol,PA2Vol,PA3Vol,PA4Vol,PA5Vol,PA6Vol,PA7Vol,PA8Vol,PA9Vol,PA10Vol,PA11Vol,PA12Vol,PA13Vol,PA14Vol,PA15Vol,PA16Vol>
```