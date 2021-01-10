# Node Voltage Analyzer with Rust

This project uses the Kirchhoff's current law (KCL) to obtain the voltage at the nodes of the
circuit. Where the circuit is feeded to the program in the form of a netlist, similar to those
used in PSPICE-like software.


### Features

* Convention used to solve the circuit is KCL with incoming_current - leaving_current = 0
* It solves electric circuits with receives as an input a file containing the NetList of the circuit.
* It is capable of solving circuits with the following configurations:
** It is a linear circuit.
** It only has one or many independent voltage and/or current sources.
** It has one or many resistors.

### How it works

Everything is more clearly understood with an example.

** Figure 1. Circuit results/test4.txt **
![GitHub Logo](./results/test4.png)

Figure 1 shows us the circuit solved with the help pf Online Multisim.
A more detail schematic of how calculations are performed in the program are explained in the following figure.

** Figure 2. Circuit schematic with nodal currents **
![GitHub Logo](./img/schema4.png)

Figure 2 shows us the circuit by which we can build our netlist, once we label all our electric components and nodes and all current directions were set. Electric current must allways flow from ahigher potential to a lower one, so there is never a current which flows from node 0, or ground, to a higher potential.
With that in mind, the NetList of the figure 1 circuit results in:

** Tble 1: NetList of the circuit **
Component | Starting Node | Ending node | Value 
----------|---------------|-------------|-------
v1 | 3 | 0 | -10
i1 | 2 | 0 | -2
r1 | 1 | 0 | 2
r2 | 1 | 2 | 1
r3 | 2 | 3 | 2


