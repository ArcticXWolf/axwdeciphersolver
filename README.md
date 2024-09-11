# AXWDecipherSolver

This is a small rust program to solve monoalphabetic substitution ciphers.
It was made to solve the daily challenge of [Decipher](https://decipher.wtf).

## Building

Just build it normally with `cargo`:

```
cargo build [--release]
```

## Usage

You can run it as binary or via `cargo`:

```
cargo run "Aol xbpjr iyvdu mve qbtwz vcly aol shgf kvn. Aopz pz h alza zlualujl mvy aol zbizapabapvu jpwoly kljvkly. Pa zovbsk mpuk tvza dvykz vm aopz tlzzhnl."
```

Result:
```
AXWDecipherSolver

Constructed key: 
A => [T,] 
B => [U,] 
C => [V,] 
D => [W,] 
E => [X,] 
F => [Y,] 
G => [Z,] 
H => [A,] 
I => [B,] 
J => [C,] 
K => [D,] 
L => [E,] 
M => [F,] 
N => [G,] 
O => [H,] 
P => [I,] 
Q => [J,] 
R => [K,] 
S => [L,] 
T => [M,] 
U => [N,] 
V => [O,] 
W => [P,] 
X => [Q,] 
Y => [R,] 
Z => [S,] 

Words found in ciphertext:
                 AOL | ["THE"]
               XBPJR | ["QUICK"]
               IYVDU | ["BROWN"]
                 MVE | ["FOX"]
               QBTWZ | ["JUMPS"]
                VCLY | ["OVER"]
                 AOL | ["THE"]
                SHGF | ["LAZY"]
                 KVN | ["DOG"]
                AOPZ | ["THIS"]
                  PZ | ["IS"]
                   H | ["A"]
                ALZA | ["TEST"]
            ZLUALUJL | ["SENTENCE"]
                 MVY | ["FOR"]
                 AOL | ["THE"]
        ZBIZAPABAPVU | ["SUBSTITUTION"]
              JPWOLY | ["CIPHER"]
             KLJVKLY | ["DECODER"]
                  PA | ["IT"]
              ZOVBSK | ["SHOULD"]
                MPUK | ["FIND"]
                TVZA | ["MOST"]
               DVYKZ | ["WORDS"]
                  VM | ["OF"]
                AOPZ | ["THIS"]
             TLZZHNL | ["MESSAGE"]

Ciphertext:  AOL XBPJR IYVDU MVE QBTWZ VCLY AOL SHGF KVN. AOPZ PZ H ALZA ZLUALUJL MVY AOL ZBIZAPABAPVU JPWOLY KLJVKLY. PA ZOVBSK MPUK TVZA DVYKZ VM AOPZ TLZZHNL.
Translation: THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG. THIS IS A TEST SENTENCE FOR THE SUBSTITUTION CIPHER DECODER. IT SHOULD FIND MOST WORDS OF THIS MESSAGE.
```