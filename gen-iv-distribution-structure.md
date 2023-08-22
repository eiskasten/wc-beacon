# Generation IV Distribution Structure

Disclaimer: This document is not written by any author of this repository.
It is the work of [Yuuto](https://gbatemp.net/members/yuuto.435475/) and is included here for the sake of completeness.
The original forum entry is
located [here](https://gbatemp.net/threads/reverse-engineering-pokemon-gen4-wonder-card-wi-fi-distribution.488212).
This application would not have been possible without this preliminary work.

## Preparation

The source material for distributing mystery gifts via Wi-Fi is a wonder card in PCD format.
It has a fixed size of 856 (`0x358`) bytes and can be divided into the following sections:

```
  0x0-0x103: [data1]  actual gift data (PGT)
0x104-0x153: [header] card title, card index, supported games
0x154-0x357: [data2]  card description, icons, receive date, redistribution limit
```

The first step is to prepend the header section to the PCD data to form an extended PCD ("xPCD").
The resulting block of data looks like this:

```
0x000-0x04f: header
0x050-0x153: data1
0x154-0x1a3: header
0x1a4-0x3a7: data2
```

## Encryption

The xPCD data is encrypted using the stream cipher RC4 (also known as ARC4, ARCFOUR).
The encryption key for this algorithm is made of the distributing system's (or any other transmitting device's) MAC
address and a checksum that is calculated over the xPCD block.

The checksum algorithm is a simple add-and-rotate algorithm:

```c
// the input data is treated as an array of 16-bit words, little-endian
uint16_t checksum(const uint16_t *data, unsigned int length)
{
    uint16_t c = 0;

    while (length--)
    {
        c += *data++;
        c = (c << 1) | (c >> 15);  // rotate c left by 1
    }

return c;
}
```

The encryption key is then generated as follows:
Code:

```c
// c_low means the lower byte of the checksum, c_high the upper one, again 16bit little endian
uint8_t key[] = { mac[0], mac[1], c_low, c_high, mac[4], mac[5], mac[2], mac[3] };
uint16_t *key_16 = (uint16_t*)key;
uint16_t hw = 0x3fa2;

for (int i = 0; i < 4; ++i)
{
    key_16[i] ^= hw;
    hw = key_16[i];
}
```

EDIT: I forgot to mention an important step in the original post:
The 8-byte key array is treated as an array of 4 halfwords and cumulatively XORed with `0x3fa2`.

In the next step the actual RC4 encryption is performed.
Unfortunately I can't post a link to the algorithm here so please look for it yourself.
The resulting block will be called "ePCD".

## Transmission

Wonder cards are transmitted using 802.11 beacon frames which are normally used to advertise a wireless access point (
AP).
After encryption the ePCD block is split into 9 equal-sized fragments of 104 (0x68) bytes with the corresponding index
numbers 0 to 8.
A tenth fragment with index number -1 is made of the unencrypted PCD header padded with zeros to a total size of 104
bytes.
Those 10 distinct fragments are embedded in beacon frames as vendor-specific data.

Below is the format of the vendor-specific element.
All values are in little endian order.

```
Length  Value/Meaning
1  0xdd (tag ID)
1  0x88 (tag length)
3  0x00 0x09 0xbf (OUI, Nintendo)
1  0x00 (OUI subtype)

132  --- actual packet ---
28  --- packet header ---
4  0xa (frames count?)
2  0x1
2  0x1
4  GGID (language code)
2  0x0
2  0x70
2  0x28
2  0xc
2  checksum
2  fragment index
4  0x3a8 (payload length)

104  --- packet payload ---
```

Possible GGID values:

```
0x400318 - English
0x8000cd - French
0x8000cf - Italian
0x8000d0 - Spanish
0x345    - Japanese
0xc00018 - Korean
0x8000ce - German
```

The resulting beacon frames are sent repeatedly by the distribution system ordered by their index in 0.010240 second
interval.