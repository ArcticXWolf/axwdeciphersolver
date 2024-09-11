# AXWDecipherSolver

This is a small rust program to solve monoalphabetic substitution ciphers.
It was made to solve the daily challenge of [Decipher](https://decipher.wtf)

## Building

Just build it normally with `cargo`:

```
cargo build [--release]
```

## Usage

You can run it as binary or via `cargo`:

```
cargo run "Aopz pz h alza mvy aol zbizapabapvu jpwoly kljvkly. Pa zovbsk mpuk tvza vm aol dvykz pu aopz zlualujlz. Thfil fvb jhu ptwyvcl pa?"

# results in:
AXWDecipherSolver

Ciphertext: AOPZ PZ H ALZA MVY AOL ZBIZAPABAPVU JPWOLY KLJVKLY. PA ZOVBSK MPUK TVZA VM AOL DVYKZ PU AOPZ ZLUALUJLZ. THFIL FVB JHU PTWYVCL PA?
Key: 
A[T,] 
B[U,] 
C[V,] 
D[F,W,] 
E[F,G,J,K,Q,W,X,Z,] 
F[Y,] 
G[F,J,K,Q,W,X,Z,] 
H[A,] 
I[B,] 
J[C,] 
K[D,] 
L[E,] 
M[F,K,] 
N[F,G,J,K,Q,W,X,Z,] 
O[H,] 
P[I,] 
Q[F,G,J,K,W,X,Z,] 
R[F,G,J,K,Q,W,X,Z,] 
S[L,] 
T[M,] 
U[N,] 
V[O,] 
W[P,] 
X[F,G,J,K,Q,W,Z,] 
Y[R,] 
Z[S,] 

Translation: THIS IS A TEST *OR THE SUBSTITUTION CIPHER DECODER. IT SHOULD *IND MOST O* THE *ORDS IN THIS SENTENCES. MAYBE YOU CAN IMPROVE IT?
                AOPZ | ["THIS"]
                  PZ | ["IS"]
                   H | ["A"]
                ALZA | ["TEST"]
                 MVY | ["FOR", "KOR"]
                 AOL | ["THE"]
        ZBIZAPABAPVU | ["SUBSTITUTION"]
              JPWOLY | ["CIPHER"]
             KLJVKLY | ["DECODER"]
                  PA | ["IT"]
              ZOVBSK | ["SHOULD"]
                MPUK | ["FIND", "KIND"]
                TVZA | ["MOST"]
                  VM | ["OF", "OK"]
                 AOL | ["THE"]
               DVYKZ | ["WORDS", "FORDS"]
                  PU | ["IN"]
                AOPZ | ["THIS"]
           ZLUALUJLZ | ["SENTENCES"]
               THFIL | ["MAYBE"]
                 FVB | ["YOU"]
                 JHU | ["CAN"]
             PTWYVCL | ["IMPROVE"]
                  PA | ["IT"]
```