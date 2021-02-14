# Smart Light CLI

A program to unify the control of my smart lights, currently supports Phillips hue and Nanoleaf. 

## Commands

The simplest commands turn the lights on and off respectively: `smart-light-cli on` and `smart-light-cli off`. A light ID may be prefixed to the command to act on the given lights,

    smart-light-cli -l <lamp> on

where `<lamp>` is a comma separated list of light IDs. If this is not given, then default to values given in in the configuration file.

## Colour Commands
There are many ways to describe the colour of the lamp. This may be given as a HSV tuple: 

    smart-light-cli on -c <hue> <sat> <bri>
where the values are in the ranges given in the table below. The colour temperature of the light may also be given 

    smart-light-cli on -T <temp>
    
or the brightness directly set without changing the colour of the light

    smart-light-cli on <bri>

value | min | max
---|----|----
hue | 0 | 360
sat | 0 | 100
bri | 0 | 100
temp | 0 | 100



## Gradients
Two colours and a time may be given to the gradient command to give a transition between the two colours.

     smart-light-cli [-l <lamp>] gradient --start <c1> --end <c2> [-t <duration>]
     
 The colours are given as HSV triplets in the same range as above. 
 
 ## Scenes
 
 The lights may also accept named colours or profiles for more complex behaviour. As the names of the profiles are defined in the respective app for the smart light, these cannot be used interchangeably across the different brands. Although we do intend to support common named colours in a later release.
 
 ## Configuration
 
To enable the API control of the lights a IP address and API key must be included for each of the brands. See the respective API documentation for instructions on how to generate this key. NB, the Nanoleaf IP can be harder to locate, an nmap scan on the local network for the port `16021` may be helpful.
