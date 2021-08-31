# Multilinear Solver for Water Distribution Networks

**mlsolver** is a Rust implementation of the Multilinear method for hydraulic analysis of pipe networks, proposed by (Moosavian, 2017). The paper : Moosavian N, 2017. Multilinear method for hydraulic analysis of pipe networks. Journal of Irrigation and Drainage Engineering. Volume 143, number 8, pages={04017020, 2017, publisher: American Society of Civil Engineers.


## Development state
**mlsolver** is currently able to solve water distribution networks with the following components :

- [x] Junctions (nodes with demands),

- [x] Pipes :

   - Hazen-Williams head loss formula, 
   
   - Status = Open or Closed.

- [x] Tanks.

- [ ] Reservoirs.

- [x] Pumps (H-Q characteristic equation : $H = αQ^2 + βQ + γ$ equation).

- [ ] Valves.
 
   - [ ] CV : Check Valve.
   - [ ] FCV : Flow Control Valve.
   - [ ] PRV : Pressure Reducing Valve. 
   - [ ] PSV : Pressure Sustaining Valve.
   - [ ] TCV : Throttle Control Valve.
   - [ ] PBV : Pressure Breaker Valve.
   - [ ] GPV : General Purpose Valve.   

-[ ] EPS (Extended Period Simulation). 

## Usage 
  

## License

MIT license.


 