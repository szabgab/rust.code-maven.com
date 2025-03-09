---
title: Creating A Mock Blockchain in Rust with Sourav Mishra
timestamp: 2025-03-09T16:30:01
author: szabgab
published: true
show_related: true
description:
---

A brief introduction about blockchain and it's components. For maybe 15 minutes. Then we move on to building our own blockchain from scratch with just Rust and some cryptography libraries.

{% youtube id="sYt-7jGPve4" file="2025-03-09-creating-a-mock-blockchain-in-rust.mp4" %}


![Sourav Mishra](images/sourav-mishra.png)

* [Sourav Mishra on LinkedIn](https://www.linkedin.com/in/web3-mishra/)

* [The blockchain](https://github.com/0xsouravm/mockchain) created during the session.
* [The Blockchain client](https://github.com/0xsouravm/mockchain-wallet-rs)


## Transcript

1
00:00:02.070 --> 00:00:28.269
Gabor Szabo: So Hi, and welcome to the Code Mavens, Youtube Channel and the Meetup group. If you are here, I'm really happy that somewhere have agreed to give this half presentation or quarter presentation, and mostly a pair programming style introduction to blockchain. I'm really happy that people are here who can listen and ask questions. Remember that this is going to be on Youtube. So if you don't want to be your.

2
00:00:28.270 --> 00:00:37.189
Gabor Szabo: I don't know name or or voice appear on Youtube. Then ask it in the in the chat, and I will notice it and and ask and

3
00:00:37.510 --> 00:00:39.770
Gabor Szabo: talk about it. Try to answer it.

4
00:00:40.570 --> 00:01:09.189
Gabor Szabo: and if you are watching the Youtube video. And if you enjoy this presentation and this meeting, then please also, like the video and follow the channel. So you will be updated. Later. Below the video, you will find a link to the repository that we are working on. I'll try to put everything into a Github Repository and push out to Github. So people who are in the Channel can follow so can pull and follow that code.

5
00:01:09.990 --> 00:01:13.550
Gabor Szabo: That's it. So so Rav. Thank you for agreeing

6
00:01:14.040 --> 00:01:20.739
Gabor Szabo: to to give this presentation, and it's your turn. Now please introduce yourself, and let's go from there.

7
00:01:21.190 --> 00:01:29.149
Sourav Mishra: Yeah, thank you so much. Gabor. So I'm Saurav. And I'm a core protocol engineer for shorter network. I have been that for the past 3 and a half years, maybe.

8
00:01:29.260 --> 00:01:35.259
Sourav Mishra: And my job is basically developing blocks and protocols from scratch using rust and a framework called substrate.

9
00:01:35.710 --> 00:01:37.300
Sourav Mishra: So today will be

10
00:01:37.600 --> 00:01:51.530
Sourav Mishra: learning a bit about blockchain, so I'll be presenting some something I have prepared slides for that, so we'll be learning about blockchains briefly, and then we'll be moving on to creating our own blockchain from scratch using rust, but not any framework. So it just plain vanilla rust and some cryptography grades.

11
00:01:51.900 --> 00:01:53.479
Sourav Mishra: So let's get started.

12
00:01:54.570 --> 00:01:56.020
Sourav Mishra: I'll share my screen now.

13
00:01:58.420 --> 00:02:03.009
Gabor Szabo: Yeah, you may. I mean, you call Co host. So you can. You can share your screen. Yeah.

14
00:02:03.970 --> 00:02:05.060
Sourav Mishra: Can you share my screen?

15
00:02:05.520 --> 00:02:19.980
Gabor Szabo: Yeah. Yeah. And just to know if anyone looks at me and sees that I'm looking upwards. That's because the the larger screen is above my my camera. So when? Where? I need to understand something and actually read the letters, and it's it's there.

16
00:02:20.120 --> 00:02:22.619
Gabor Szabo: So don't worry. I'd like to look at that.

17
00:02:24.690 --> 00:02:27.919
Sourav Mishra: Alright. So I will minimize zoom, and let's begin.

18
00:02:28.390 --> 00:02:29.539
Gabor Szabo: It's good, good.

19
00:02:29.750 --> 00:02:38.149
Sourav Mishra: So this is a presentation I used when I presented for the 1st time my 1st talk in my college. It is still relevant to us. So I'm still using this. So

20
00:02:39.050 --> 00:02:45.709
Sourav Mishra: before starting to build our own blockchain, we should understand what a blockchain is, and to understand that

21
00:02:45.970 --> 00:02:55.100
Sourav Mishra: the simplest of definitions would be. It's a chain of blocks. It's also the most ridiculous definition. But it's the one we're going to be working with, because it's also the easiest to understand.

22
00:02:55.540 --> 00:03:10.710
Sourav Mishra: So as anyone like, if you are a user or a developer, anyone who is remotely concerned about blockchains, you should be primarily focused on those 2 things. That is what exactly are these chains made up of? And what exactly, is the data that is contained within these blocks?

23
00:03:12.160 --> 00:03:21.580
Sourav Mishra: So when I say what data is contained, basically, we care about what type of data is contained, in what format it is contained and to what extent it is contained.

24
00:03:22.770 --> 00:03:29.520
Sourav Mishra: and it typically varies from different blockchain to blockchain, because they have different standards. But essentially

25
00:03:30.190 --> 00:03:33.670
Sourav Mishra: it contains of 2 things. One is the block header.

26
00:03:34.000 --> 00:03:35.959
Sourav Mishra: and other is the transaction data.

27
00:03:36.820 --> 00:03:41.950
Sourav Mishra: So Block Header contains 4 things not always, but usually

28
00:03:42.550 --> 00:03:53.139
Sourav Mishra: one of them is has of the previous block. So has, or a cryptographic has is just a 1 way function where you put some input and it gives you a fixed output, a random output, random string output.

29
00:03:53.670 --> 00:03:59.009
Sourav Mishra: And it is incredibly difficult or almost impossible to go back to the input from the output.

30
00:03:59.270 --> 00:04:05.929
Sourav Mishra: So once you have some input and you got the output already. So you cannot guess what the input was just by looking at the output.

31
00:04:06.040 --> 00:04:10.029
Sourav Mishra: or you cannot mathematically or programmatically go back to the input from the output.

32
00:04:10.820 --> 00:04:13.990
Sourav Mishra: So that's why hash functions are also called travel functions.

33
00:04:14.300 --> 00:04:20.139
Sourav Mishra: So yeah, so the current block has the hash of the previous block. That is how these blocks are chained.

34
00:04:20.459 --> 00:04:31.369
Sourav Mishra: and then it has a timestamp. So timestamp is the timestamp. When the block was added to the blockchain and nonce is something we're gonna look at it later, when we understand consensus, a particular consensus.

35
00:04:31.530 --> 00:04:34.440
Sourav Mishra: And then Merkel loot is something we're gonna look at right now.

36
00:04:35.210 --> 00:04:42.789
Sourav Mishra: So apart from the block header, we have transaction data. So all the transactions that should be included in this block. Go here

37
00:04:43.450 --> 00:04:46.149
Sourav Mishra: and let's go to the Merkel group. Now.

38
00:04:47.360 --> 00:04:52.459
Sourav Mishra: So marker load is basically a step by step, hash of the transactions.

39
00:04:52.560 --> 00:04:58.999
Sourav Mishra: So when you arrange the transactions in the way that I've arranged in the right hand side, it forms something called a mercury tree.

40
00:04:59.120 --> 00:05:02.219
Sourav Mishra: and the root of that maple tree is called a maple root.

41
00:05:02.620 --> 00:05:03.900
Sourav Mishra: So how do you?

42
00:05:05.643 --> 00:05:09.180
Sourav Mishra: You have the transactions? Let's say, transaction one to transaction 4.

43
00:05:09.660 --> 00:05:18.169
Sourav Mishra: Put them side by side, hash all the transactions. So after hashing transaction, one you get hash, one after hashing transition 2 you get hash, 2, so on and so forth.

44
00:05:18.620 --> 00:05:21.000
Sourav Mishra: Then we concatenate, hash one and hash 2,

45
00:05:21.230 --> 00:05:28.719
Sourav Mishra: and you hash it again. So you get hash, 1, 2 do the same thing with hash, 3 and 4. You get hash, 3, 4. Concatenate the hashes again.

46
00:05:28.830 --> 00:05:31.000
Sourav Mishra: then has it again. You get the mokal root.

47
00:05:31.490 --> 00:05:36.540
Gabor Szabo: Okay, I have a question. The the number of transactions is like always 4, or is just some.

48
00:05:36.540 --> 00:05:39.719
Sourav Mishra: No, no, it it can be anything like

49
00:05:40.020 --> 00:05:42.550
Sourav Mishra: it depends on what blockchain you're following. So

50
00:05:42.660 --> 00:05:51.120
Sourav Mishra: I just took 4 transactions because it was easier to demonstrate the merkel root. Using 4 transactions. It can be 5, 6, or 10, whatever it is. I mean.

51
00:05:51.360 --> 00:05:52.120
Gabor Szabo: Okay.

52
00:05:52.120 --> 00:05:57.749
Sourav Mishra: I think ideally, it should be 10 in some blockchains. It is 10. But yeah, you can decide that on your own. There is no.

53
00:05:57.750 --> 00:06:02.810
Sourav Mishra: and and the hashing algorithm that that is used is is a.

54
00:06:03.280 --> 00:06:07.580
Gabor Szabo: What kind of hashing which which hashing algorithm is is one of the well-known.

55
00:06:07.820 --> 00:06:17.480
Sourav Mishra: For our project. We are going to use Ss. 2, 56. Then it varies from blockchain to blockchain and algorithm. Some might use Blake. 3. Some might use something else.

56
00:06:17.680 --> 00:06:21.489
Sourav Mishra: But for our today's project we're gonna use haz 256.

57
00:06:21.890 --> 00:06:22.610
Gabor Szabo: Okay.

58
00:06:23.340 --> 00:06:27.729
Sourav Mishra: No, so any doubts so far regarding how the Murker route is constructed.

59
00:06:28.740 --> 00:06:30.710
Gabor Szabo: Yeah, more or less. Yeah, I understand.

60
00:06:31.440 --> 00:06:32.110
Sourav Mishra: I'm good.

61
00:06:32.500 --> 00:06:35.080
Sourav Mishra: So now let's move on to some blocking concepts.

62
00:06:38.100 --> 00:06:44.110
Sourav Mishra: So a blockchain is decentralized, that is, it is not owned by a single entity, or owned by a centralized entity.

63
00:06:44.220 --> 00:06:59.070
Sourav Mishra: Every participant in the network owns a local copy of that blockchain which to which they contribute, and then they flood that entire contribution to the whole network. And then the whole network adds it to their own local copies. So that's how everyone has the same copy. Essentially

64
00:06:59.410 --> 00:07:00.960
Sourav Mishra: at a given point of time.

65
00:07:00.960 --> 00:07:02.789
Gabor Szabo: Everyone has all the data.

66
00:07:03.040 --> 00:07:11.059
Sourav Mishra: Yeah, everyone needs to have all the data. Because, let's say, if I add a new block to the blockchain, I have to propagate it to the network, so that you guys can also add it to your team.

67
00:07:11.980 --> 00:07:12.910
Gabor Szabo: Okay.

68
00:07:14.480 --> 00:07:25.289
Sourav Mishra: And then it's immutable because of the hashes I just mentioned. So if you change if you change the tiniest bit in the block, then the entire block has changes drastically.

69
00:07:25.470 --> 00:07:37.780
Sourav Mishra: and since the blocks are linked by the hash of the previous blocks, so if the hash of one block changes, then all the hashes of the corresponding blocks that follow. They are all going to be changed, and you will be the only one

70
00:07:38.020 --> 00:07:42.360
Sourav Mishra: with a changed blockchain, so you will be immediately kicked out of yeah.

71
00:07:42.360 --> 00:07:55.600
Gabor Szabo: So one of the things that I'm really unclear to me is that I mean there are these nodes, I guess right that they are the block that on the node someone adds a new block to the chain.

72
00:07:56.149 --> 00:08:02.589
Gabor Szabo: And if what happens if 2 people on 2 different nodes add to the same

73
00:08:02.900 --> 00:08:10.990
Gabor Szabo: block, another one that so do they have now 2 blocks on the same route, same parent, parent, block.

74
00:08:12.630 --> 00:08:26.419
Sourav Mishra: Yeah, that is actually a highly technical concept. So what you just mentioned like, if you add 2 different blocks by 2 different people, and then it is propagated to the network at the same time. Then what you get is called a fork. So there are different algorithms to resolve forks.

75
00:08:26.860 --> 00:08:46.859
Sourav Mishra: and usually what happens is people follow the longest chain. So if the fork that you made has more blocks added to it after you, added the block, then that chain is usually considered valid, and the orphan chain that the other fork that is not considered valid is kind of discarded, and all the transactions are rolled back.

76
00:08:47.420 --> 00:08:58.540
Sourav Mishra: So the transaction that you added in your chain in your book, and you propagated that if it is not the longest chain, then it's kind of like. It's the transaction get rolled back and that entire chain is discarded.

77
00:08:59.210 --> 00:09:02.700
Sourav Mishra: and my chain gets to like replace the valid chain.

78
00:09:03.250 --> 00:09:10.970
Sourav Mishra: So that is one algorithm to solve. For there are other algorithms. I will not go deep into that because it's like really technical. Generally.

79
00:09:11.280 --> 00:09:17.670
Sourav Mishra: if a conflict, then the longest chain kind of gets validated and kept it as the new chain.

80
00:09:18.250 --> 00:09:19.070
Gabor Szabo: Okay.

81
00:09:20.420 --> 00:09:32.459
Sourav Mishra: All right. So I hope immutability is clear, because data once written, it cannot be changed because of the hashes. And if you change one tiny bit in one block. Then the entire blockchain becomes a different blockchain because of the hashes.

82
00:09:32.890 --> 00:09:40.250
Sourav Mishra: so you can be easily caught and kicked out of the network because of malpractice, because only you have the different chain, while everyone else has a different chain.

83
00:09:42.410 --> 00:09:43.110
Gabor Szabo: Right.

84
00:09:43.110 --> 00:09:52.209
Sourav Mishra: And then anonymity, because identities are unknown. So we don't use regular identities like email or passport, or names, or something

85
00:09:52.380 --> 00:10:01.019
Sourav Mishra: to like transact on the blockchain or mine blocks on the blockchain, or add blocks to the blockchain. We use something called a public key, about which I'm going to look at it later.

86
00:10:02.070 --> 00:10:16.550
Sourav Mishra: So yeah, next, it is transparent, because everything that is recorded is available openly to everyone. You can always query a blockchain and see all the transactions like nothing is hidden to anyone, so once it is written on the blockchain, it can be viewed by anyone

87
00:10:16.660 --> 00:10:17.490
Sourav Mishra: out there.

88
00:10:18.060 --> 00:10:19.330
Sourav Mishra: So it's transplanted

89
00:10:20.360 --> 00:10:26.510
Sourav Mishra: and coming back to anonymous again. So like I said, public key is used instead of regular identities like these.

90
00:10:26.780 --> 00:10:35.240
Sourav Mishra: So public keys are just it's a long bunch of heterdicimal numbers. So it's not easily

91
00:10:35.340 --> 00:10:47.799
Sourav Mishra: rememberable by humans. And it's not easily readable by humans also. So you can't look at 2 publications and tell that this belongs to this guy, and this belongs to this guy, unless you have a really good memory, and you can memorize all the public keys and to whom they belong to.

92
00:10:47.940 --> 00:10:52.309
Sourav Mishra: Otherwise it's kind of impossible to figure out who this belongs to.

93
00:10:52.490 --> 00:10:54.009
Sourav Mishra: That is why it's anonymous.

94
00:10:56.540 --> 00:10:59.899
Sourav Mishra: So yeah, now let's look a bit at public key cryptographic.

95
00:11:00.760 --> 00:11:08.539
Sourav Mishra: So it's also synonymous with a symmetric key. Cryptography. There is something called symmetric key cryptography, which is kind of less secure than this.

96
00:11:08.710 --> 00:11:13.959
Sourav Mishra: So in a symmetric key cryptography, you have 2 separate keys, one public key and one private key.

97
00:11:14.330 --> 00:11:15.819
Sourav Mishra: So you use the

98
00:11:16.691 --> 00:11:26.370
Sourav Mishra: private key. The private key is basically a secret key like, think of it as your email and password. So email, you say openly with everyone but your password. You keep it secret to yourself only

99
00:11:27.080 --> 00:11:37.179
Sourav Mishra: so. A private key is kept with you secretly, and a public key is propagated out of the world for, like, for sending money to you, or you send money to someone else, using a public key, etc. Etc.

100
00:11:38.500 --> 00:11:43.290
Sourav Mishra: And because it is asymmetric. So you encrypt with one key and decrypt with other.

101
00:11:43.480 --> 00:11:51.260
Sourav Mishra: So usually, what happens during signatures is you sign a message using a private key, and the other person can decrypt it, using your public key.

102
00:11:51.530 --> 00:11:54.950
Gabor Szabo: Is this the same like what what you use usually with sh! Right.

103
00:11:55.640 --> 00:11:56.690
Sourav Mishra: In what sorry.

104
00:11:56.690 --> 00:12:06.010
Gabor Szabo: In Ssh, when you're Ssh into somewhere, and then you're using private key and public key authentication unless you really want to type in your password every time.

105
00:12:06.170 --> 00:12:09.149
Sourav Mishra: Yeah, that's that's that's pretty much the same thing.

106
00:12:09.150 --> 00:12:09.840
Gabor Szabo: Okay.

107
00:12:09.840 --> 00:12:17.020
Sourav Mishra: 2 keys, you use one key to encrypt, and the other key to decrypt. Now it depends on you which key use, because if you encrypt with your

108
00:12:17.310 --> 00:12:34.809
Sourav Mishra: public key, then it has to be decrypted the private key, because it makes no sense, because nobody can decrypt that, because your private, supposedly secret to you, but if you do, encrypted with your private key, which is secret to you and you only other people can decrypt it with your public, and they can, you know.

109
00:12:35.290 --> 00:12:36.730
Sourav Mishra: very fair something like that.

110
00:12:40.410 --> 00:12:44.590
Sourav Mishra: and it's more secure than symmetric cryptography. Because, I said

111
00:12:45.000 --> 00:12:48.989
Sourav Mishra: in symmetric key cryptography, we use only one key for both encryption and decryption.

112
00:12:49.210 --> 00:12:54.029
Sourav Mishra: and it is really difficult to let the other party know what exactly your key is.

113
00:12:54.390 --> 00:13:02.860
Sourav Mishra: and it can easily be intercepted by attackers in the middle while you're setting keys. So if the key is lost, then your entire channel is like it's compromised.

114
00:13:04.750 --> 00:13:14.299
Sourav Mishra: and then Rsa and Ecc are the 2 most common algorithms for asymmetrically cryptography. Rsa was Rsa is not being used anymore, I guess, because Ecc is

115
00:13:14.650 --> 00:13:15.950
Sourav Mishra: much better than that.

116
00:13:16.070 --> 00:13:31.140
Sourav Mishra: So to just give an example. Acc keys provide the same amount of security with almost half the size of Rsa keys. So Ecc. Keys are like much more compact and easier to handle, plus the algorithm is modern. That's why Rsa is not being used

117
00:13:31.963 --> 00:13:34.789
Sourav Mishra: nowadays much. But Ecc is really popular.

118
00:13:36.790 --> 00:13:41.559
Sourav Mishra: So rac uses large prime numbers and their factorization to compute the keys.

119
00:13:41.720 --> 00:13:47.360
Sourav Mishra: So basically, you take 2 large prime numbers and you multiply them together.

120
00:13:47.560 --> 00:13:59.339
Sourav Mishra: And it is understood that, because if the product is also very large, because the plan numbers are very large, so it's not really easy to factorize the final product. So that is the base of Rc. Cryptographic.

121
00:14:00.110 --> 00:14:10.389
Sourav Mishra: and for Ecc you use elliptic curve. So you select a really large random number that's within the bound of the relative curve, and we multiply a generator point to that that gives you.

122
00:14:11.360 --> 00:14:12.750
Sourav Mishra: Is there a chat question.

123
00:14:13.650 --> 00:14:18.770
Gabor Szabo: No, no! I turn this off. I have this beep whenever someone connects.

124
00:14:19.480 --> 00:14:22.039
Sourav Mishra: Yeah, I thought, like somebody had a question or something.

125
00:14:22.280 --> 00:14:26.300
Gabor Szabo: No, no, I just I have this on. I I turn this off sorry.

126
00:14:29.490 --> 00:14:41.860
Sourav Mishra: So like, we're saying, Yeah, Ecc uses elliptic curves as their base. And you multiply a really large number with the generator point to get a public key, and that really large number is usually a private key.

127
00:14:42.730 --> 00:14:48.649
Sourav Mishra: So we don't go into details of how these curves work. But yeah, rac uses prime numbers and ecc uses electric curves.

128
00:14:49.590 --> 00:14:53.470
Sourav Mishra: We will be a particular electric curve today. It's called Sccp

129
00:14:53.760 --> 00:14:56.659
Sourav Mishra: to like in our project while they're building it.

130
00:14:57.430 --> 00:15:00.239
Sourav Mishra: It is the same curve that is used by the Bitcoin blockchain.

131
00:15:02.350 --> 00:15:03.140
Sourav Mishra: Alright.

132
00:15:03.699 --> 00:15:07.469
Sourav Mishra: Moving on. Let's look at the final part of the presentation. It's a

133
00:15:07.700 --> 00:15:13.340
Sourav Mishra: some consensus mechanisms. So basically consensus is how the

134
00:15:13.920 --> 00:15:18.050
Sourav Mishra: blockchain arrives at a unified decision, that yes, this is right, and this is wrong.

135
00:15:18.280 --> 00:15:21.179
Sourav Mishra: So the 1st consensus is proof of work

136
00:15:21.350 --> 00:15:28.819
Sourav Mishra: in which minus are the people who add blocks to the blockchain. So they solve complicated puzzles to validate transactions and create new blocks.

137
00:15:29.220 --> 00:15:33.090
Sourav Mishra: This requires a lot of computational power because of the nature of the puzzle.

138
00:15:33.510 --> 00:15:39.999
Sourav Mishra: So if you remember, we saw a keyword called nonce in our blog header when I was showing you the block diagram.

139
00:15:40.560 --> 00:15:45.419
Sourav Mishra: So basically minus, I trade over all possible.

140
00:15:46.130 --> 00:16:11.869
Sourav Mishra: And then they try to hash the block. And if the hash of the block matches a particular difficulty that is set by the blockchain, then that block is considered valid. So let's say, if I start with nonch equal to one. Then I hash all the block data. Then I check the hash. If it starts with, let's say, 3 zeros, then it should be considered valid, but mine starts with 2 zeros, then that nonce is not valid, so I try with nonce to. I do the same. I repeat the same process. Hash it, check it.

141
00:16:12.190 --> 00:16:16.929
Sourav Mishra: It doesn't. Then I increment the nonce nonce equal to 3. Then I do the same thing.

142
00:16:17.160 --> 00:16:28.310
Sourav Mishra: So it goes on and on and on, until I actually find a particular nonce that satisfies the difficulty of the blockchain. Difficulty in this case is how many leading Zeros the Hask should start with.

143
00:16:28.710 --> 00:16:40.980
Sourav Mishra: So if the number of leading zeros is 3, then the hash that the final hash that you get after finding the nonce, it should all start with 3 zeros. It doesn't matter what follows out of 3 zeros, but the hash should at least start with 3 zeros.

144
00:16:42.180 --> 00:16:52.539
Sourav Mishra: So the miners try with different values of nonce and under analyst they get to that. And the 1st person who solves this is the person who gets to mind the block and get the block reward.

145
00:16:53.720 --> 00:16:55.560
Sourav Mishra: So that is how proof of work works.

146
00:16:57.080 --> 00:16:57.910
Gabor Szabo: Okay.

147
00:16:59.940 --> 00:17:01.030
Sourav Mishra: Any doubts on this.

148
00:17:04.630 --> 00:17:15.509
Gabor Szabo: Yeah, it's it's it's a i read about it. And and I still don't really understand this whole whole mining thing. But I wonder if if that's really important for or

149
00:17:16.089 --> 00:17:18.040
Gabor Szabo: actual implementation.

150
00:17:18.349 --> 00:17:21.509
Sourav Mishra: It is. Actually, we're going to implement a proof of all blockchain today.

151
00:17:22.619 --> 00:17:23.839
Gabor Szabo: Okay. Good.

152
00:17:24.460 --> 00:17:26.690
Sourav Mishra: So. Which part did you want? Answer? I could explain it to you again.

153
00:17:27.859 --> 00:17:35.679
Gabor Szabo: I think we can just go on, and then if if I get to that point again, then then maybe that will clarify me.

154
00:17:36.169 --> 00:17:38.649
Gabor Szabo: And then we can get back to this. Okay.

155
00:17:39.010 --> 00:17:48.800
Sourav Mishra: But just to explain it once more for people who had trouble understanding it the 1st time, Rubov is basically a consensus in which miners need to solve for a particular value of nonce

156
00:17:48.910 --> 00:17:58.460
Sourav Mishra: that should satisfy the difficulty of the blockchain and difficulty of the blockchain, for example, could be something like the final hash of the block, could start with N number of zeros.

157
00:17:58.740 --> 00:18:12.309
Sourav Mishra: So let's say, if N equals to 2, and you choose a nonce value, and the final hash of the block starts with 2 zeros. Then that is considered a valid nonce and a valid block. And you get to mine. That block mining basically means adding the block to the blockchain.

158
00:18:12.700 --> 00:18:13.040
Gabor Szabo: Yeah.

159
00:18:13.380 --> 00:18:23.329
Sourav Mishra: If you're a miner, and you found the nonce first, st and your nonce satisfies the difficulty of the blockchain, then you get to mind the block 1st and get the block rewards.

160
00:18:23.850 --> 00:18:24.710
Gabor Szabo: Okay.

161
00:18:25.170 --> 00:18:29.469
Sourav Mishra: So basically, miners compete with each other to find the block or find the nonce first.st

162
00:18:30.810 --> 00:18:33.560
Sourav Mishra: Whoever finds it. 1st they get to mind the block and get the rewards.

163
00:18:35.630 --> 00:18:38.039
Gabor Szabo: Pricing. No, no, I get it. Yeah.

164
00:18:38.620 --> 00:18:42.379
Sourav Mishra: Yeah. And the next consensus is proof of stake.

165
00:18:42.600 --> 00:18:59.909
Sourav Mishra: It is a much lightweight consensus because it doesn't, doesn't need miners or validators to solve complex puzzles. Validators are basically chosen based on the amount they have stake and how long they have been a network participant and how long they have staked the amount. Usually it's about how much amount they have staked.

166
00:19:00.580 --> 00:19:09.639
Sourav Mishra: So people argue that this is a bit centralized because of the rich get richer mentality, because whoever has the most stake easily is chosen to be the validator for the next blog.

167
00:19:10.220 --> 00:19:13.200
Sourav Mishra: So that's there, and

168
00:19:13.380 --> 00:19:27.060
Sourav Mishra: you don't have to compete with anyone because you are chosen based on your stake. So if you're chosen as a validator, then you get to add the block to the blockchain. You don't have to like, compete with the other validators to find some particular value or something, and then add the block to the blockchain.

169
00:19:27.440 --> 00:19:30.610
Sourav Mishra: So that's how it is. Pretty energy efficient because

170
00:19:30.890 --> 00:19:33.069
Sourav Mishra: you just get chosen and you add the block.

171
00:19:33.660 --> 00:19:34.270
Gabor Szabo: That's up!

172
00:19:36.550 --> 00:19:41.189
Sourav Mishra: And the last example. The 3rd type of consensus is proof of authority.

173
00:19:41.590 --> 00:19:42.390
Sourav Mishra: So

174
00:19:42.760 --> 00:20:03.699
Sourav Mishra: this is by far the most centralized consensus, because only people with known authority are allowed to add blocks to the network. Like, say, if you're an organization, then the Board of Directors or the CEO, or the upper management, only they get to add the blocks because they have the authority to add the blocks. And the rest of the people are just network participants. They don't get to like, participate in the governance of the chain.

175
00:20:03.960 --> 00:20:10.450
Sourav Mishra: That's why it is more centralized. But then it really flourishes in those cultures like corporate cultures, or somewhere where you have a hierarchy.

176
00:20:13.240 --> 00:20:28.330
Sourav Mishra: so you can think of proof of authority to be similar to proof of stake. But instead of validators or minus or block address being chosen by the number of the amount of the stake they are just predefined by the authority they hold in the

177
00:20:28.560 --> 00:20:29.640
Sourav Mishra: automation.

178
00:20:30.580 --> 00:20:40.159
Gabor Szabo: There is a question here. The the idea is close to the question you asked before, what if 2 different nodes mine 2 blocks with the same parent.

179
00:20:40.460 --> 00:20:47.059
Gabor Szabo: We say the node with the longer chain wins. But why wouldn't Hacker just create extremely long chain?

180
00:20:47.690 --> 00:21:01.100
Gabor Szabo: The proof of work proves that you used work money into the creating your chain. It costs you money, so as long as a majority of players act honestly, hackers will have troubles to forge

181
00:21:01.310 --> 00:21:06.250
Gabor Szabo: blocks that are different than what the honest players are creating.

182
00:21:06.880 --> 00:21:11.729
Gabor Szabo: Okay? It was what? That was the question to clarification. Okay, thank you. Okay.

183
00:21:15.990 --> 00:21:18.300
Sourav Mishra: Yeah, so we can move forward.

184
00:21:19.530 --> 00:21:21.229
Gabor Szabo: Yeah, yeah, it's looking good.

185
00:21:21.230 --> 00:21:23.490
Sourav Mishra: Alright. So this kind of ends my presentation here.

186
00:21:24.100 --> 00:21:29.110
Sourav Mishra: So this is all I had to talk about. So this is all you should know before writing on blockchain.

187
00:21:29.880 --> 00:21:30.680
Gabor Szabo: Okay.

188
00:21:31.190 --> 00:21:34.709
Sourav Mishra: So if there are no questions, then we can directly start to the code.

189
00:21:35.510 --> 00:21:39.659
Gabor Szabo: Okay. So so now, should I share my screen now? Right.

190
00:21:39.790 --> 00:21:44.289
Sourav Mishra: Yes, can you please clone the repository that I just said the 1st one.

191
00:21:46.120 --> 00:21:48.450
Gabor Szabo: Should I clone that repository? Okay, yeah.

192
00:21:48.960 --> 00:21:52.179
Gabor Szabo: okay, let's just share the screen.

193
00:21:55.390 --> 00:21:58.900
Gabor Szabo: Okay? So you were supposed to see the

194
00:21:59.925 --> 00:22:03.680
Gabor Szabo: this one, the this one we want. I wanted right?

195
00:22:04.000 --> 00:22:04.570
Sourav Mishra: Yeah.

196
00:22:05.340 --> 00:22:07.250
Gabor Szabo: Okay.

197
00:22:18.070 --> 00:22:19.110
Gabor Szabo: okay.

198
00:22:21.990 --> 00:22:26.800
Sourav Mishra: And you can check out to the workshop branch. I think it's called board Maven Workshop.

199
00:22:28.530 --> 00:22:29.330
Gabor Szabo: Sorry.

200
00:22:30.730 --> 00:22:31.610
Gabor Szabo: Should I check?

201
00:22:32.200 --> 00:22:32.940
Sourav Mishra: Yeah.

202
00:22:33.945 --> 00:22:34.770
Gabor Szabo: There's a bridge!

203
00:22:35.080 --> 00:22:38.559
Sourav Mishra: Yeah. No code hyphen, mavin, hyphen workshop.

204
00:22:43.780 --> 00:22:46.240
Gabor Szabo: Even workshop.

205
00:22:47.286 --> 00:22:49.069
Sourav Mishra: Code hyphen maven hyphen workshop.

206
00:22:52.720 --> 00:22:54.770
Gabor Szabo: Wait a second. I'll I'll look at here here.

207
00:22:56.350 --> 00:23:07.370
Gabor Szabo: Code, maybe a workshop. Okay. Code me? And virtual, okay, hopefully, we are there.

208
00:23:10.810 --> 00:23:12.119
Sourav Mishra: Right. This is the one.

209
00:23:12.830 --> 00:23:19.129
Gabor Szabo: Just a second. It it opens the always opens into my other screen. Drag it here.

210
00:23:19.900 --> 00:23:24.169
Gabor Szabo: Okay, so we have the editor open.

211
00:23:30.130 --> 00:23:36.050
Sourav Mishra: But the reason why I asked you to clone is because it has some starter code that is otherwise. It cumbersome to write yourself.

212
00:23:37.120 --> 00:23:41.819
Gabor Szabo: Okay. So this is the, this is only the skeleton that we have. It is not the

213
00:23:42.140 --> 00:23:43.189
Gabor Szabo: the implementation. Yeah.

214
00:23:43.190 --> 00:23:44.559
Sourav Mishra: It's not the final product. Yeah.

215
00:23:45.260 --> 00:23:47.409
Sourav Mishra: we are going to build on top of this.

216
00:23:51.900 --> 00:23:52.610
Gabor Szabo: 3.

217
00:23:55.030 --> 00:23:56.479
Sourav Mishra: Yeah, let's let's begin then.

218
00:23:57.280 --> 00:24:05.870
Sourav Mishra: So I'll give you a quick walkthrough of what are the files we have here? So we'll be writing code to the main file. Only we're not going to create different files for the simplicity of the project.

219
00:24:06.690 --> 00:24:12.320
Sourav Mishra: and you might want to turn off your analyzer because it's going to show a lot of red lines because the code is not complete.

220
00:24:14.810 --> 00:24:28.350
Sourav Mishra: And so the files we have defined here is the main Rs file, which is our main file, and that is the main server file. Then there's a build dot Rs file that basically compiles our protocol. And if you go back to the proto folder.

221
00:24:30.070 --> 00:24:31.489
Gabor Szabo: If I go back to where? Sorry?

222
00:24:31.490 --> 00:24:33.300
Sourav Mishra: The proto folder, the 1st folder.

223
00:24:37.340 --> 00:24:39.010
Gabor Szabo: The Protocol.

224
00:24:39.440 --> 00:24:41.239
Sourav Mishra: Yeah. The 1st folder right above searching.

225
00:24:41.660 --> 00:24:44.330
Sourav Mishra: Yeah. And you open the blockchain at 2 to 5.

226
00:24:47.110 --> 00:24:48.250
Gabor Szabo: Sorry I didn't understand.

227
00:24:49.010 --> 00:24:59.640
Sourav Mishra: Yeah, so this is our protocol definition. So we have defined a service. And then there are some messages. I will come back to this file later, when we explain the main function.

228
00:25:00.210 --> 00:25:22.990
Sourav Mishra: But basically, we have integrated Grpc connections to our blockchain so that our wallet can connect to our blockchain, and send requests and requests like standard transaction, or get balance or get coins from the faucet. So these 3 things that you see on line 7, 10 and 13 from the Rpc lines. So basically, those are the methods that are exposed to our client.

229
00:25:23.590 --> 00:25:32.269
Sourav Mishra: So our client can submit a transaction to our blockchain. It can get balance from our blockchain, and it can get the faucet money from our blockchain. Basically.

230
00:25:33.390 --> 00:25:34.330
Gabor Szabo: Okay.

231
00:25:34.610 --> 00:25:38.529
Sourav Mishra: Yeah, and I'm using Grpc, hence the protofile, the Protobuff file.

232
00:25:40.460 --> 00:25:41.380
Gabor Szabo: Okay.

233
00:25:41.610 --> 00:25:43.029
Sourav Mishra: Let's go back to the main again.

234
00:25:45.750 --> 00:25:58.310
Gabor Szabo: Wait a second. There was some comment there, Pavel, I think that in bitcoins blockchain is the longest chain with the maximum aggregated pow difficulty, not simply the longest chain by the number of blocks.

235
00:25:58.600 --> 00:26:01.540
Gabor Szabo: Okay, that's just another comment. Thank you.

236
00:26:03.740 --> 00:26:07.170
Sourav Mishra: Okay, so let's move on to the main function. Main file. Yeah.

237
00:26:08.090 --> 00:26:08.680
Gabor Szabo: Yeah.

238
00:26:09.350 --> 00:26:11.460
Sourav Mishra: So we have a bunch of imports here.

239
00:26:11.780 --> 00:26:25.019
Sourav Mishra: Chrono is for managing date and time. Sccp is the curve that are using for sequences and key management, public and private key survey is for serializing and deserializing our transaction data mainly so that we can sign it.

240
00:26:25.200 --> 00:26:27.560
Sourav Mishra: And sh. A tool is basically, the

241
00:26:27.730 --> 00:26:34.920
Sourav Mishra: has an algorithm, the 2 56. And then Vet, Dq is something you're using for a transaction pool maintenance

242
00:26:35.160 --> 00:26:39.880
Sourav Mishra: and arc and mutex. We are using that for starting our blockchain server

243
00:26:40.400 --> 00:26:43.240
Sourav Mishra: and system. Time is because of timestamps again.

244
00:26:43.580 --> 00:26:47.580
Sourav Mishra: and tonic is something we are using to handle grpc.

245
00:26:47.720 --> 00:26:51.510
Sourav Mishra: otherwise it would be almost impossible for us to develop this without tonic.

246
00:26:51.980 --> 00:26:55.380
Sourav Mishra: and we have to write a lot of code by ourselves which is not advisable.

247
00:26:55.840 --> 00:27:00.419
Sourav Mishra: And then log is used for this logging info and one going to the console.

248
00:27:01.400 --> 00:27:07.349
Sourav Mishra: So this pub MoD blockchain is something that comes from the photograph file.

249
00:27:07.770 --> 00:27:11.249
Sourav Mishra: So tonic include proto gives us a module called blockchain.

250
00:27:11.850 --> 00:27:19.689
Sourav Mishra: and the Const faucet modechain address is just a random address. I mean, it's not exactly an address. It's just a string. But since we

251
00:27:19.890 --> 00:27:32.789
Sourav Mishra: representing addresses as strings, so I just put it here like this as a constant, because if it is a faucet transaction, then you don't need to actually verify the sender and receiver and everything else. You just quickly pass it on to the block

252
00:27:33.150 --> 00:27:34.530
Sourav Mishra: and add it to the blockchain.

253
00:27:34.980 --> 00:27:38.729
Sourav Mishra: So that's why I just created a separate, unique address for the faucet.

254
00:27:39.050 --> 00:27:43.250
Sourav Mishra: You can choose any address you wish, but I just do the social link to make it simple.

255
00:27:44.410 --> 00:27:49.180
Sourav Mishra: and then line number 18 onwards to line 26. We have

256
00:27:50.848 --> 00:27:57.390
Sourav Mishra: imports from the blockchain module that we just defined online 12. That is the Protobuf module.

257
00:27:57.640 --> 00:27:58.709
Sourav Mishra: so I'll commit.

258
00:27:58.710 --> 00:28:00.210
Gabor Szabo: 36.

259
00:28:01.360 --> 00:28:01.990
Sourav Mishra: Sorry.

260
00:28:02.370 --> 00:28:03.940
Gabor Szabo: You said line 36.

261
00:28:04.180 --> 00:28:05.979
Sourav Mishra: No, no line, 18 to 26.

262
00:28:06.250 --> 00:28:06.990
Gabor Szabo: Okay.

263
00:28:08.170 --> 00:28:11.230
Sourav Mishra: So these are all the blocks and model imports that we just defined above.

264
00:28:11.230 --> 00:28:12.480
Gabor Szabo: Okay. He's worse.

265
00:28:13.980 --> 00:28:14.660
Sourav Mishra: So

266
00:28:14.830 --> 00:28:24.459
Sourav Mishra: are not. Defined in rust code. This is just defined in the prototype file. But tonic helps us create these types for us so you can use them.

267
00:28:25.710 --> 00:28:26.550
Gabor Szabo: Okay.

268
00:28:28.140 --> 00:28:30.859
Sourav Mishra: Alright. Then can you go down to line 28.

269
00:28:36.060 --> 00:28:37.469
Gabor Szabo: 2838.

270
00:28:37.670 --> 00:28:41.580
Sourav Mishra: 2828, yeah, just a bit above. So you can see the other quote below. Also.

271
00:28:42.170 --> 00:28:43.050
Gabor Szabo: Okay.

272
00:28:43.070 --> 00:28:54.639
Sourav Mishra: Alright. So we have a consensus trade. So basically because we are creating a blockchain with a pluggable consensus mechanism. So we have a consensus trade. That kind of like.

273
00:28:55.170 --> 00:29:02.340
Sourav Mishra: You need to implement this straight for every consensus algorithm that you write so that we can use dynamic dispatch to

274
00:29:02.640 --> 00:29:16.649
Sourav Mishra: dynamically update the consensus whenever going like. If I want to select proof of work, I can choose proof of work. If I want to select proof of stake, I can select proof of stake. So the blockchain code doesn't change. Only my consensus implementation changes based on the trait.

275
00:29:18.400 --> 00:29:24.390
Sourav Mishra: And then we have the main function that I'll come back later after I defined everything else. So that's for the last.

276
00:29:25.160 --> 00:29:25.920
Gabor Szabo: Okay.

277
00:29:27.060 --> 00:29:30.080
Sourav Mishra: Now let's start writing codes.

278
00:29:30.610 --> 00:29:32.600
Sourav Mishra: One thing that we have been waiting for so long.

279
00:29:33.490 --> 00:29:36.870
Gabor Szabo: Yeah, every second. It's complaining about something.

280
00:29:37.520 --> 00:29:41.110
Sourav Mishra: Yeah, I mean, it will complain because a lot of things are missing.

281
00:29:41.820 --> 00:29:42.560
Gabor Szabo: Okay.

282
00:29:43.320 --> 00:29:48.729
Sourav Mishra: So I suggest for you to turn off your analyzer. Please disable it for the timing.

283
00:29:49.270 --> 00:29:50.959
Gabor Szabo: Yeah. Well, okay.

284
00:29:51.130 --> 00:29:53.180
Sourav Mishra: I mean, unless you're not bothered by the red lines. Then.

285
00:29:53.180 --> 00:29:55.630
Gabor Szabo: Yeah, it's it's okay. It's okay for me. No, no problem.

286
00:29:56.290 --> 00:29:56.850
Sourav Mishra: Alright.

287
00:29:58.340 --> 00:30:02.430
Sourav Mishra: So now let's define an enum called consensus type.

288
00:30:05.910 --> 00:30:06.700
Gabor Szabo: Skype.

289
00:30:07.430 --> 00:30:10.170
Sourav Mishra: On line 35. Let's define it.

290
00:30:11.210 --> 00:30:11.840
Gabor Szabo: Yeah.

291
00:30:12.150 --> 00:30:15.050
Sourav Mishra: You can slap a derived debug, and then define the enum.

292
00:30:16.610 --> 00:30:26.589
Gabor Szabo: I think the best if you type in. Okay. So I wanted you to do myself to type. But I don't. I'm not sure I understand the how do you spell this? What! What you want to? Continues.

293
00:30:27.115 --> 00:30:30.810
Sourav Mishra: Consensus. That's the one that you consensus type.

294
00:30:31.390 --> 00:30:35.830
Gabor Szabo: Oh, consensus type. Okay.

295
00:30:39.210 --> 00:30:40.519
Gabor Szabo: this is what you entered.

296
00:30:41.450 --> 00:30:42.880
Sourav Mishra: It's still loading for me.

297
00:30:43.571 --> 00:30:47.959
Gabor Szabo: It takes time. Oh, I didn't know that it takes time for you to to see what I'm typing.

298
00:30:50.515 --> 00:30:53.530
Sourav Mishra: Yes, this is what I wanted you to type.

299
00:30:54.780 --> 00:30:55.520
Gabor Szabo: Okay.

300
00:30:55.820 --> 00:31:01.609
Gabor Szabo: it's just copilot telling me. I mean, I guess it's reading your code. And then it's offering me the same thing.

301
00:31:01.610 --> 00:31:04.639
Sourav Mishra: That is also correct. Actually, I wanted you to type the same thing.

302
00:31:05.310 --> 00:31:06.350
Gabor Szabo: Okay.

303
00:31:06.350 --> 00:31:09.940
Sourav Mishra: Yeah, just add a derived debug on the above the email.

304
00:31:11.840 --> 00:31:12.600
Gabor Szabo: Sorry.

305
00:31:13.224 --> 00:31:17.239
Sourav Mishra: Derive debug a derive macro with debug rate implementation for the enum.

306
00:31:21.850 --> 00:31:23.400
Sourav Mishra: Let me write it in chat.

307
00:31:24.600 --> 00:31:28.900
Gabor Szabo: Derive. What drive? What should I drive if you clone or.

308
00:31:28.900 --> 00:31:30.079
Sourav Mishra: A debug debug.

309
00:31:30.340 --> 00:31:31.090
Gabor Szabo: Debug.

310
00:31:31.450 --> 00:31:32.120
Sourav Mishra: Yeah.

311
00:31:33.150 --> 00:31:34.000
Gabor Szabo: Okay.

312
00:31:42.190 --> 00:31:44.759
Gabor Szabo: Well, maybe. Oh, okay.

313
00:31:48.400 --> 00:31:49.830
Sourav Mishra: Yeah, d was not alone.

314
00:31:50.150 --> 00:31:51.701
Gabor Szabo: So, okay, so so

315
00:31:53.090 --> 00:31:58.390
Gabor Szabo: now that I I see that there is an issue that it takes time for you to load the

316
00:31:58.860 --> 00:32:12.719
Gabor Szabo: to see what I'm typing. Maybe it's really better that we switch back to the original, that you're going to type, and it's going to be faster. But it will have the same delay for all of us. What do you say?

317
00:32:14.330 --> 00:32:16.739
Sourav Mishra: I mean, I'm okay with typing, no worries. I can type in.

318
00:32:17.120 --> 00:32:20.919
Gabor Szabo: Okay. So maybe it's better. So let's switch back to you.

319
00:32:21.220 --> 00:32:26.287
Gabor Szabo: I'll leave it as it is now for myself. And then

320
00:32:28.477 --> 00:32:33.200
Gabor Szabo: stop the sharing, so you can share, and then and it might be easier.

321
00:32:35.840 --> 00:32:36.580
Sourav Mishra: Okay.

322
00:32:43.160 --> 00:32:46.679
Sourav Mishra: One sec. Let me just check out to the workshop branch.

323
00:33:22.950 --> 00:33:23.800
Gabor Szabo: Okay.

324
00:33:27.630 --> 00:33:28.830
Sourav Mishra: Everyone, see my screen.

325
00:33:30.933 --> 00:33:32.579
Gabor Szabo: Yeah, yeah.

326
00:33:33.050 --> 00:33:34.210
Sourav Mishra: I'll just turn off my video.

327
00:33:34.523 --> 00:33:37.340
Gabor Szabo: Maybe you can enlarge the font a little bit.

328
00:33:38.917 --> 00:33:40.170
Sourav Mishra: Zooming I mean.

329
00:33:40.730 --> 00:33:41.430
Gabor Szabo: Sorry.

330
00:33:41.760 --> 00:33:42.939
Sourav Mishra: Do you want me to zoom in.

331
00:33:43.710 --> 00:33:46.480
Gabor Szabo: Yeah. Zoom in. Yeah, please. Okay, that's fine. Fine.

332
00:33:46.480 --> 00:33:47.290
Sourav Mishra: Right? Okay.

333
00:33:47.930 --> 00:33:51.209
Gabor Szabo: Thank you. And then there was. There was a question in Oops.

334
00:33:51.960 --> 00:33:54.070
Gabor Szabo: I almost closed the window.

335
00:33:54.070 --> 00:33:55.930
Sourav Mishra: Is there a chat question?

336
00:33:56.950 --> 00:33:57.290
Gabor Szabo: So.

337
00:33:57.290 --> 00:34:02.659
Sourav Mishra: Sync. Okay, so send and sync are basically used to make something thread safe

338
00:34:02.940 --> 00:34:05.550
Sourav Mishra: because we are going to use multiple threads.

339
00:34:05.760 --> 00:34:10.420
Sourav Mishra: So we need our consensus rate to be thread safe so that it can be passed between threads

340
00:34:10.679 --> 00:34:15.429
Sourav Mishra: and sentencing don't need to be defined anywhere, because they are from the Russian Standard Library.

341
00:34:19.500 --> 00:34:21.649
Sourav Mishra: Alright. So let's begin.

342
00:34:34.860 --> 00:34:37.300
Sourav Mishra: Are you guys able to see it in real time? Or is it.

343
00:34:37.300 --> 00:34:40.770
Gabor Szabo: Yeah, you already derived. That's what I can see.

344
00:34:41.620 --> 00:34:47.920
Sourav Mishra: Okay, that's not the same tax manager.

345
00:34:47.929 --> 00:34:57.679
Gabor Szabo: I mean, if you say as you write, then we'll see when we'll be able to understand if you if you're if it happens at the same time, or if we can get some delay.

346
00:34:58.310 --> 00:35:02.199
Sourav Mishra: Alright. So I'll now define the enum per enum consensus type.

347
00:35:02.520 --> 00:35:09.340
Sourav Mishra: And just for clarity, we're going to define a different type as well. A different consensus type. So let's call it proof of state type.

348
00:35:09.790 --> 00:35:21.349
Sourav Mishra: But we are not going to implement this today. We just have to define it so that it will be clear for you why, we have different types here, and for proof of stake type, we are not going to take difficulty. Rather, we are going to take a means take.

349
00:35:21.560 --> 00:35:23.330
Sourav Mishra: and that would be a U 64.

350
00:35:26.050 --> 00:35:33.029
Sourav Mishra: So we have, anonym consensus type. And we have defined 2 consensus types of proof of work and proof of stake.

351
00:35:34.480 --> 00:35:38.689
Sourav Mishra: Now, next, we are going to define the other data types that are going to use.

352
00:35:38.800 --> 00:35:46.179
Sourav Mishra: So 1st comes a stripe for transaction. So this basically defines what our transaction is gonna look like.

353
00:35:47.110 --> 00:35:51.860
Sourav Mishra: We will essentially have a center address, a recipient address, and an amount

354
00:35:52.600 --> 00:35:57.590
Sourav Mishra: and timestamp and signature. So sender, receiver amount should be clear timestamp as well.

355
00:35:57.730 --> 00:36:01.929
Sourav Mishra: and signature is basically the signature of the sender who is sending the money to the receiver.

356
00:36:02.230 --> 00:36:04.088
Sourav Mishra: So we're gonna call it

357
00:36:05.080 --> 00:36:07.539
Sourav Mishra: pub, from which is a string.

358
00:36:08.480 --> 00:36:12.739
Sourav Mishra: And it's basically a string representation of a public key.

359
00:36:13.370 --> 00:36:17.660
Sourav Mishra: Then they're going to call like pub 2, which is the receiver.

360
00:36:17.960 --> 00:36:21.649
Sourav Mishra: Again, it's going to be an address in the string representation.

361
00:36:22.330 --> 00:36:27.270
Sourav Mishra: Then there is pub amount, which is the amount of money I'm sending it is gonna be 64.

362
00:36:28.350 --> 00:36:30.609
Sourav Mishra: Finally, timestamp is also 54.

363
00:36:30.850 --> 00:36:33.060
Sourav Mishra: Pool and signatory is the configured.

364
00:36:35.260 --> 00:36:44.260
Sourav Mishra: So this is our transaction type or transaction truck. So any transaction that we are going to register on the blockchain is going to be of this type containing these information.

365
00:36:44.510 --> 00:36:45.520
Sourav Mishra: this information.

366
00:36:45.790 --> 00:36:49.990
Sourav Mishra: So I'll slap a derive again, because I need it to be of specific types.

367
00:36:50.200 --> 00:36:52.309
Sourav Mishra: We look to implement specific traits.

368
00:36:52.570 --> 00:36:54.240
Sourav Mishra: So if we work.

369
00:36:55.250 --> 00:37:03.980
Gabor Szabo: So I still would like to get understand? A little bit more here. So there is a blockchain out there

370
00:37:04.700 --> 00:37:10.819
Gabor Szabo: functioning right? And now, what you're writing is some implementation of a node for it.

371
00:37:12.812 --> 00:37:15.619
Sourav Mishra: No, I am writing implementation of the blockchain itself.

372
00:37:16.100 --> 00:37:30.380
Gabor Szabo: The blockchain itself. So you assume that there's no blockchain. You just want to create a new blockchain, and in that blockchain this is going to be, and and for that blockchain we'll have to have a server and and clients right.

373
00:37:30.610 --> 00:37:36.549
Sourav Mishra: Yes, and that is internally handled by Grpc, using this definition.

374
00:37:38.090 --> 00:37:45.290
Gabor Szabo: The network connections are are implemented this way. Right? That's what you're saying.

375
00:37:45.630 --> 00:37:56.209
Sourav Mishra: Exactly. We'll learn more about that when we come to the main function, like how exactly a server is going to be implemented, how exactly. Our client has been implemented. So client is basically the wallet that I implemented. But we're not going to build that today.

376
00:37:56.510 --> 00:37:59.479
Sourav Mishra: So the second link that I sent to you. It's for the wallet.

377
00:38:00.090 --> 00:38:00.460
Gabor Szabo: It's.

378
00:38:00.610 --> 00:38:05.220
Sourav Mishra: Company built. You just use that to connect to the blockchain, and it also uses Grpc.

379
00:38:06.720 --> 00:38:07.410
Gabor Szabo: Yeah.

380
00:38:09.510 --> 00:38:16.969
Sourav Mishra: So the server part is kind of the last thing that I want to implement today. Before that I'm going to write the core logic of our blockchain and define the data types.

381
00:38:19.210 --> 00:38:24.869
Gabor Szabo: Okay. But so the the trans I mean the transaction has to be the same for all the clients. Right?

382
00:38:25.230 --> 00:38:31.724
Gabor Szabo: That's so. I wouldn't be able just to implement a client for an existing

383
00:38:32.730 --> 00:38:36.660
Gabor Szabo: blockchain without you reusing this information right?

384
00:38:37.630 --> 00:38:38.670
Sourav Mishra: Hmm!

385
00:38:40.080 --> 00:38:45.880
Sourav Mishra: I don't think I understood your question so like you want to define a client for an existing blockchain.

386
00:38:46.210 --> 00:38:46.930
Gabor Szabo: Yeah.

387
00:38:47.560 --> 00:38:57.499
Sourav Mishra: Yeah, you can do that. You don't have to create a blockchain for creating client like you can create a client for ethereum. You can create a client for Bitcoin. You can create client for Polka dot, or any other blockchain out there.

388
00:38:57.680 --> 00:39:01.580
Gabor Szabo: But you. You have to know what is the transaction format.

389
00:39:02.230 --> 00:39:18.880
Sourav Mishra: Yeah, you have to know what formats they follow like. What are the Rpcs. The blocks and has exposed to you that you can use in a client what what data types they use? What is the format? Etcetera, because all the code is open source. So you can just look at the code and check out.

390
00:39:18.880 --> 00:39:28.499
Gabor Szabo: I understand. Yeah, I didn't mean that you have to find out some secrets. It's just that you have to do the same thing as as they have done.

391
00:39:29.060 --> 00:39:29.810
Sourav Mishra: Yes.

392
00:39:30.140 --> 00:39:30.740
Gabor Szabo: Okay.

393
00:39:32.640 --> 00:39:33.430
Gabor Szabo: Okay.

394
00:39:33.830 --> 00:39:38.769
Sourav Mishra: All right, then I'm going to copy the same thing, the Debug Lone serialize and deserialize and put it there

395
00:39:38.930 --> 00:39:44.790
Sourav Mishra: and then I'm going to define a struct called block. So this is going to be our block structure.

396
00:39:45.200 --> 00:39:48.389
Sourav Mishra: and our blockchain is going to be made of these blocks.

397
00:39:50.420 --> 00:39:54.639
Sourav Mishra: So 1st thing that comes is the block number or the block index.

398
00:39:57.350 --> 00:39:59.320
Sourav Mishra: Then the timestamp

399
00:39:59.500 --> 00:40:04.939
Sourav Mishra: timestamp is basically the timestamp at which the block was added to the blockchain, like I mentioned earlier.

400
00:40:05.270 --> 00:40:11.450
Sourav Mishra: and it's going to be a date time of Utc.

401
00:40:14.160 --> 00:40:17.030
Sourav Mishra: Then we have transactions. So

402
00:40:17.890 --> 00:40:25.310
Sourav Mishra: we'll be using this thing that is defined here. It's going to be a web of good connection.

403
00:40:29.510 --> 00:40:36.069
Sourav Mishra: and then we'll have a previous hash. That is the previous hash of the that is, the hash of the previous block that this block comes after.

404
00:40:38.430 --> 00:40:41.159
Sourav Mishra: Then we'll have the hash of the current book

405
00:40:42.040 --> 00:40:46.549
Sourav Mishra: that is going to be used for for mining to check against this particular hash.

406
00:40:46.980 --> 00:40:48.710
Sourav Mishra: and of course the nonce.

407
00:40:50.270 --> 00:40:56.890
Sourav Mishra: This will be calculated again and again while you're mining, just to ensure that you match the difficulty, or you satisfy the difficulty of the blockchain.

408
00:40:57.340 --> 00:40:58.809
Sourav Mishra: and then pop! My number

409
00:40:59.150 --> 00:41:04.279
Sourav Mishra: miner, is the guy who mined this blockchain, so his address is there in the blockchain block.

410
00:41:06.830 --> 00:41:07.939
Sourav Mishra: Any doubts here.

411
00:41:11.360 --> 00:41:16.756
Gabor Szabo: No. But what? What kind of address is there? I mean, if you have an address of someone, then

412
00:41:17.810 --> 00:41:20.599
Gabor Szabo: there is some privacy issues. Right? So it's not.

413
00:41:20.600 --> 00:41:23.029
Sourav Mishra: No, the address is public for everyone to see.

414
00:41:25.052 --> 00:41:27.740
Sourav Mishra: Basically your public key.

415
00:41:28.160 --> 00:41:29.700
Gabor Szabo: The public key. Okay.

416
00:41:29.950 --> 00:41:35.640
Sourav Mishra: So instead of storing the public key directly, because I ran into serialization and deserialization issues.

417
00:41:35.800 --> 00:41:38.990
Sourav Mishra: So I am storing a string representation of the public key.

418
00:41:39.880 --> 00:41:40.770
Gabor Szabo: Okay.

419
00:41:40.770 --> 00:41:47.428
Sourav Mishra: So basically, a publicly is something really large, like, ff, a d, cp, something like this.

420
00:41:47.970 --> 00:41:51.870
Sourav Mishra: So I'm basically storing a string representation of this like this.

421
00:41:56.350 --> 00:41:59.840
Sourav Mishra: Okay, so this is our block struct and we're done in that.

422
00:42:00.770 --> 00:42:03.799
Sourav Mishra: The next thing to implement is our blockchain.

423
00:42:04.120 --> 00:42:07.010
Sourav Mishra: the stripe blockchain, not the real blockchain.

424
00:42:09.600 --> 00:42:14.730
Sourav Mishra: So the stripe blockchain will have a chain that is just a backup blocks.

425
00:42:21.340 --> 00:42:25.159
Sourav Mishra: Or our chain is basically a collection of blocks.

426
00:42:25.880 --> 00:42:28.480
Sourav Mishra: So hence the Vecer block, and then

427
00:42:28.890 --> 00:42:31.150
Sourav Mishra: we are going to have a transaction pool.

428
00:42:35.370 --> 00:42:38.279
Sourav Mishra: And this is where we're going to use the big Bq.

429
00:42:40.640 --> 00:42:42.860
Sourav Mishra: And of transaction.

430
00:42:45.750 --> 00:42:48.700
Sourav Mishra: And then we have del Ferra.

431
00:42:49.240 --> 00:42:51.029
Sourav Mishra: Pretty much consensus has already moved.

432
00:42:55.520 --> 00:43:00.529
Sourav Mishra: And, like I mentioned earlier, it's going to be box of dynamic dispatch because

433
00:43:01.160 --> 00:43:13.008
Sourav Mishra: we accept any consensus type that implements the trade consensus, so be it proof of work, or be it proof of stake as long as it implements the consensus trade for itself. We can accept that as

434
00:43:13.690 --> 00:43:16.229
Sourav Mishra: has the correct value for this type for this field.

435
00:43:20.940 --> 00:43:26.550
Sourav Mishra: Now we are done with this. So let's define our blockchain server.

436
00:43:34.940 --> 00:43:39.720
Sourav Mishra: It's going to take a blockchain, but it's not going to be something as simple as this.

437
00:43:41.310 --> 00:43:50.559
Sourav Mishra: because our blockchain server is actually a multi-threaded server. So on one thread it runs the server, and on another thread it mines the block

438
00:43:50.840 --> 00:43:51.690
Sourav Mishra: the blocks.

439
00:43:52.000 --> 00:43:57.879
Sourav Mishra: So that's why we're going to use arc and mutex arc, to ensure that

440
00:43:58.350 --> 00:44:06.579
Sourav Mishra: the references are shared safely between threads and mutex, to ensure that only one thread modifies the chain at one time.

441
00:44:06.700 --> 00:44:09.600
Sourav Mishra: because mutex provides us with a log a major electric block.

442
00:44:09.950 --> 00:44:16.039
Sourav Mishra: so that lock is especially helpful to ensure that only one thread modifies the blockchain at one time

443
00:44:16.470 --> 00:44:19.139
Sourav Mishra: to prevent the data is and deadlock and everything.

444
00:44:21.360 --> 00:44:25.030
Sourav Mishra: So yeah, this is our blockchain server, pretty simple just to contains the blockchain.

445
00:44:26.820 --> 00:44:30.123
Sourav Mishra: And then let's define the proof of what struck

446
00:44:34.580 --> 00:44:39.859
Sourav Mishra: this struct is the type for which we're going to implement the consensus trait.

447
00:44:40.500 --> 00:44:43.349
Sourav Mishra: It's going to have only one field. It's called difficulty

448
00:44:44.280 --> 00:44:49.629
Sourav Mishra: and difficulty. U, 32 basically means, if the difficulty is, let's say 2,

449
00:44:49.770 --> 00:44:54.019
Sourav Mishra: then that means the hash of the blocks would start with 2 zeros.

450
00:44:54.210 --> 00:45:00.000
Sourav Mishra: So the difficulty is 4. It means the hash of the block should start with 4 zeros for it to be accepted as a valid block.

451
00:45:00.750 --> 00:45:02.619
Sourav Mishra: So I'm taking it as U. 32.

452
00:45:03.720 --> 00:45:06.739
Sourav Mishra: So anything can go here that is under the

453
00:45:07.620 --> 00:45:13.260
Sourav Mishra: jurisdiction of the user who's starting the blockchain so it can be 2, 3, 4, anything that they want. So you 32 for that.

454
00:45:15.440 --> 00:45:19.470
Sourav Mishra: and then a simple implementation for proof of work. Just a constructor.

455
00:45:20.533 --> 00:45:25.180
Sourav Mishra: Mind you, this proof of work. Type is not the same as this proof of work variant.

456
00:45:25.640 --> 00:45:27.230
Sourav Mishra: Those are 2 different things.

457
00:45:30.160 --> 00:45:33.450
Sourav Mishra: So we write a function new for this.

458
00:45:36.710 --> 00:45:40.189
Sourav Mishra: and this parameter would be, I think this is correct.

459
00:45:40.910 --> 00:45:42.470
Sourav Mishra: Yeah, it's good.

460
00:45:43.030 --> 00:45:46.900
Sourav Mishra: So instead of proof of work, let's just say self to make it more generic. Yeah.

461
00:45:48.680 --> 00:46:02.669
Sourav Mishra: So we defined a proof of work struct for which we will be implementing the consensus type later, the consensus trait later, and we also implemented a constructor for the proof of work type that gives us a proof of work. Object by initializing the difficulty.

462
00:46:05.700 --> 00:46:10.499
Sourav Mishra: No, that's out of the way. Any doubts here, so far like the types you are defined so far.

463
00:46:16.980 --> 00:46:18.950
Sourav Mishra: So anything you want to go over again.

464
00:46:19.980 --> 00:46:22.520
Gabor Szabo: No, no! What was the question? Sorry?

465
00:46:23.240 --> 00:46:24.900
Sourav Mishra: I was asking if you guys have any doubts.

466
00:46:29.930 --> 00:46:31.600
Gabor Szabo: Looks good so far.

467
00:46:33.050 --> 00:46:37.219
Sourav Mishra: So now that we are done with all the types, let's start implementing stuff

468
00:46:37.740 --> 00:46:39.769
Sourav Mishra: like giving meaning to the types.

469
00:46:40.810 --> 00:46:43.810
Sourav Mishra: So the 1st thing we're gonna implement is

470
00:46:45.470 --> 00:46:52.970
Sourav Mishra: just a general implementation for the consensus type enum. This is the same enum as this, and

471
00:46:54.660 --> 00:46:57.849
Sourav Mishra: it will have a function called create consensus

472
00:47:01.520 --> 00:47:06.989
Sourav Mishra: that takes self, sorry ambers and so forth.

473
00:47:08.670 --> 00:47:12.730
Sourav Mishra: And it returns. The box consensus

474
00:47:13.270 --> 00:47:15.589
Sourav Mishra: which we need to use in our blockchain.

475
00:47:23.840 --> 00:47:26.150
Sourav Mishra: We're going to match because it's an enum.

476
00:47:27.820 --> 00:47:31.410
Sourav Mishra: So if it is let's say, self

477
00:47:33.420 --> 00:47:35.730
Sourav Mishra: proof of what type of difficulty?

478
00:47:38.000 --> 00:47:40.680
Sourav Mishra: Something wrong? Yeah, nobody.

479
00:47:41.900 --> 00:47:52.520
Sourav Mishra: Then we are going to return walks of new approve of what this is, how we just

480
00:47:52.750 --> 00:47:55.990
Sourav Mishra: work with boxes. So it's just plain the syntax

481
00:47:59.480 --> 00:48:04.350
Sourav Mishra: and difficulty something on.

482
00:48:08.510 --> 00:48:12.860
Sourav Mishra: Okay, we have not implemented consensus for proof of work. So yeah, it will go away soon.

483
00:48:13.450 --> 00:48:16.770
Sourav Mishra: Then let's do the same thing with self of proof of stake type.

484
00:48:18.960 --> 00:48:24.320
Sourav Mishra: We are not gonna implement that this time. But let's just have it, because otherwise our enemy is not complete.

485
00:48:26.850 --> 00:48:30.340
Sourav Mishra: I'll just put mean stake, that is this.

486
00:48:30.940 --> 00:48:33.280
Sourav Mishra: And here we can use the Tudo macro

487
00:48:33.520 --> 00:48:39.790
Sourav Mishra: because we're not gonna do it right now think it works good.

488
00:48:40.710 --> 00:48:55.478
Sourav Mishra: So what this create consensus does is it returns us a consensus box consensus based on the type we have selected. So if you want a proof of work consensus type, then we pass on the difficulty, and then we get a corresponding box type.

489
00:48:56.000 --> 00:49:00.070
Sourav Mishra: given given to us. And if you want to start with the proof of stake type, then

490
00:49:00.350 --> 00:49:03.890
Sourav Mishra: we'll do the same thing here, just copy and paste this.

491
00:49:04.490 --> 00:49:06.750
Sourav Mishra: instead of proof of work, make it proof of string.

492
00:49:07.210 --> 00:49:09.720
Sourav Mishra: and then, instead of new difficulty, make it main stick.

493
00:49:12.470 --> 00:49:18.400
Sourav Mishra: But we do not have a proof of stake type yet. So we are not going to implement this. Just keep it blank user.

494
00:49:18.400 --> 00:49:34.850
Gabor Szabo: One second, just one second. Sorry. So someone is asking about the the to do, Macro. So it's just panics with, with some explanation that it's you're about to implement that later on. So it's just a syntax placeholder.

495
00:49:37.630 --> 00:49:49.360
Sourav Mishra: So when you have to return something. But you don't have a particular return for that I generally use for when on in those cases you can do something like this. Also return your your empty type, but then it expects something like this.

496
00:49:49.530 --> 00:49:53.320
Sourav Mishra: so you can't easily, really use this. Hence the tutor Macro comes in handy.

497
00:49:56.180 --> 00:50:02.320
Sourav Mishra: and if you want, then you can add more types also here, like you can add a proof of authority type as well.

498
00:50:03.480 --> 00:50:11.470
Sourav Mishra: But again, that's up to you. If you want to implement those you can, you're free to do that. But for today's session we're not going to implement any of these just proof of work? Right?

499
00:50:13.310 --> 00:50:14.050
Sourav Mishra: Okay?

500
00:50:14.200 --> 00:50:21.839
Sourav Mishra: So it's now done, the consensor type implementation. Let's move on to implement block.

501
00:50:25.190 --> 00:50:30.949
Sourav Mishra: So block implementation will have 2 functions. One is obviously the constructor. That will give us a new block.

502
00:50:32.660 --> 00:50:36.429
Sourav Mishra: and it has the block index, which is v. 64.

503
00:50:38.810 --> 00:50:43.660
Sourav Mishra: And then you have the transactions, which is a lack of transaction.

504
00:50:46.780 --> 00:50:48.900
Sourav Mishra: and finally, the previous S.

505
00:50:51.940 --> 00:50:58.119
Sourav Mishra: It is a single did I misspell it, Stanga.

506
00:51:01.930 --> 00:51:04.790
Sourav Mishra: and it returns self. That is, it returns a block.

507
00:51:05.400 --> 00:51:11.319
Sourav Mishra: It takes in these parameters, and it returns a block to me.

508
00:51:12.340 --> 00:51:25.670
Sourav Mishra: So we're gonna construct a block. So let's new block smooth of type, block equals to self

509
00:51:29.870 --> 00:51:35.900
Sourav Mishra: index can be written as it is sort and syntax timestamp would be utc now

510
00:51:36.920 --> 00:51:41.379
Sourav Mishra: like the current timestamp, though time, when this thing is executed.

511
00:51:41.800 --> 00:51:44.540
Sourav Mishra: and then transactions again can be used as is

512
00:51:45.560 --> 00:51:49.669
Sourav Mishra: previous, has same thing with previous has that gave me everything will be nice.

513
00:51:50.200 --> 00:51:57.699
Sourav Mishra: So currently that hash of the block is empty, because we have not calculated that we need to calculate that amount

514
00:51:58.330 --> 00:52:06.219
Sourav Mishra: using the mining techniques, and the miner is also empty now, because you do not know who the miner is and the nonce is 0 again.

515
00:52:06.780 --> 00:52:10.249
Sourav Mishra: because that's something we're going to calculate repeatedly to make sure the hash

516
00:52:10.550 --> 00:52:13.269
Sourav Mishra: is in accordance with the difficulty of the blockchain.

517
00:52:17.200 --> 00:52:25.420
Sourav Mishra: Now I can go to block dot hash equals to blocked out. Calculate has.

518
00:52:26.130 --> 00:52:29.579
Sourav Mishra: So this calculate has method has not been defined yet.

519
00:52:31.470 --> 00:52:36.320
Sourav Mishra: and we're going to return block this.

520
00:52:40.390 --> 00:52:44.390
Sourav Mishra: Now. The very next function that we're going to define is calculate has, of course.

521
00:52:47.330 --> 00:52:51.930
Sourav Mishra: it takes a reference to self, that is, it takes a block, it's a method.

522
00:52:52.410 --> 00:52:56.270
Sourav Mishra: and it returns a string which is the hash of the block.

523
00:52:58.530 --> 00:53:03.360
Sourav Mishra: So to calculate hash, we need to initialize a hazard with the Sar. 256 new.

524
00:53:04.190 --> 00:53:06.370
Sourav Mishra: You can already see the syntax being suggested.

525
00:53:16.200 --> 00:53:21.679
Sourav Mishra: No, and then let's create a string to hash.

526
00:53:22.230 --> 00:53:28.189
Sourav Mishra: It cannot directly hash the struct as it is, so just need to create a string with the contents of the struct.

527
00:53:28.330 --> 00:53:33.579
Sourav Mishra: so we can call it input, or we can call it content. Whatever sales about. I just call it top

528
00:53:33.780 --> 00:53:35.030
Sourav Mishra: block content.

529
00:53:38.200 --> 00:53:44.050
Sourav Mishra: And it's gonna be, here's where you're gonna use certain

530
00:53:47.250 --> 00:53:49.040
Sourav Mishra: serialize it into a string.

531
00:53:59.000 --> 00:54:02.490
Sourav Mishra: What else is there self?

532
00:54:02.610 --> 00:54:04.350
Sourav Mishra: And on next.

533
00:54:05.720 --> 00:54:13.990
Sourav Mishra: So the timestamp. Basically everything that is contained in the block. We are going to hash that wait.

534
00:54:14.190 --> 00:54:17.289
Sourav Mishra: concatenate them, end to end and hash that

535
00:54:23.080 --> 00:54:34.560
Sourav Mishra: 12 floor transactions and self dot previouss and finally set up nods.

536
00:54:38.450 --> 00:54:40.199
Sourav Mishra: Then we're gonna unwrap this.

537
00:54:40.610 --> 00:54:44.750
Sourav Mishra: You should not actually force unwrap this because it's unsafe.

538
00:54:44.950 --> 00:54:52.969
Sourav Mishra: You should check if it produces a valid result and then unwrap it or handle the result accordingly, using an if letter match statement.

539
00:54:54.200 --> 00:54:56.050
Sourav Mishra: But for simplicity, I'll just use unwrapped.

540
00:54:57.190 --> 00:55:05.440
Sourav Mishra: And then you're gonna use hazard, dot update block content.

541
00:55:05.780 --> 00:55:12.490
Sourav Mishra: Don't ask bytes again. This is something that the hazard expects for us to do.

542
00:55:13.400 --> 00:55:16.350
Sourav Mishra: and then they're going to encode that into a hex string

543
00:55:28.660 --> 00:55:29.430
Sourav Mishra: keeps.

544
00:55:29.810 --> 00:55:32.929
Sourav Mishra: So now we also defined an implementation for our block.

545
00:55:33.110 --> 00:55:39.190
Sourav Mishra: So it has a new method, a new associated function that gives you a returns you a block.

546
00:55:39.420 --> 00:55:46.109
Sourav Mishra: and then a calculate hash method that takes the content of the block and creates a hash out of it.

547
00:55:46.790 --> 00:55:50.709
Sourav Mishra: So to use the has to calculate the hash, we're using Ssa 256.

548
00:55:51.290 --> 00:55:58.730
Sourav Mishra: That's the hashing algorithm. So we 1st initialize that with the hash, and then we create a string, using survey to string.

549
00:55:59.270 --> 00:56:00.490
Sourav Mishra: Then we

550
00:56:00.610 --> 00:56:06.440
Sourav Mishra: convert that string to bytes, and then we give those bytes to the hazard to actually has it.

551
00:56:06.560 --> 00:56:14.499
Sourav Mishra: And then, finally, after hashing it, we use hex import to import the final hash into a readable string or a storeable string.

552
00:56:15.080 --> 00:56:17.429
Sourav Mishra: because that's what we are expecting as a return.

553
00:56:19.060 --> 00:56:20.309
Sourav Mishra: Any doubts here.

554
00:56:24.200 --> 00:56:25.690
Gabor Szabo: Not from me.

555
00:56:26.720 --> 00:56:27.480
Gabor Szabo: Thanks.

556
00:56:27.700 --> 00:56:30.719
Sourav Mishra: I don't see any insights, either, so I guess you can proceed

557
00:56:33.300 --> 00:56:42.630
Sourav Mishra: again. This is all syntax like, I'm not defining these functions anywhere like, I have not defined update function anywhere, or I've not defined finalize function anywhere.

558
00:56:42.760 --> 00:56:45.830
Sourav Mishra: It comes to me when I use this.

559
00:56:46.050 --> 00:56:47.889
Sourav Mishra: These are all defined on this

560
00:56:50.600 --> 00:56:53.099
Sourav Mishra: alright. So our block implementation is done.

561
00:56:54.060 --> 00:56:55.850
Sourav Mishra: We are almost halfway there

562
00:56:58.020 --> 00:57:04.390
Sourav Mishra: and then, finally, a big chunk of our implementation implement consensus for proof of work.

563
00:57:05.990 --> 00:57:08.619
Sourav Mishra: This is one of the major parts of our program.

564
00:57:10.340 --> 00:57:17.029
Sourav Mishra: What basically, this implementation is gonna do is it is gonna make proof of work type compatible to be used

565
00:57:17.170 --> 00:57:18.870
Sourav Mishra: ought to be returned like this.

566
00:57:19.310 --> 00:57:23.009
Sourav Mishra: and in turn to be used in our blockchain.

567
00:57:24.450 --> 00:57:29.899
Sourav Mishra: So currently, I can't just use proof of work as a consensus because it has not implemented the consensus rate yet.

568
00:57:30.200 --> 00:57:34.610
Sourav Mishra: and it needs to implement that consistent state because I am using dynamic dispatch with box.

569
00:57:36.380 --> 00:57:38.390
Sourav Mishra: So although I'm returning it here.

570
00:57:38.580 --> 00:57:43.809
Sourav Mishra: You can see if I comment this, I get an error here

571
00:57:44.210 --> 00:57:47.399
Sourav Mishra: because consensus is not implemented by proof of our collect.

572
00:57:47.700 --> 00:57:51.709
Sourav Mishra: So if I implement this, or if I just tried right this.

573
00:57:51.930 --> 00:57:53.490
Sourav Mishra: then you can see these error groups.

574
00:57:53.840 --> 00:57:57.399
Sourav Mishra: because now we have an implementation for consensor, for

575
00:57:59.600 --> 00:58:07.290
Sourav Mishra: so we can see the methods that the consensus trade incorporates these 4 methods. Let's just copy and paste this for simplicity.

576
00:58:09.650 --> 00:58:11.209
Sourav Mishra: So we're gonna implement these.

577
00:58:11.560 --> 00:58:16.219
Sourav Mishra: So the 1st method that they're going to implement is the generate block method

578
00:58:22.000 --> 00:58:24.249
Sourav Mishra: amenities, because it's distracting. Yeah.

579
00:58:25.640 --> 00:58:30.340
Sourav Mishra: So how do we generate a block when it comes to proof of work.

580
00:58:32.070 --> 00:58:36.980
Sourav Mishra: I just explained how mining works and everything works. So how exactly do we generate a block in proof of works

581
00:58:38.130 --> 00:58:45.659
Sourav Mishra: so generating a block means we are finding a block that whose hash matches the difficulty parameters.

582
00:58:46.380 --> 00:58:55.170
Sourav Mishra: So we're gonna say, let mute block equals to block of new.

583
00:58:58.510 --> 00:59:05.089
Sourav Mishra: We can see what's provided to us. Already we have the index. We have the transactions, and we have the previous has

584
00:59:05.340 --> 00:59:14.430
Sourav Mishra: exactly the things that we need for this new function. We need the index, the transactions, and the previous S, and we are getting that

585
00:59:14.960 --> 00:59:16.099
Sourav Mishra: in this.

586
00:59:17.370 --> 00:59:20.190
Sourav Mishra: So I can just directly pass it like this index.

587
00:59:20.780 --> 00:59:22.539
Sourav Mishra: and the accents and previous has.

588
00:59:23.110 --> 00:59:24.549
Sourav Mishra: And now I have a block.

589
00:59:25.570 --> 00:59:32.459
Sourav Mishra: Now, what do I do with this block? I I got a block. Yes, but right now this block does not satisfy the difficulty, like I do not know if it does

590
00:59:33.040 --> 00:59:36.449
Sourav Mishra: so, I would say I would 1st set a target

591
00:59:39.730 --> 00:59:42.640
Sourav Mishra: equal to 0, and I'll repeat it.

592
00:59:43.390 --> 00:59:48.270
Sourav Mishra: How many times do I get to repeat it? I get to repeat it difficulty number of times.

593
00:59:48.470 --> 00:59:57.799
Sourav Mishra: like I showed back. Then, if my difficulty is 3, then this 0 is repeated 3 times, if my difficulty is 4, it's repeated 4 times, so on and so forth.

594
01:00:00.690 --> 01:00:01.940
Sourav Mishra: Something wrong.

595
01:00:11.850 --> 01:00:14.339
Gabor Szabo: And wanted you you to make it u size.

596
01:00:15.270 --> 01:00:21.240
Sourav Mishra: Yes, I'll do one better. I'll make this 2 size.

597
01:00:22.100 --> 01:00:22.950
Gabor Szabo: Oh, okay.

598
01:00:27.470 --> 01:00:35.780
Sourav Mishra: So that is done. Now, we have also defined our target. Now, we just need to check in a loop whether or not our block starts with this given target.

599
01:00:36.520 --> 01:00:38.110
Sourav Mishra: So we're going to take a while loop.

600
01:00:38.350 --> 01:00:45.310
Sourav Mishra: because that's the most feasible here. And we're gonna check. If the block pass starts with

601
01:00:48.920 --> 01:00:50.010
Sourav Mishra: our target.

602
01:00:53.610 --> 01:01:05.020
Sourav Mishra: Our target is basically a string of zeros, repeated difficulty number of times. So if our difficulty is 3, then our target is 3 3 zeros. If our difficulty is 4, then our target is 4 zeros, so on and so forth.

603
01:01:05.190 --> 01:01:11.170
Sourav Mishra: So I'm checking. If my, it's block, not black. Sorry. Yeah, if the block has starts with

604
01:01:11.550 --> 01:01:14.329
Sourav Mishra: differentiating number of zeros. That is our target.

605
01:01:14.600 --> 01:01:19.960
Sourav Mishra: then it's a valid block. Otherwise I enter the loop, and I increment the nonce by one.

606
01:01:20.120 --> 01:01:21.610
Sourav Mishra: and recalculate the hash again.

607
01:01:25.350 --> 01:01:32.869
Sourav Mishra: So by the time I exit this loop I expect that my block has is actually the asset I need. So I can just return the block.

608
01:01:35.840 --> 01:01:40.290
Sourav Mishra: So this ends our generate block method. In case of proof of work

609
01:01:40.470 --> 01:01:44.619
Sourav Mishra: for proof of stake, it will be substantially different. So let's not go there right now. But yeah.

610
01:01:44.950 --> 01:01:55.170
Sourav Mishra: the reason why we have a consensus rate is because these methods are going to differ between algorithms. So for proof of work which is different for proof of sticker, it's going to be different. So on and so forth.

611
01:01:56.480 --> 01:01:58.570
Sourav Mishra: Let's start with the validated block. Now.

612
01:02:03.130 --> 01:02:09.870
Sourav Mishra: So for validating a block and proof of work, or, let's say, any consensus, it's gonna be bit generic.

613
01:02:10.100 --> 01:02:17.050
Sourav Mishra: We're going to see that if the blocks previous has is actually equal to the previous has like it's during the previous has.

614
01:02:17.180 --> 01:02:22.579
Sourav Mishra: And we're going to check if the block hash is actually equal to the calculated hash of the block.

615
01:02:23.030 --> 01:02:27.510
Sourav Mishra: And then we're gonna finally check. If the block hash starts with the target.

616
01:02:28.750 --> 01:02:36.130
Sourav Mishra: So those 3 things here, we're just gonna go one by one over them. The 1st thing if Block Dot previous has equals to previous has

617
01:02:39.680 --> 01:02:42.280
Sourav Mishra: articles to big business.

618
01:02:43.780 --> 01:02:49.140
Sourav Mishra: then I'm going to say, that the block is not valid, so I return false.

619
01:02:54.218 --> 01:03:00.710
Sourav Mishra: Then I'm going to check if the block has is

620
01:03:01.350 --> 01:03:04.180
Sourav Mishra: what I calculate using. The calculate has method.

621
01:03:06.700 --> 01:03:10.190
Sourav Mishra: If it is not, then again return false.

622
01:03:13.750 --> 01:03:17.739
Sourav Mishra: And finally, I'm gonna check. If the block has starts with

623
01:03:25.170 --> 01:03:28.929
Sourav Mishra: 0 repeated difficulty number of times.

624
01:03:34.410 --> 01:03:40.489
Sourav Mishra: So these are the 3 checks that I am checking to ensure that the blog that I just received is a valid blog or not.

625
01:03:41.130 --> 01:03:45.019
Sourav Mishra: This check is necessary, because the one that I calculated here

626
01:03:45.880 --> 01:03:52.160
Sourav Mishra: has the has as an empty string, the nonce has 0, and the minor also as distinct

627
01:03:52.570 --> 01:03:54.170
Sourav Mishra: an empty address.

628
01:03:54.970 --> 01:04:00.739
Sourav Mishra: But this has changed. At least these 2 are changed when I and go to generate, blog.

629
01:04:01.370 --> 01:04:09.759
Sourav Mishra: So generate block updates my nons and my has until and unless it satisfies this difficulty, and when it satisfies the difficulty, it returns me the block.

630
01:04:10.110 --> 01:04:13.509
Sourav Mishra: So after I am calling this generate block function, my block has changed.

631
01:04:13.800 --> 01:04:16.689
Sourav Mishra: At least these 2 fields may not work have changed

632
01:04:18.350 --> 01:04:21.449
Sourav Mishra: so because they have changed. I need to check that.

633
01:04:21.740 --> 01:04:28.200
Sourav Mishra: The current block has equals to. When I calculate. The has 1st just a fallback method

634
01:04:28.380 --> 01:04:32.840
Sourav Mishra: just to ensure that it all works out alright.

635
01:04:33.910 --> 01:04:36.790
Sourav Mishra: And then we have the start method.

636
01:04:42.290 --> 01:04:51.899
Sourav Mishra: So the start method is where we're going to implement a separate thread that is gonna start our consensus algorithm and start mining and all.

637
01:04:52.080 --> 01:04:58.499
Sourav Mishra: So we're going to implement this a bit later, and because it returns nothing. So I can just do this. And it's gonna work.

638
01:04:58.930 --> 01:05:02.930
Sourav Mishra: And for now let's do this, will it work, or should it. Clown. Yeah, that's okay.

639
01:05:04.650 --> 01:05:06.400
Sourav Mishra: We are going to implement that later.

640
01:05:06.850 --> 01:05:10.409
Sourav Mishra: And the name function is the simplest function in this.

641
01:05:10.590 --> 01:05:11.780
Sourav Mishra: And I pause it.

642
01:05:12.050 --> 01:05:13.940
Sourav Mishra: Just return the name of the consensus.

643
01:05:14.960 --> 01:05:15.790
Sourav Mishra: That's all.

644
01:05:17.520 --> 01:05:18.220
Sourav Mishra: Yeah.

645
01:05:20.290 --> 01:05:21.769
Sourav Mishra: Any doubts so far.

646
01:05:22.000 --> 01:05:25.720
Sourav Mishra: Don't worry about the start. We're gonna do that later, very soon. In fact.

647
01:05:28.240 --> 01:05:30.939
Gabor Szabo: No, I think you're going. You can see you can go ahead.

648
01:05:31.550 --> 01:05:32.160
Sourav Mishra: Okay.

649
01:05:36.670 --> 01:05:37.440
Sourav Mishra: alright.

650
01:05:38.890 --> 01:05:45.349
Sourav Mishra: So we're done with the implementation for consensus for proof of work. Now let's implement blockchain.

651
01:05:51.560 --> 01:05:55.729
Sourav Mishra: So again, as usual, start with our constructor

652
01:06:02.050 --> 01:06:05.830
Sourav Mishra: to initialize a blockchain, we are gonna start it with a consensus.

653
01:06:19.270 --> 01:06:27.589
Sourav Mishra: 1st thing that we do is we need to define the Genesis block. Genesis block is the 1st block in the blockchain. It's called a Genesis block.

654
01:06:28.930 --> 01:06:30.760
Sourav Mishra: So when we start a blockchain.

655
01:06:31.920 --> 01:06:34.010
Sourav Mishra: we need to define the Genesis block.

656
01:06:34.530 --> 01:06:38.899
Sourav Mishra: and from there the blocks into the roots like new blocks get added to that.

657
01:06:42.320 --> 01:06:45.938
Sourav Mishra: Then it says, blocks based on this. It's

658
01:06:47.360 --> 01:06:52.710
Sourav Mishra: Then census dot generate block.

659
01:06:56.650 --> 01:07:01.460
Sourav Mishra: So we wrote this generate block function method to give us a block.

660
01:07:02.170 --> 01:07:04.660
Sourav Mishra: it returns a block. So we are going to use that.

661
01:07:05.970 --> 01:07:08.630
Sourav Mishra: And internally it uses block of name.

662
01:07:10.600 --> 01:07:13.269
Sourav Mishra: So you're gonna pass the arguments now.

663
01:07:13.530 --> 01:07:15.553
Sourav Mishra: So the index is

664
01:07:16.750 --> 01:07:20.159
Sourav Mishra: it's going to be 0 because it's in this block. So we start with 0,

665
01:07:20.470 --> 01:07:26.850
Sourav Mishra: and then we do not have any transactions yet in the dentist blog. So let's do this.

666
01:07:27.798 --> 01:07:33.859
Sourav Mishra: We can do wake off new, or we can just do a web macro with empty whatever sales about ending works.

667
01:07:35.100 --> 01:07:42.079
Sourav Mishra: And finally, the previous has is going to be 0,

668
01:07:45.630 --> 01:07:50.790
Sourav Mishra: because the genesis block is the 1st block, so there is no previous hash as such.

669
01:07:51.090 --> 01:07:52.890
Sourav Mishra: Hence let's keep it 0.

670
01:07:54.940 --> 01:07:58.710
Sourav Mishra: So that's done, then let's add a login

671
01:08:24.340 --> 01:08:25.929
Sourav Mishra: and return self.

672
01:08:26.620 --> 01:08:33.750
Sourav Mishra: So for chain, we have our initial block ready. So it's going to be Genesis block of deck.

673
01:08:34.880 --> 01:08:40.589
Sourav Mishra: And then for the transaction pool, it's going to be empty. We're clicking you because we don't have

674
01:08:40.819 --> 01:08:43.510
Sourav Mishra: a transaction pool rate just while you're starting.

675
01:08:44.430 --> 01:08:48.400
Sourav Mishra: And then the consensus, the one that we received here.

676
01:08:50.590 --> 01:08:53.600
Sourav Mishra: So that's the constructor for the the blockchain.

677
01:08:57.050 --> 01:09:01.859
Sourav Mishra: Now, we're going to define a function that is going to add a transaction to our blockchain

678
01:09:05.779 --> 01:09:06.740
Sourav Mishra: so.

679
01:09:09.940 --> 01:09:11.120
Sourav Mishra: or

680
01:09:13.220 --> 01:09:17.999
Sourav Mishra: describe the success. If the translation was added successfully, then it's true. If it was not, then it's false.

681
01:09:18.560 --> 01:09:26.190
Sourav Mishra: It's going to be taking a mutable self because the blockchain state is going to change. Once we add the transaction, so it's supposed to be mutable.

682
01:09:27.390 --> 01:09:29.300
Sourav Mishra: And then the transaction

683
01:09:32.430 --> 01:09:34.639
Sourav Mishra: is the transaction that you are trying to add

684
01:09:44.149 --> 01:09:47.920
Sourav Mishra: so like I mentioned earlier, if the transaction is from a faucet

685
01:09:48.822 --> 01:09:53.290
Sourav Mishra: like, if it is a faucet transaction, then we're just going to skip it and return through.

686
01:09:58.340 --> 01:09:59.840
Gabor Szabo: What is this for, sir?

687
01:10:00.980 --> 01:10:01.570
Sourav Mishra: Sorry.

688
01:10:01.890 --> 01:10:03.150
Gabor Szabo: What is this?

689
01:10:04.390 --> 01:10:06.369
Gabor Szabo: Was it? Mock chain address.

690
01:10:07.616 --> 01:10:09.970
Sourav Mishra: It's the const we have defined above.

691
01:10:12.270 --> 01:10:17.839
Sourav Mishra: So basically, a faucet is somewhere where you request funds from the blockchain for testing the blockchain out.

692
01:10:18.110 --> 01:10:25.920
Sourav Mishra: So if I am requesting funds to test the blockchain out, then I don't really need to go through verification and everything, because it's just a faucet request.

693
01:10:26.020 --> 01:10:28.149
Sourav Mishra: so it should automatically pass through.

694
01:10:29.540 --> 01:10:30.610
Gabor Szabo: Okay.

695
01:10:31.620 --> 01:10:35.990
Sourav Mishra: Yeah. So I'm skipping everything that follows. If the transaction is a partial transaction.

696
01:10:39.000 --> 01:10:40.990
Sourav Mishra: I will add a logger here.

697
01:10:41.460 --> 01:10:44.719
Sourav Mishra: This, let's say it's adding transaction to the faucet.

698
01:10:48.870 --> 01:10:51.269
Sourav Mishra: The transaction from faucet to pool

699
01:11:08.710 --> 01:11:16.100
Sourav Mishra: transaction.com and transaction dot month.

700
01:11:16.520 --> 01:11:19.159
Sourav Mishra: Let's make it a bit more robust.

701
01:11:20.420 --> 01:11:30.640
Sourav Mishra: Shit faucet address is this and online is this?

702
01:11:33.340 --> 01:11:36.510
Sourav Mishra: Yeah, that's done.

703
01:11:37.350 --> 01:11:47.179
Sourav Mishra: And I'm going to add this transaction to the transaction pool, because, even though it's a false transaction, it's still a valid transaction, and it is supposed to be inside the pool to mine.

704
01:11:48.610 --> 01:11:53.389
Sourav Mishra: I have the connection here. So, and finally, on returning to

705
01:11:57.540 --> 01:11:58.270
Sourav Mishra: okay.

706
01:11:58.900 --> 01:12:07.929
Sourav Mishra: so we successfully handled the faucet transaction. If it is a faucet transaction, then you will log that it's a faucet transaction. Then you push the transaction to the pool, and then you return through.

707
01:12:09.900 --> 01:12:12.249
Sourav Mishra: Next, we're going to verify the transaction.

708
01:12:15.350 --> 01:12:18.740
Sourav Mishra: If transaction got verified.

709
01:12:19.730 --> 01:12:22.939
Sourav Mishra: Wait. Have you defined these methods before? Just a sec?

710
01:12:30.450 --> 01:12:32.100
Sourav Mishra: Okay, no, we have not. But all right.

711
01:12:33.030 --> 01:12:37.009
Sourav Mishra: If transaction dot verify, then just return through here.

712
01:12:37.430 --> 01:12:39.219
Sourav Mishra: Are you going to implement this later?

713
01:12:41.050 --> 01:12:50.360
Sourav Mishra: Then I've self or check balance. So this check balance method. What it does is it checks. If the

714
01:12:50.520 --> 01:12:54.570
Sourav Mishra: sender has enough balance to send it to the receiver. If not, then

715
01:12:55.540 --> 01:12:58.650
Sourav Mishra: I return false. Sorry this should be right.

716
01:12:59.650 --> 01:13:02.639
Sourav Mishra: If not connection, verifier return false.

717
01:13:03.130 --> 01:13:08.559
Sourav Mishra: It's not then return false again. We're gonna implement these later.

718
01:13:10.660 --> 01:13:21.249
Sourav Mishra: So at the end. If everything is successful, like, if the transaction is successfully verified and the balance checks out like. If the sender has enough balance to send to the receiver.

719
01:13:21.360 --> 01:13:30.070
Sourav Mishra: Then again add a log, adding, and the action to pull

720
01:13:39.370 --> 01:13:43.880
Sourav Mishra: from this who love this

721
01:13:51.590 --> 01:13:55.880
Sourav Mishra: works in the form and section dot 2.

722
01:13:58.070 --> 01:14:01.539
Sourav Mishra: So we have. Yeah.

723
01:14:02.840 --> 01:14:07.279
Sourav Mishra: the log is added. Now again, we are going to do the same thing that we already did. Here.

724
01:14:07.510 --> 01:14:11.790
Sourav Mishra: Are you going to push it to the transaction pool.

725
01:14:12.050 --> 01:14:14.030
Sourav Mishra: and we are going to return to

726
01:14:16.180 --> 01:14:21.619
Sourav Mishra: so verify and check balance have not been defined yet. That's why we're getting this error. Let's define them.

727
01:14:27.200 --> 01:14:30.840
Sourav Mishra: So let's 1st define check balance so that the error will go away.

728
01:14:32.990 --> 01:14:35.619
Sourav Mishra: Check balance takes a reference to the boxing.

729
01:14:37.560 --> 01:14:40.580
Sourav Mishra: It returns the balance.

730
01:14:40.860 --> 01:14:42.910
Sourav Mishra: Oh, yeah, it also takes the address

731
01:14:43.670 --> 01:14:46.449
Sourav Mishra: of for which we are finding out the balance

732
01:14:47.600 --> 01:14:50.120
Sourav Mishra: so addresses 1% St. O

733
01:15:10.630 --> 01:15:15.020
Sourav Mishra: fine, which you want checking the balance.

734
01:15:15.450 --> 01:15:17.310
Sourav Mishra: That is 0 65.

735
01:15:17.510 --> 01:15:27.040
Sourav Mishra: So what this function basically does is it takes an address, and it takes an amount and it checks. If the address has at least this much amount present in their wallet. If it does not, then

736
01:15:27.420 --> 01:15:44.920
Sourav Mishra: I'm trying to turn on my mind so like balance equals to self.net dwellings of address.

737
01:15:46.510 --> 01:15:53.320
Sourav Mishra: and if balance greater than equal to amount, then return true.

738
01:15:54.690 --> 01:15:59.060
Sourav Mishra: Again, we get get balance is not there.

739
01:16:00.250 --> 01:16:02.500
Sourav Mishra: so let's define. Get balance. Next

740
01:16:10.070 --> 01:16:11.950
Sourav Mishra: returns the balance to me.

741
01:16:12.810 --> 01:16:14.360
Sourav Mishra: 64.

742
01:16:20.000 --> 01:16:22.620
Sourav Mishra: It takes in a reference to the server

743
01:16:24.360 --> 01:16:28.440
Sourav Mishra: and an address for which you are fetching the balance.

744
01:16:32.690 --> 01:16:35.579
Sourav Mishra: So let's send it mutable balance

745
01:16:35.880 --> 01:16:45.439
Sourav Mishra: equal to 0. So how we're going to do this is, we're gonna query the blockchain. And we're going to query every block of the blockchain, and wherever this address appears

746
01:16:45.570 --> 01:16:51.200
Sourav Mishra: we are gonna check. What is the opening and closing balance? Basically, and then

747
01:16:53.300 --> 01:17:01.300
Sourav Mishra: add it to the balance that you have defined above this new balance, and then return that so

748
01:17:01.470 --> 01:17:15.660
Sourav Mishra: for block and self blockchain, so for blocks in the blockchain and for transaction, and

749
01:17:16.170 --> 01:17:20.620
Sourav Mishra: blocked of transaction, because transactions is also a vector

750
01:17:21.750 --> 01:17:25.410
Sourav Mishra: so we're checking each transaction. And if transaction dot from address

751
01:17:26.630 --> 01:17:29.679
Sourav Mishra: equals to the address that we have passed here.

752
01:17:30.440 --> 01:17:37.179
Sourav Mishra: Then sorry it's not from address extended to address. Yeah.

753
01:17:37.630 --> 01:17:49.490
Sourav Mishra: if transaction to address is equal to the address that we have passed here, that is, if the recipient is the address that we are looking the balance for, then add the amount to the balance

754
01:17:58.560 --> 01:17:59.840
Sourav Mishra: extended amount

755
01:18:05.600 --> 01:18:06.520
Sourav Mishra: they're

756
01:18:09.680 --> 01:18:18.929
Sourav Mishra: transaction. Dot from address is equal to the address that means, if you have sent some money, then you should. Your balance is going to be deducted.

757
01:18:20.420 --> 01:18:26.030
Sourav Mishra: So this is the simpler way to do this, but this might result in

758
01:18:26.210 --> 01:18:31.319
Sourav Mishra: underflow and overflow. So it's better to use saturating sub.

759
01:18:45.430 --> 01:18:47.845
Sourav Mishra: And then finally, the last thing, if

760
01:18:49.790 --> 01:18:53.709
Sourav Mishra: the block minus address is the address that we have given.

761
01:18:53.810 --> 01:18:56.550
Sourav Mishra: or that the address is for the address which we're querying for.

762
01:18:56.770 --> 01:19:02.360
Sourav Mishra: then add a reward for mining the block. Let's say it can be any arbitrary reward that you can set

763
01:19:02.960 --> 01:19:08.190
Sourav Mishra: It should be actually handled dynamically in real chains. But since it's a mocking. It's

764
01:19:08.380 --> 01:19:11.409
Sourav Mishra: set 20 coins as the bulk mining reward.

765
01:19:13.890 --> 01:19:17.180
Sourav Mishra: And then, at the end of everything, just return the balance

766
01:19:19.730 --> 01:19:23.260
Sourav Mishra: so to quickly go through what we have done here is

767
01:19:23.740 --> 01:19:28.369
Sourav Mishra: we are running a loop through the blockchain, through every block.

768
01:19:28.570 --> 01:19:33.179
Sourav Mishra: and for every block. We are running a loop through all the transactions that are present in the block

769
01:19:33.350 --> 01:19:39.879
Sourav Mishra: and checking. If the address is receiver, then we are adding the balance to the address.

770
01:19:40.410 --> 01:19:45.600
Sourav Mishra: and if we if the address is a sender, then you're separating the balance.

771
01:19:45.830 --> 01:19:56.039
Sourav Mishra: and at the end of everything, adding and subtracting in a loop. If the block minus address is same as the address that we have passed here. Then you're adding an additional 20,

772
01:19:56.290 --> 01:19:58.230
Sourav Mishra: because that's the block mining reward.

773
01:19:58.840 --> 01:20:02.130
Gabor Szabo: Shouldn't that that ad also be saturating ad.

774
01:20:04.108 --> 01:20:09.859
Sourav Mishra: There's a a really low chance that it's gonna be an overflow because it's U 64.

775
01:20:10.710 --> 01:20:14.419
Sourav Mishra: But ideally, yes, it should be a saturating area. You're correct.

776
01:20:14.710 --> 01:20:39.099
Gabor Szabo: And and what happens really, if if the saturating sub so what have what will happen? Really if you have? I don't know. 20 in the balance. And and they are, you're subtracting 30. Okay? So the saturating at at the sub will give you the the sub. Now the sub the the from the sender. So it will get you to to 0. Right?

777
01:20:39.660 --> 01:20:42.079
Gabor Szabo: If you yeah. But.

778
01:20:42.080 --> 01:20:46.389
Sourav Mishra: Ideally that should not happen because we have defined a check balance function here.

779
01:20:46.640 --> 01:21:01.570
Sourav Mishra: So any transaction that would render your amount to go into render your balance to go into negative will not be accepted, because for every transaction that you make it is firsthand checked. Whether or not you have enough balance in your account to send a particular amount.

780
01:21:01.990 --> 01:21:02.680
Gabor Szabo: Okay.

781
01:21:03.170 --> 01:21:03.969
Sourav Mishra: So unless you know.

782
01:21:03.970 --> 01:21:10.389
Gabor Szabo: No, no need for the saturating sub right, because because this is already protected here.

783
01:21:11.290 --> 01:21:14.680
Sourav Mishra: Just a follow back just for the sake of writing safe code.

784
01:21:15.110 --> 01:21:25.870
Gabor Szabo: Okay, no, no problem. One of the one of the main, I mean, I came from Perl and Python programming. And this saturating, saturating sub.

785
01:21:26.040 --> 01:21:39.450
Gabor Szabo: They all all look nice. But it's always a question. Okay, okay? Then, what do you do? What do you do? If if how do you handle? I mean, there is okay. So your your code didn't crash. But still you have something you have to deal with.

786
01:21:39.660 --> 01:21:40.295
Gabor Szabo: Right.

787
01:21:40.930 --> 01:21:46.609
Sourav Mishra: This actually is more relevant when you are adding a balance, because balance adding has a chance of being overflowed.

788
01:21:46.950 --> 01:21:51.420
Sourav Mishra: So if the balance is overflowing. Then you can just say it's U. 64 of Max.

789
01:21:54.170 --> 01:21:54.860
Gabor Szabo: Yeah.

790
01:21:55.750 --> 01:21:57.280
Sourav Mishra: I mean, we can put this here.

791
01:22:01.840 --> 01:22:02.600
Gabor Szabo: Okay.

792
01:22:04.400 --> 01:22:06.539
Sourav Mishra: Is there something wrong? Is this not.

793
01:22:10.700 --> 01:22:11.819
Gabor Szabo: What did it say?

794
01:22:13.320 --> 01:22:16.529
Sourav Mishra: It said, need to add it for.

795
01:22:18.230 --> 01:22:23.709
Gabor Szabo: Oh, because it doesn't know what is the type? Okay, it needs to know the exact integer type.

796
01:22:26.440 --> 01:22:31.060
Gabor Szabo: So why is it not knowing what is the the amount is?

797
01:22:32.370 --> 01:22:33.170
Sourav Mishra: Okay, so.

798
01:22:33.230 --> 01:22:35.710
Gabor Szabo: I could say explicitly that it's U. 64.

799
01:22:35.890 --> 01:22:38.110
Sourav Mishra: Yeah, all right.

800
01:22:38.950 --> 01:22:44.700
Sourav Mishra: So we can add that. And when it is saturated. So the I think you're gonna get something like this.

801
01:22:44.930 --> 01:22:49.860
Sourav Mishra: Oh, 2, 64 of Max is what you're gonna get. If it is saturated.

802
01:22:54.830 --> 01:22:56.190
Sourav Mishra: you are on mute.

803
01:22:59.650 --> 01:23:01.080
Sourav Mishra: you are mute, Gabur.

804
01:23:05.080 --> 01:23:18.419
Gabor Szabo: Right? Yeah, I just keep clicking on new. Sorry so noises I might make. You're not not disrupting. So so that that's fine. So I mean, U, 64 is big, so it won't really happen, probably.

805
01:23:18.540 --> 01:23:43.620
Gabor Szabo: but on. But but still, at the end, you we decided that it's saturating. Add, because it's we. If we have 20 left, and and we add 30, we don't want it to overflow right. But but we still we added only 20, and this this subtracted from the other person. 30. So still we still had some

806
01:23:45.370 --> 01:23:54.669
Gabor Szabo: right. So so probably the the the bond that is checking the balance needs to also. So you probably also need to check whether it won't overflow

807
01:23:54.820 --> 01:24:01.839
Gabor Szabo: the other one right? And and you you would need to stop the the transaction

808
01:24:02.450 --> 01:24:04.660
Gabor Szabo: if if it would overflow right.

809
01:24:06.020 --> 01:24:16.229
Sourav Mishra: Yeah, I actually, you should do that, because if it overflows, then the money that you are sending that is kind of wasted. So it's deducted from your account, but it's not deposited in their account.

810
01:24:16.480 --> 01:24:20.709
Gabor Szabo: Yeah, I'm just trying to to think through how these things.

811
01:24:21.530 --> 01:24:37.360
Gabor Szabo: how how you really really use the saturating add and saturating. So not just here in general terms, right? Because it's so. It's fine that it doesn't doesn't crash. But it still generates you. Invalid data or incorrect data, not not invalid. Right?

812
01:24:38.060 --> 01:24:38.630
Sourav Mishra: Yeah.

813
01:24:39.300 --> 01:24:47.230
Gabor Szabo: Yeah, okay, go ahead. Just no. No problem. Okay. You're not really built something that it's going to be in production tomorrow.

814
01:24:47.770 --> 01:24:52.039
Sourav Mishra: Yeah. So it's a good point. So I'm just mentioning it here that add other things, add also

815
01:25:03.500 --> 01:25:04.279
Sourav Mishra: on it.

816
01:25:05.250 --> 01:25:12.870
Sourav Mishra: So we have check balance, and we have good balance. Now let's add these functions. So check balance is already there. But we just need to get the

817
01:25:13.690 --> 01:25:16.719
Sourav Mishra: arguments correct. So I'll just put them here.

818
01:25:22.710 --> 01:25:23.949
Sourav Mishra: This is going to.

819
01:25:23.950 --> 01:25:25.639
Gabor Szabo: Remove the types. Yeah, yeah.

820
01:25:25.640 --> 01:25:31.619
Sourav Mishra: Yeah, this is going to be transaction from. And this is going to be transaction or amount.

821
01:25:32.510 --> 01:25:33.990
Sourav Mishra: Yeah, that checks out.

822
01:25:37.220 --> 01:25:38.810
Sourav Mishra: It doesn't need these yet

823
01:25:44.150 --> 01:25:46.449
Sourav Mishra: interaction a month.

824
01:25:49.170 --> 01:25:50.650
Sourav Mishra: Something is wrong again.

825
01:25:55.280 --> 01:25:56.800
Sourav Mishra: expecting Sdr.

826
01:25:59.220 --> 01:26:09.039
Gabor Szabo: Hmm, like expecting an str and u, 64. And and what is this from? Does it define?

827
01:26:09.210 --> 01:26:11.920
Gabor Szabo: Yeah, yeah, just the Ampersand was missing.

828
01:26:12.110 --> 01:26:12.680
Sourav Mishra: Yeah.

829
01:26:13.680 --> 01:26:22.289
Sourav Mishra: So now, the only undefined thing here is verify. But that's all right, because that's defined under the implementation of transactions, not for blockchain.

830
01:26:22.720 --> 01:26:25.320
Sourav Mishra: So our blockchain implementation is essentially complete.

831
01:26:25.970 --> 01:26:30.080
Sourav Mishra: We wrote all the functions that we need to write, or did I miss something?

832
01:26:30.470 --> 01:26:34.399
Sourav Mishra: Oh, wait! Oh, you missed one function, my bias Commissioner, mind function.

833
01:26:34.980 --> 01:26:37.670
Sourav Mishra: So how do you mind the pending transactions?

834
01:26:39.890 --> 01:26:43.450
Sourav Mishra: So let's call it 9 pending transactions

835
01:26:47.330 --> 01:26:50.409
Sourav Mishra: inverted, so that on. So that on

836
01:26:54.945 --> 01:26:56.700
Sourav Mishra: option of block.

837
01:26:57.980 --> 01:27:01.650
Sourav Mishra: That is these transactions on mine. And now you have this block

838
01:27:02.070 --> 01:27:03.869
Sourav Mishra: with the transition that you got mine.

839
01:27:05.410 --> 01:27:06.336
Sourav Mishra: So if

840
01:27:07.480 --> 01:27:13.890
Sourav Mishra: self dot transaction. Pool is empty, then return none, because there are no transactions to mine in the pool, because it's empty.

841
01:27:14.200 --> 01:27:24.440
Sourav Mishra: But if it is not, then let transactions going to back off and action.

842
01:27:25.920 --> 01:27:32.890
Sourav Mishra: We need to explicitly define this type because we're going to do some iterator and collect stuff. So we need to know what we're exactly collecting into.

843
01:27:33.350 --> 01:27:35.909
Sourav Mishra: Hence the expected definition for work of transaction.

844
01:27:36.420 --> 01:27:39.770
Sourav Mishra: Then we're going to write self of transaction pool.

845
01:27:40.330 --> 01:27:43.240
Sourav Mishra: So from the pool, we are going to train.

846
01:27:46.280 --> 01:27:48.010
Sourav Mishra: and we are going to collect.

847
01:27:50.590 --> 01:27:59.640
Sourav Mishra: So drain is essentially how do I say it? It's a Dk method

848
01:28:06.040 --> 01:28:10.920
Sourav Mishra: question will itself, and minus public key.

849
01:28:20.140 --> 01:28:26.300
Sourav Mishra: So this brings the transaction pool of everything, and then it collects that and wraps it up in a backup.

850
01:28:28.100 --> 01:28:29.550
Sourav Mishra: and then let's mine it.

851
01:28:29.780 --> 01:28:33.610
Sourav Mishra: So you'll say previous block

852
01:28:36.450 --> 01:28:40.059
Sourav Mishra: equals to self dot chain dot last.

853
01:28:42.320 --> 01:28:47.550
Sourav Mishra: and since it returns an option, we can just use the custom map operator rather than 4 should be unwrapping it.

854
01:28:48.900 --> 01:28:52.730
Sourav Mishra: And then, let's say, mute block. This is going to be the current block.

855
01:28:55.120 --> 01:29:02.099
Sourav Mishra: We need this previous block because our current block needs to have the previous blocks has. That's why we queried the current block the previous block.

856
01:29:02.450 --> 01:29:04.909
Sourav Mishra: And now let's create the new block

857
01:29:14.740 --> 01:29:16.129
Sourav Mishra: for readability.

858
01:29:16.860 --> 01:29:22.290
Sourav Mishra: Previous block is just previous previous assets, as previous blocks has.

859
01:29:24.120 --> 01:29:25.710
Sourav Mishra: And we're gonna clone it.

860
01:29:44.750 --> 01:29:47.980
Sourav Mishra: Oh, wait. It's not a struct definition. It's just this, okay

861
01:29:48.230 --> 01:29:53.169
Sourav Mishra: for transactions. We have the transactions already. We just drain them from the transaction pool.

862
01:29:53.560 --> 01:30:01.720
Sourav Mishra: and for the index we are going to do previous block of index last one.

863
01:30:02.510 --> 01:30:07.359
Sourav Mishra: So the index of the new block is basically the one added to the index of the previous block

864
01:30:07.910 --> 01:30:12.600
Sourav Mishra: and the transactions. We are retaining it from the pool and adding it there

865
01:30:14.100 --> 01:30:19.230
Sourav Mishra: and then we are setting the minus address. So block dot minor equals to.

866
01:30:20.420 --> 01:30:22.230
Sourav Mishra: Again we are doing it.

867
01:30:22.830 --> 01:30:25.969
Sourav Mishra: We're converting the public key or public key into a string type.

868
01:30:27.150 --> 01:30:32.590
Sourav Mishra: So minus of dot surrealization.

869
01:30:35.370 --> 01:30:41.370
Sourav Mishra: And then self dot chain, dot, push the block, or

870
01:30:42.630 --> 01:30:46.309
Sourav Mishra: now we are adding the block that we just created here to the chain.

871
01:30:46.970 --> 01:30:51.650
Sourav Mishra: And finally, you're returning some block the block that you just added to the chain you're returning that.

872
01:30:53.090 --> 01:30:58.819
Sourav Mishra: So this completes our mind pending transaction method also. So what we did here 1st was we checked. If the pool was empty.

873
01:30:59.220 --> 01:31:01.489
Sourav Mishra: and if it was empty, then it would turn none.

874
01:31:01.680 --> 01:31:06.300
Sourav Mishra: and next, if it was not empty, then you fetch the transactions that are in the pool.

875
01:31:06.730 --> 01:31:11.800
Sourav Mishra: and then we check. If the previous blog, it exists, and

876
01:31:12.180 --> 01:31:21.030
Sourav Mishra: if it exists, then if it's the previous block, and then you create the current block with the data that you fetch from the previous block and the transactions that you drain from the pool.

877
01:31:21.680 --> 01:31:23.890
Sourav Mishra: and then you add the minus address.

878
01:31:24.720 --> 01:31:27.529
Sourav Mishra: And finally, after doing that, you push the block.

879
01:31:33.570 --> 01:31:34.969
Sourav Mishra: No doubts. Yeah, I hope.

880
01:31:46.760 --> 01:31:48.296
Sourav Mishra: And so let's proceed.

881
01:31:51.410 --> 01:31:54.900
Gabor Szabo: It looks good for so far. What? What I understand.

882
01:31:57.800 --> 01:31:59.469
Sourav Mishra: Okay, then we can proceed.

883
01:32:02.790 --> 01:32:06.130
Sourav Mishra: We are at the last stages of our blockchain.

884
01:32:06.770 --> 01:32:08.899
Sourav Mishra: We only need to define 2 more traits

885
01:32:09.140 --> 01:32:13.190
Sourav Mishra: and 2 more implementations. We need to implement 2 more implementation for 2 more types.

886
01:32:14.260 --> 01:32:17.729
Sourav Mishra: So let's implement transaction the big boy

887
01:32:22.880 --> 01:32:26.580
Sourav Mishra: for transaction. There is or constructor.

888
01:32:26.690 --> 01:32:29.819
Sourav Mishra: As usual, it's gonna return itself.

889
01:32:31.720 --> 01:32:38.220
Sourav Mishra: And it's going to take prompt, which is an Ampersand address.

890
01:32:38.730 --> 01:32:45.279
Sourav Mishra: And it's gonna take out 2, which is the receiver's address in string.

891
01:32:45.580 --> 01:32:47.479
Sourav Mishra: And then it's gonna take the amount

892
01:32:47.960 --> 01:32:51.810
Sourav Mishra: because the other things we can calculate. But these things you need as the user input

893
01:32:54.400 --> 01:32:57.300
Sourav Mishra: so that's there and then return itself.

894
01:32:58.870 --> 01:33:02.610
Sourav Mishra: So from it's from.to string

895
01:33:03.630 --> 01:33:06.809
Sourav Mishra: and to sing to the tuition

896
01:33:08.230 --> 01:33:11.149
Sourav Mishra: amount can be used as is because it's

897
01:33:11.440 --> 01:33:16.430
Sourav Mishra: already in the right format and then, timestamp, we're gonna do some

898
01:33:17.190 --> 01:33:24.860
Sourav Mishra: really weird stuff here. So we are, gonna do system time of now

899
01:33:28.100 --> 01:33:40.480
Sourav Mishra: then, duration of since unix people. So unix people is something we are importing above this

900
01:33:41.370 --> 01:33:44.400
Sourav Mishra: system time and unix report. All come from here.

901
01:33:46.020 --> 01:33:51.009
Sourav Mishra: If you do time, and then don't expect it.

902
01:33:51.900 --> 01:33:55.909
Sourav Mishra: Some random message, let's say invalid time or something.

903
01:33:58.860 --> 01:34:08.890
Sourav Mishra: And finally, as seconds, that gives us our timestamp in the desired format

904
01:34:09.890 --> 01:34:15.269
Sourav Mishra: and signature, since there is none, so far, so signature is going to be just an empty laptop.

905
01:34:17.030 --> 01:34:21.830
Sourav Mishra: and there we have a new translation object return using the new function.

906
01:34:25.070 --> 01:34:28.980
Sourav Mishra: Right? So once this is done. The next thing to implement is

907
01:34:30.570 --> 01:34:39.460
Sourav Mishra: the message to sign, so we'll call it good signing message.

908
01:34:41.030 --> 01:34:47.360
Sourav Mishra: And so the message is going to be a string.

909
01:34:52.970 --> 01:35:03.829
Sourav Mishra: So when signing anything you 1st hash them like. If you have a raw text message, you 1st hash the message and then you sign the hash of that message. You don't sign the message directly.

910
01:35:04.220 --> 01:35:07.970
Sourav Mishra: So like earlier, we are going to define a hazard.

911
01:35:09.410 --> 01:35:11.400
Sourav Mishra: the cycle for the 6th of noon.

912
01:35:12.300 --> 01:35:15.470
Sourav Mishra: and then we're gonna update the asset

913
01:35:18.610 --> 01:35:22.650
Sourav Mishra: before, and we're gonna use survey to string.

914
01:35:27.860 --> 01:35:34.140
Sourav Mishra: So self dot from self.to self dot amount server timestamp.

915
01:35:34.580 --> 01:35:35.990
Sourav Mishra: I need this

916
01:35:39.750 --> 01:35:42.870
Sourav Mishra: Lord untap my passwords.

917
01:35:44.960 --> 01:35:48.710
Sourav Mishra: Did I do it in the wrong place? Yeah, I did it in the wrong place.

918
01:35:50.920 --> 01:35:53.670
Sourav Mishra: Look at our credit card.

919
01:35:55.100 --> 01:35:59.789
Sourav Mishra: So finally return the hassle. But finally, whatever you finalize, return that.

920
01:36:02.760 --> 01:36:05.169
Sourav Mishra: How do you return? That you return that as a Vec.

921
01:36:08.870 --> 01:36:16.660
Sourav Mishra: I guess I messed up the return types. Here signature is a vec, so no, that's right.

922
01:36:19.340 --> 01:36:21.179
Sourav Mishra: Have it as a web of your

923
01:36:23.120 --> 01:36:26.139
Sourav Mishra: yeah. Now we're gonna sign in this wake-off, Eva.

924
01:36:27.670 --> 01:36:32.099
Sourav Mishra: So what we did here was we generated a message for us to sign.

925
01:36:32.380 --> 01:36:36.939
Sourav Mishra: And how did you do that? We created a string message

926
01:36:37.610 --> 01:36:48.819
Sourav Mishra: using certain to string, and we converted that to bytes, and we use those bytes in the hash hash them. And finally, we are returning the hash bytes as vec.

927
01:36:49.170 --> 01:36:53.169
Sourav Mishra: so basically the resultant back that we are returning. Here

928
01:36:53.310 --> 01:36:58.090
Sourav Mishra: is the final thing that we are going to sign. We're going to put our signature on.

929
01:36:59.560 --> 01:37:05.660
Sourav Mishra: So that's there. And finally, with the function verify that has been showing us, as Madam

930
01:37:08.570 --> 01:37:12.899
Sourav Mishra: Verify would return a pool, that is, if the verification is successful or not.

931
01:37:14.300 --> 01:37:22.790
Sourav Mishra: So to verify, we're gonna check. If the transaction is a faucet transaction, if it is, then just ignore it.

932
01:37:23.020 --> 01:37:26.049
Sourav Mishra: It's true always. We don't need to further verify anything

933
01:37:26.900 --> 01:37:31.659
Sourav Mishra: element. If it is not, then create an scp object.

934
01:37:33.580 --> 01:37:37.409
Sourav Mishra: So sccp 2, 56 k. 1 is the elective curve

935
01:37:43.330 --> 01:37:48.900
Sourav Mishra: that we are going to use for generating.

936
01:37:50.820 --> 01:37:51.750
Sourav Mishra: Keep it.

937
01:37:52.270 --> 01:37:53.470
Sourav Mishra: And same.

938
01:37:55.340 --> 01:37:56.410
Sourav Mishra: So

939
01:37:56.780 --> 01:38:10.900
Sourav Mishra: we are using this particular relative curve to generate our key pair. And we are using an algorithm called Ecdsa, that is based on this curve that is responsible for signing. So Ecdsa is a signing algorithm

940
01:38:12.710 --> 01:38:14.530
Sourav Mishra: that is based on the Ccp.

941
01:38:16.450 --> 01:38:20.569
Sourav Mishra: So we created an instance of this Scc type Sccp new.

942
01:38:21.620 --> 01:38:30.509
Sourav Mishra: And then we're gonna verify whether or not the public key that was sent to us

943
01:38:32.970 --> 01:38:34.209
Sourav Mishra: is valid or not.

944
01:38:34.330 --> 01:38:39.610
Sourav Mishra: So we are going to match it. X decode

945
01:38:43.670 --> 01:38:48.830
Sourav Mishra: self dot from stuff

946
01:38:54.190 --> 01:38:58.359
Sourav Mishra: of bytes like. If we could successfully decode that.

947
01:38:58.620 --> 01:39:02.309
Sourav Mishra: then it's cool. Okay of bytes and return the bytes.

948
01:39:07.740 --> 01:39:11.750
Sourav Mishra: There is some error. Then return the error

949
01:39:16.300 --> 01:39:19.120
Sourav Mishra: exactly the time. Just use the one, macro.

950
01:39:21.750 --> 01:39:27.630
Sourav Mishra: I don't know recording public key and

951
01:39:29.210 --> 01:39:31.370
Sourav Mishra: use the error that you just got there.

952
01:39:33.500 --> 01:39:35.209
Sourav Mishra: Return falls, of course.

953
01:39:36.040 --> 01:39:36.910
Sourav Mishra: Show it off.

954
01:39:40.510 --> 01:39:41.620
Sourav Mishra: No, that's done.

955
01:39:43.410 --> 01:39:46.299
Sourav Mishra: But semicolon. Yeah, you're good to go.

956
01:39:47.190 --> 01:39:48.400
Sourav Mishra: Then. Next

957
01:39:51.880 --> 01:39:56.499
Sourav Mishra: we are going to check the validity of the public key from the public key bytes that we just received

958
01:39:57.690 --> 01:40:00.620
Sourav Mishra: so publicly equals to match.

959
01:40:02.750 --> 01:40:05.589
Sourav Mishra: There's just a lot of validation here, because these are

960
01:40:06.470 --> 01:40:14.019
Sourav Mishra: really sensitive affairs. So you're sending money. So if it doesn't go to the right place, or if it is not from the right place, then it can go and land up anywhere

961
01:40:14.150 --> 01:40:19.570
Sourav Mishra: you are, gonna lose real money. So all these checks are to ensure that you don't do. You don't lose your money.

962
01:40:24.000 --> 01:40:25.360
Sourav Mishra: Public works.

963
01:40:27.850 --> 01:40:32.950
Sourav Mishra: Okay? So if it's okay, then return the key to me.

964
01:40:37.690 --> 01:40:42.359
Sourav Mishra: If it is an error again, log a warning

965
01:40:58.480 --> 01:40:59.150
Sourav Mishra: for you.

966
01:41:02.990 --> 01:41:04.139
Sourav Mishra: the 10 points.

967
01:41:08.260 --> 01:41:18.990
Sourav Mishra: Yeah. Public key validation is done here. So at least after this point, we are sure that the public key that is sending this transaction is actually a valid public key.

968
01:41:24.320 --> 01:41:26.710
Sourav Mishra: Now let's proceed to signing the transaction.

969
01:41:27.630 --> 01:41:30.500
Sourav Mishra: or at least verifying the sign of the transaction

970
01:41:33.810 --> 01:41:36.279
Sourav Mishra: signature equals to

971
01:41:44.780 --> 01:41:47.570
Sourav Mishra: Cdsa. This is the algorithm that I talked about.

972
01:41:50.370 --> 01:41:51.100
Gabor Szabo: Sorry.

973
01:41:52.470 --> 01:41:53.140
Sourav Mishra: Sorry.

974
01:41:53.400 --> 01:41:55.219
Gabor Szabo: Yeah, you did you ask something?

975
01:41:55.510 --> 01:42:00.720
Sourav Mishra: Oh, no, no, no. I'm saying that this issue here is the algorithm that I talked about. This thing's algorithm here

976
01:42:01.110 --> 01:42:02.150
Sourav Mishra: and signing.

977
01:42:02.400 --> 01:42:05.429
Sourav Mishra: So this is the algorithm that is used for signatures.

978
01:42:05.810 --> 01:42:06.540
Gabor Szabo: Okay.

979
01:42:07.910 --> 01:42:09.939
Sourav Mishra: From compact

980
01:42:13.950 --> 01:42:16.080
Sourav Mishra: self dot signature.

981
01:42:21.150 --> 01:42:28.669
Sourav Mishra: So what I am doing here is basically converting the signature from whatever type I had defined

982
01:42:28.780 --> 01:42:33.679
Sourav Mishra: to the actual sequencer type that is defined by Ecpc for further processing.

983
01:42:37.170 --> 01:42:44.859
Sourav Mishra: Then I'm gonna create the message again, because it's a good practice to create the signing message on your end also before verifying

984
01:42:46.200 --> 01:42:49.620
Sourav Mishra: the signature, and then a flare.

985
01:42:51.710 --> 01:42:52.620
Sourav Mishra: Sure

986
01:43:13.090 --> 01:43:15.729
Sourav Mishra: data probability is kind of really smart like. It

987
01:43:16.470 --> 01:43:21.089
Sourav Mishra: suggests almost the exact correct thing almost all the time.

988
01:43:28.250 --> 01:43:33.590
Sourav Mishra: Then I'm gonna return the verification result after signature.

989
01:43:41.080 --> 01:43:43.699
Sourav Mishra: I'll take the message.

990
01:43:43.920 --> 01:43:48.259
Sourav Mishra: I think it will be confused with necessity. So let's call this MSG,

991
01:43:51.310 --> 01:43:55.039
Sourav Mishra: and so so do you have done your signature

992
01:43:56.580 --> 01:44:00.910
Sourav Mishra: and the public key. And finally, if it is okay and put

993
01:44:02.780 --> 01:44:05.919
Sourav Mishra: rationally, return false. If none of these were

994
01:44:07.080 --> 01:44:14.329
Sourav Mishra: then it's false by default, so the transition verification is not a success solved by default.

995
01:44:17.470 --> 01:44:22.059
Sourav Mishra: Alright, the transaction implementation is also done, so is there any doubts? Are there any doubts.

996
01:44:26.580 --> 01:44:28.040
Gabor Szabo: Not from me.

997
01:44:30.180 --> 01:44:35.460
Sourav Mishra: I'll just go through the verify function again. So you are verifying. If it's a posture transaction, then you're skipping it.

998
01:44:35.670 --> 01:44:41.230
Sourav Mishra: then you are decoding the public key bytes, and you are checking if the public key is valid or not.

999
01:44:42.840 --> 01:44:45.230
Sourav Mishra: and then, finally, you are checking the signature.

1000
01:44:45.410 --> 01:44:47.640
Sourav Mishra: So you are creating a sequencer type.

1001
01:44:47.770 --> 01:44:50.060
Sourav Mishra: And then you are also getting the message.

1002
01:44:50.820 --> 01:44:58.400
Sourav Mishra: And then you are constructing a message type that is defined by Sccp. Using your string message that you just constructed.

1003
01:45:00.590 --> 01:45:07.819
Sourav Mishra: and once you are done with constructing the signature and the message, you are finally verifying that against the public key.

1004
01:45:09.800 --> 01:45:17.090
Sourav Mishra: So you verify the sign which you sign something using your private key, but you verify that using your public key.

1005
01:45:19.760 --> 01:45:23.120
Sourav Mishra: So if this verification is successful, it means you are the one who signed it.

1006
01:45:24.440 --> 01:45:25.250
Gabor Szabo: Okay.

1007
01:45:28.890 --> 01:45:33.612
Sourav Mishra: And finally, the last thing that we need to implement is our blockchain server.

1008
01:45:35.560 --> 01:45:40.760
Sourav Mishra: So we're gonna create a blocks and server type. I think we created this earlier? Not we're not creating a type.

1009
01:45:41.400 --> 01:45:47.959
Sourav Mishra: Yeah, we have the type already. Blockchain server that takes a blockchain. That is, our communities of a blockchain and blockchain. Is this

1010
01:46:01.080 --> 01:46:02.990
Sourav Mishra: so? Both function. New

1011
01:46:42.850 --> 01:46:45.620
Sourav Mishra: constructor is also done.

1012
01:46:47.120 --> 01:46:50.839
Sourav Mishra: Finally, the last thing that is left to implement is the

1013
01:46:52.620 --> 01:46:57.550
Sourav Mishra: service. This particular blockchain service like the Rpc. Service for our blockchain server.

1014
01:46:58.280 --> 01:47:02.729
Sourav Mishra: So our blockchain server needs to expose Rpc. Endpoints for our client to connect.

1015
01:47:03.010 --> 01:47:06.990
Sourav Mishra: and the Rpc. Service is defined in this protobuf file.

1016
01:47:07.930 --> 01:47:19.410
Sourav Mishra: So we need to implement this particular thing, this blockchain service. It's not a trait yet. It's just a service according to the protocol file. But since we are using tonic to compile, we already have a trait with the same name

1017
01:47:19.580 --> 01:47:25.729
Sourav Mishra: that has these same methods, a method called summit transaction method called, Get balance and method called request faucet.

1018
01:47:26.100 --> 01:47:30.630
Sourav Mishra: So you're just going to implement that trade phone also

1019
01:47:48.700 --> 01:47:55.040
Sourav Mishra: is going to be an Asynct tape by tonic. So a required macro

1020
01:48:09.250 --> 01:48:10.119
Sourav Mishra: and she

1021
01:48:13.640 --> 01:48:18.869
Sourav Mishra: connection add transaction. I'll just open this side by side.

1022
01:48:24.690 --> 01:48:26.560
Sourav Mishra: and I'll zoom out just a bit.

1023
01:48:28.590 --> 01:48:34.170
Sourav Mishra: So the methods we have on us are submit transaction, get balance and request faucet.

1024
01:48:34.270 --> 01:48:36.750
Sourav Mishra: The 1st method is going to be submit transaction.

1025
01:48:42.280 --> 01:48:45.060
Sourav Mishra: and it's going to take a self.

1026
01:48:45.250 --> 01:48:46.680
Sourav Mishra: That's the 1st parameter.

1027
01:48:47.480 --> 01:48:58.469
Sourav Mishra: And then there's gonna be requested, which is of type the best and go to a transaction.

1028
01:48:59.120 --> 01:49:02.479
Sourav Mishra: So these are the types that we have implemented.

1029
01:49:02.670 --> 01:49:05.540
Sourav Mishra: we have imported basically from above.

1030
01:49:05.970 --> 01:49:10.059
Sourav Mishra: So we have a request response and status from panic.

1031
01:49:10.360 --> 01:49:13.360
Sourav Mishra: And we have prototransaction from blockchain.

1032
01:49:14.610 --> 01:49:16.749
Sourav Mishra: which is the module we have just defined above

1033
01:49:24.840 --> 01:49:34.650
Sourav Mishra: some transaction, and it returns reserve of response.

1034
01:49:41.300 --> 01:49:48.560
Sourav Mishra: We got some a response and status.

1035
01:49:58.640 --> 01:49:59.726
Sourav Mishra: So let

1036
01:50:03.720 --> 01:50:09.620
Sourav Mishra: transaction equals to request log into enough.

1037
01:50:10.380 --> 01:50:16.520
Sourav Mishra: So this into inner, is again something that is given to us by by tonic.

1038
01:50:16.670 --> 01:50:27.639
Sourav Mishra: I have not defined this anywhere. So this into Nr, basically gives us the message. So if I don't put into enough if you can, you can see the type. The type is of request.

1039
01:50:27.980 --> 01:50:36.040
Sourav Mishra: and then the generic type. T. So in our case, the generic type T is transaction. So it has 3 things, 2 of which I don't really care about.

1040
01:50:36.420 --> 01:50:43.039
Sourav Mishra: But you can see, all of these fields are private fields, so there is no way for me to directly access the message. Field

1041
01:50:43.430 --> 01:50:45.080
Sourav Mishra: asses because it's private.

1042
01:50:46.420 --> 01:50:52.740
Sourav Mishra: So there is a function. There's a method exposed that called that's called in trainer. So I'm gonna use that to get the message.

1043
01:50:54.680 --> 01:51:03.359
Sourav Mishra: And once I have gotten the message, I'm gonna say, like transaction equals to.

1044
01:51:04.670 --> 01:51:07.150
Sourav Mishra: I'm just gonna construct it into my own type.

1045
01:51:09.770 --> 01:51:12.369
Sourav Mishra: So from it's basically

1046
01:51:12.540 --> 01:51:19.099
Sourav Mishra: transaction.com 2 is basically transaction dot 2 amount is transaction amount. Which is that correct?

1047
01:51:19.370 --> 01:51:23.740
Sourav Mishra: And then this timestamp, I'm gonna use the one that I get from there

1048
01:51:24.160 --> 01:51:27.810
Sourav Mishra: just to ensure that these 2 things are same.

1049
01:51:28.350 --> 01:51:39.840
Sourav Mishra: because there might be a certain delay between that timestamp and the timestamp when I calculate it right now, and if they are different. Then the hash will be different, and the sign won't be verified properly.

1050
01:51:40.060 --> 01:51:44.319
Sourav Mishra: That's why I'm just gonna use the timestamp from this, the one that I'm receiving

1051
01:51:47.700 --> 01:51:48.430
Sourav Mishra: alright.

1052
01:51:49.650 --> 01:51:52.200
Sourav Mishra: Everything looks fine. I have my transition object now.

1053
01:51:53.050 --> 01:52:01.780
Sourav Mishra: So I'm gonna say, let new chain equals to self dot blockchain.

1054
01:52:02.800 --> 01:52:07.365
Sourav Mishra: and I'm gonna put a mutex lock on this, so that further, when I'm like

1055
01:52:08.260 --> 01:52:11.380
Sourav Mishra: I'm going to modify the chain now by adding a template. So

1056
01:52:12.260 --> 01:52:17.660
Sourav Mishra: I am putting a lock here so that only I can modify it like this thread can modify it, not anyone else.

1057
01:52:18.680 --> 01:52:22.430
Sourav Mishra: Then I'm gonna share alert success.

1058
01:52:24.250 --> 01:52:29.750
Sourav Mishra: It was to change or add transaction. And then in the transaction that I just created above

1059
01:52:30.170 --> 01:52:33.709
Sourav Mishra: don't necessarily need to clone it, because I'm not going to use it anywhere downstairs.

1060
01:52:34.860 --> 01:52:35.660
Sourav Mishra: So yeah.

1061
01:52:39.280 --> 01:52:51.510
Sourav Mishra: so far, I created a mutex log. And I added my transaction to the gym, and I got a success result that could either be true or false, based on the success of me. Get adding the transaction to the gym.

1062
01:52:52.970 --> 01:52:56.150
Sourav Mishra: And finally, I'll create the return type. That is

1063
01:53:00.680 --> 01:53:06.179
Sourav Mishra: again, this response and everything just to remind you it comes from tonic, not something that we have defined by ourselves.

1064
01:53:10.080 --> 01:53:14.829
Sourav Mishra: Success is this value, and the last thing is message.

1065
01:53:25.740 --> 01:53:27.410
Sourav Mishra: We can close this for now.

1066
01:53:29.440 --> 01:53:33.620
Sourav Mishra: so message is based on the success. So if the transaction is successful, then it's

1067
01:53:33.810 --> 01:53:37.920
Sourav Mishra: added to the pool, or will it say transaction accepted.

1068
01:53:38.190 --> 01:53:40.840
Sourav Mishra: and if it is not, then we can say transaction failed.

1069
01:53:42.540 --> 01:53:46.019
Sourav Mishra: So that is our 1st function, the submit transaction function.

1070
01:53:48.240 --> 01:53:53.410
Sourav Mishra: The next layer is get balance. This is a really simple function.

1071
01:53:54.710 --> 01:53:58.989
Sourav Mishra: Okay, balance. Just query the chain and get the balance.

1072
01:54:00.700 --> 01:54:05.290
Sourav Mishra: And then this will return something like above. So let's copy this for simplicity.

1073
01:54:06.860 --> 01:54:07.770
Sourav Mishra: Okay.

1074
01:54:11.550 --> 01:54:18.870
Sourav Mishra: so forget balance. The return type is not transaction response, whether it is balances once.

1075
01:54:23.260 --> 01:54:26.499
Sourav Mishra: And this is also pretty similar. So let's copy it.

1076
01:54:28.240 --> 01:54:33.800
Sourav Mishra: It takes an instance to sell, and the request is not this but balance request?

1077
01:54:37.799 --> 01:54:42.920
Sourav Mishra: Where am I getting these things? The balance request, balance, response, and everything. I'm just getting this from here.

1078
01:54:43.520 --> 01:54:48.480
Sourav Mishra: The protocol file. So balance request is already there. Balance response is also already there.

1079
01:54:50.600 --> 01:54:53.280
Sourav Mishra: Transaction response. As you can see, it's already there.

1080
01:54:55.540 --> 01:55:03.110
Sourav Mishra: And I'm just type aliasing this as photo transaction, because this is also transaction here.

1081
01:55:03.340 --> 01:55:09.060
Sourav Mishra: And my local type transaction is also called transaction, just to avoid confusion. I'm calling it floater transaction

1082
01:55:09.970 --> 01:55:15.979
Sourav Mishra: and proto transaction comes from here so you can see transaction as proto transaction. I'm importing it as that

1083
01:55:20.230 --> 01:55:21.880
Sourav Mishra: right? Oh, no.

1084
01:55:22.640 --> 01:55:28.320
Sourav Mishra: Try this function. Let address Equals to

1085
01:55:30.990 --> 01:55:35.689
Sourav Mishra: request Dot into enough, because we need the message, the inner message, that's right.

1086
01:55:37.090 --> 01:55:39.080
Sourav Mishra: And then, finally, we need the address

1087
01:55:43.160 --> 01:55:50.910
Sourav Mishra: which chain equals to self, dot blockchain, dot low dot of it

1088
01:55:56.300 --> 01:56:04.410
Sourav Mishra: same as before, because we need a mutex lock and let balance

1089
01:56:05.300 --> 01:56:10.989
Sourav Mishra: equals to chain dot get balance. This get balance is the method that we just defined above

1090
01:56:11.160 --> 01:56:15.170
Sourav Mishra: for our login implementation, and it takes an address.

1091
01:56:16.480 --> 01:56:19.689
Sourav Mishra: So I got the balance now, and I'll just construct the response.

1092
01:56:22.030 --> 01:56:27.229
Sourav Mishra: And this is so you can see the return type is satisfied. Date expects

1093
01:56:27.648 --> 01:56:30.600
Sourav Mishra: response type in the okay. So that's what I gave him.

1094
01:56:33.490 --> 01:56:36.660
Sourav Mishra: And this balance is the balance that I'm fetching from the chain.

1095
01:56:39.580 --> 01:56:47.140
Sourav Mishra: There is one last method that's the last and the last thing in the project itself. It's the request faucet method.

1096
01:56:48.280 --> 01:56:51.340
Sourav Mishra: It's going to be a little long. But bear with me

1097
01:56:53.360 --> 01:56:57.380
Sourav Mishra: so, as usual, let's just copy this. It's easier that way.

1098
01:56:58.180 --> 01:57:03.810
Sourav Mishra: and then put it here, and let's call this request faucet

1099
01:57:05.884 --> 01:57:09.670
Sourav Mishra: before diving into this. Do you have any doubts.

1100
01:57:14.040 --> 01:57:15.260
Gabor Szabo: What is this?

1101
01:57:15.480 --> 01:57:17.460
Gabor Szabo: Have a more general question later on.

1102
01:57:19.467 --> 01:57:20.389
Sourav Mishra: Sorry come again.

1103
01:57:20.390 --> 01:57:24.160
Gabor Szabo: I'll have another question, but once once you finish it.

1104
01:57:25.180 --> 01:57:26.480
Sourav Mishra: Okay? Sure.

1105
01:57:34.730 --> 01:57:40.519
Sourav Mishra: So this is going to be of type faucet request. And this is going to be of type. Faucet response.

1106
01:57:41.410 --> 01:57:48.930
Sourav Mishra: Again, faucet request and faucet response is something we're getting from the profile, request and response.

1107
01:57:51.670 --> 01:57:56.429
Sourav Mishra: And then status is something we have from tonic itself. We're importing that from tonic itself

1108
01:57:59.510 --> 01:58:00.420
Sourav Mishra: all right.

1109
01:58:01.750 --> 01:58:03.349
Sourav Mishra: Where did that cool yep.

1110
01:58:05.730 --> 01:58:13.079
Sourav Mishra: So like address again, equals to request or entering network address?

1111
01:58:13.220 --> 01:58:19.409
Sourav Mishra: Because we need the address, because this is the address to which we are gonna send the coins after the faucet transaction.

1112
01:58:21.180 --> 01:58:25.419
Sourav Mishra: and then we'll create a transaction

1113
01:58:26.820 --> 01:58:29.889
Sourav Mishra: like we did here. So let's just copy and paste. This.

1114
01:58:35.220 --> 01:58:39.470
Sourav Mishra: so from address is going to be faucet address

1115
01:58:40.290 --> 01:58:43.809
Sourav Mishra: to address is going to be the address that we just received.

1116
01:58:44.110 --> 01:58:47.160
Sourav Mishra: or the address that we just got from the message.

1117
01:58:48.810 --> 01:58:55.069
Sourav Mishra: They want to do it and amount is something that we can

1118
01:58:56.420 --> 01:58:59.848
Sourav Mishra: defined by ourselves. So this is hard coded. Let's say, it's

1119
01:59:00.260 --> 01:59:03.279
Sourav Mishra: 1,400 coins, any arbitrary amount.

1120
01:59:04.010 --> 01:59:11.189
Sourav Mishra: And, Pakistan, anybody, we're gonna use the same thing as this.

1121
01:59:12.590 --> 01:59:13.320
Sourav Mishra: No

1122
01:59:23.760 --> 01:59:29.170
Sourav Mishra: signature is going to be empty because it's for your transaction.

1123
01:59:33.190 --> 01:59:33.860
Sourav Mishra: No.

1124
01:59:37.040 --> 01:59:39.179
Sourav Mishra: discharge the rules, and

1125
01:59:43.650 --> 01:59:47.249
Sourav Mishra: yep, we also constructed a transaction here. The 1st transaction.

1126
01:59:47.410 --> 01:59:53.240
Sourav Mishra: Now let's add it to the chain to the transaction pool whoops

1127
01:59:54.960 --> 02:00:00.079
Sourav Mishra: join equals to copy and paste this.

1128
02:00:00.900 --> 02:00:03.360
Sourav Mishra: then copy and paste this whole thing starting from there

1129
02:00:13.390 --> 02:00:18.620
Sourav Mishra: and then we're going to mine a block the transaction

1130
02:00:22.040 --> 02:00:26.830
Sourav Mishra: faucet transactions are mined immediately, at least in our journey.

1131
02:00:30.290 --> 02:00:40.970
Sourav Mishra: So we're gonna create another scp object like we did earlier, then create a new keypad.

1132
02:00:43.340 --> 02:00:45.159
Sourav Mishra: Let's call it faucet key.

1133
02:01:06.920 --> 02:01:12.220
Sourav Mishra: So ideally, this should actually come from your env files, because this will not change.

1134
02:01:12.480 --> 02:01:15.860
Sourav Mishra: But every time you request for 4 C transaction. This kind of changes.

1135
02:01:27.990 --> 02:01:28.989
Sourav Mishra: I'll block it.

1136
02:01:31.380 --> 02:01:34.750
Sourav Mishra: 10 dot, 9 pending transactions.

1137
02:01:36.040 --> 02:01:41.130
Sourav Mishra: Use the faucet key that we just created.

1138
02:01:45.460 --> 02:01:50.820
Sourav Mishra: If it is successful, then add a log that the faucet transaction was successful, and it was mine successful

1139
02:01:52.010 --> 02:01:53.010
Sourav Mishra: doing good.

1140
02:01:56.980 --> 02:02:05.989
Sourav Mishra: Poster blog no cash, only a good price.

1141
02:02:08.070 --> 02:02:10.310
Sourav Mishra: It's blog, dot hash?

1142
02:02:14.190 --> 02:02:15.890
Sourav Mishra: Then send a response.

1143
02:02:59.190 --> 02:03:02.770
Sourav Mishra: So this is covered for the success part.

1144
02:03:03.250 --> 02:03:07.279
Sourav Mishra: What if this fails? What if we're not able to mine the transaction? What if we're not able to get a block.

1145
02:03:07.550 --> 02:03:11.009
Sourav Mishra: So for that, I'm just gonna create an error response

1146
02:03:17.990 --> 02:03:19.780
Sourav Mishra: supposed to copy this thing.

1147
02:03:24.660 --> 02:03:33.020
Sourav Mishra: There is no faucet amount in our case. So let's say, start code 1,400 always works. Yeah.

1148
02:03:36.790 --> 02:03:42.639
Sourav Mishra: mount in 0 because it's already failed.

1149
02:03:43.270 --> 02:03:49.470
Sourav Mishra: So let's surface is false among this 0, let's say is, is.

1150
02:03:54.460 --> 02:03:56.830
Sourav Mishra: I need to process once it request.

1151
02:03:59.440 --> 02:04:00.280
Sourav Mishra: That's it.

1152
02:04:01.900 --> 02:04:05.529
Sourav Mishra: So we are essentially done with the chain. Let me just check if

1153
02:04:12.550 --> 02:04:13.210
Sourav Mishra: hmm!

1154
02:04:13.750 --> 02:04:24.619
Sourav Mishra: I think we had a missing implementation. Someone difficulty is, where did they unload?

1155
02:04:31.820 --> 02:04:32.570
Sourav Mishra: Room?

1156
02:04:33.330 --> 02:04:36.720
Sourav Mishra: Probably missed something somewhere. Yeah, we missed these things

1157
02:04:37.660 --> 02:04:40.640
Sourav Mishra: and the start method. Obviously, those things you may have missed.

1158
02:04:41.040 --> 02:04:42.729
Sourav Mishra: So let's implement them quickly

1159
02:04:46.710 --> 02:04:53.139
Sourav Mishra: supplemented for consensus. And okay, the start method. So

1160
02:04:55.120 --> 02:04:57.320
Sourav Mishra: how do we start it? We're going to spawn a new thread

1161
02:04:57.850 --> 02:05:08.549
Sourav Mishra: because one thread is responsible. The main thread is responsible for running the Glp server is responsible for mining.

1162
02:05:12.010 --> 02:05:14.820
Sourav Mishra: So we sponsored.

1163
02:05:20.320 --> 02:05:24.860
Sourav Mishra: It's getting annoying. Yeah, and then assume move

1164
02:05:25.910 --> 02:05:29.950
Sourav Mishra: again. It's all syntax for asynchronous coding and rust

1165
02:05:37.710 --> 02:05:45.050
Sourav Mishra: and Sccp equals to Ccp. 2, 56 a 1,

1166
02:05:45.860 --> 02:05:48.190
Sourav Mishra: and for you on your Ccp object?

1167
02:05:49.590 --> 02:05:51.820
Sourav Mishra: Then you're gonna generate a key.

1168
02:05:53.790 --> 02:05:55.830
Sourav Mishra: It's going to the public key.

1169
02:05:57.520 --> 02:06:04.530
Sourav Mishra: It goes to the net repair. Let me start this, then I'm gonna log.

1170
02:06:05.360 --> 02:06:06.780
Sourav Mishra: The manage address.

1171
02:06:10.650 --> 02:06:11.820
Sourav Mishra: My name?

1172
02:06:25.140 --> 02:06:28.290
Sourav Mishra: Probably not super nice

1173
02:06:32.420 --> 02:06:35.520
Sourav Mishra: something else playing with this. I don't need this.

1174
02:06:47.090 --> 02:06:49.259
Sourav Mishra: Next, we're gonna loop.

1175
02:06:50.200 --> 02:06:56.780
Sourav Mishra: because it's a continuous process checking for transactions in the transaction pool. They keep on checking, and if you find transactions, then you mine them.

1176
02:06:58.860 --> 02:07:01.030
Sourav Mishra: So the loop. Let's create a scope.

1177
02:07:01.460 --> 02:07:03.699
Sourav Mishra: Let's let mute Jane

1178
02:07:07.130 --> 02:07:10.520
Sourav Mishra: goes to acquire a mutex lock.

1179
02:07:18.530 --> 02:07:24.599
Sourav Mishra: If chain dot transaction pool has a set number of transactions only, then start mining.

1180
02:07:25.000 --> 02:07:33.850
Sourav Mishra: So for testing purposes we can set the limit to one as in

1181
02:07:34.860 --> 02:07:43.700
Sourav Mishra: if there is one single transaction in the transaction pool, then lately.

1182
02:08:14.880 --> 02:08:19.289
Sourav Mishra: and then once you mind a walk, sleep for some time 5 seconds, for example.

1183
02:08:22.610 --> 02:08:24.810
Sourav Mishra: who's we are done now?

1184
02:08:25.500 --> 02:08:30.420
Sourav Mishra: So just to go over this method again, it spawns a thread. It creates a new minor key.

1185
02:08:30.780 --> 02:08:35.819
Sourav Mishra: and it starts mining. It starts listening to the transaction pool in a loop.

1186
02:08:35.980 --> 02:08:42.830
Sourav Mishra: and when the number of transactions in the pool is greater than one, or you can say, if it is at least one.

1187
02:08:42.930 --> 02:08:48.329
Sourav Mishra: then the minor it drains all the transactions from the pool and starts mining them.

1188
02:08:48.730 --> 02:08:52.539
Sourav Mishra: If you want it to be strictly greater than one, then you can just remove the equal to, and it will work.

1189
02:09:00.790 --> 02:09:06.900
Sourav Mishra: Yeah, this part is unimplemented. Okay, number. Now, this needs some login

1190
02:09:09.590 --> 02:09:16.220
Sourav Mishra: transaction. Verification failed. This also needs some login information, valid information. Correct?

1191
02:09:16.950 --> 02:09:19.170
Sourav Mishra: So yeah, our blockchain code is now ready.

1192
02:09:20.440 --> 02:09:21.960
Sourav Mishra: Anyone has any doubts.

1193
02:09:29.690 --> 02:09:30.700
Gabor Szabo: But for me.

1194
02:09:33.070 --> 02:09:35.120
Sourav Mishra: You had a question somewhere else, right?

1195
02:09:36.970 --> 02:09:44.109
Gabor Szabo: Yeah, but it's it's like a more more generic question. So we implement you implemented that a blockchain

1196
02:09:44.360 --> 02:09:47.040
Gabor Szabo: system for for sending money.

1197
02:09:47.260 --> 02:09:48.830
Gabor Szabo: And and

1198
02:09:49.390 --> 02:09:57.181
Gabor Szabo: and I was wondering, Okay, what? Why do? Why do we need another one? Why do we need a platform to implement these

1199
02:10:00.030 --> 02:10:07.890
Gabor Szabo: the next? Another way to to to send money. If you have already a number of these blockchains for

1200
02:10:08.020 --> 02:10:13.799
Gabor Szabo: for sending money, so would would a company just implement one, and then then use it, for I don't know

1201
02:10:14.670 --> 02:10:16.059
Gabor Szabo: their clients.

1202
02:10:17.936 --> 02:10:18.770
Sourav Mishra: Send it.

1203
02:10:19.197 --> 02:10:26.040
Gabor Szabo: Let's say a bank will will open one for themselves. What was the idea behind it?

1204
02:10:28.170 --> 02:10:34.473
Sourav Mishra: Yeah. So blockchains are not exactly used only for sending and receiving money. There are a lot of other implications also, like

1205
02:10:35.427 --> 02:10:39.400
Sourav Mishra: Real world, asset, exchange and everything. Name a few.

1206
02:10:40.040 --> 02:10:46.729
Sourav Mishra: But why should you create a new blockchain? You should create a new blockchain if you are creating something unique.

1207
02:10:47.140 --> 02:10:55.379
Sourav Mishra: This is just a general explanation of how a blockchain functions in general. This is not actually a production ready, unique blockchain that people are going to use tomorrow like you said.

1208
02:10:55.700 --> 02:10:57.240
Gabor Szabo: Right, yeah, yeah, yeah.

1209
02:10:57.240 --> 02:11:04.430
Sourav Mishra: Yeah, this was just created to explain people how the internals of a blockchain work, how the components of a blockchain can be coded using rushed.

1210
02:11:04.730 --> 02:11:09.129
Sourav Mishra: Of course it has room for a lot of improvement, but this is like

1211
02:11:09.300 --> 02:11:15.590
Sourav Mishra: kind of like stripping down a fully functional blockchain that is out there in production to its grassroots, level.

1212
02:11:16.110 --> 02:11:18.540
Gabor Szabo: Okay, I'll ask it a different way. Okay.

1213
02:11:18.760 --> 02:11:27.080
Gabor Szabo: I totally understand. Why would someone learn how to write a web application? Okay? Because you need web applications for

1214
02:11:28.000 --> 02:11:29.820
Gabor Szabo: millions of companies.

1215
02:11:30.130 --> 02:11:31.090
Gabor Szabo: Okay.

1216
02:11:31.290 --> 02:11:32.280
Sourav Mishra: Okay.

1217
02:11:32.560 --> 02:11:36.140
Gabor Szabo: But I I don't understand.

1218
02:11:36.660 --> 02:11:43.590
Gabor Szabo: but I don't think that the 1,000, even thousands of companies will want to have their own blockchain.

1219
02:11:43.880 --> 02:11:46.759
Gabor Szabo: Yeah, company don't usually have their own logins.

1220
02:11:47.262 --> 02:11:50.399
Sourav Mishra: So companies don't usually have their own blockchains.

1221
02:11:50.400 --> 02:11:56.650
Gabor Szabo: Exactly so. So you have very few blockchains in the world at the end, right.

1222
02:11:56.650 --> 02:11:59.179
Sourav Mishra: Do have a lot of blockage in the world. But.

1223
02:11:59.180 --> 02:12:04.739
Gabor Szabo: How how many I mean hundreds, thousands, or or how many do you think there there are.

1224
02:12:06.560 --> 02:12:11.800
Sourav Mishra: it could be hundreds could be thousands. I'm not really sure. But it's a big number. It's not like.

1225
02:12:11.800 --> 02:12:24.919
Gabor Szabo: That's fine. That's fine. But it's not like a million. So the question is, okay. So I see lots of ads, for example, for peeping people

1226
02:12:25.060 --> 02:12:30.129
Gabor Szabo: writing code. Okay? So programmers who know blockchain, whatever level.

1227
02:12:30.240 --> 02:12:32.589
Gabor Szabo: what kind of work do they do? I mean.

1228
02:12:33.170 --> 02:12:35.540
Gabor Szabo: they don't implement new blockchains, right?

1229
02:12:36.170 --> 02:12:43.709
Sourav Mishra: No, they build blockchains from scratch, like I am a core protocol engineer. So basically, that's what I do. That's my job. Building blockchains from scratch.

1230
02:12:43.900 --> 02:12:52.909
Sourav Mishra: And we implement basically different protocols that are not pre-existing. Or we implement protocols that are suited to our needs.

1231
02:12:54.560 --> 02:12:54.980
Gabor Szabo: Do this.

1232
02:12:55.920 --> 02:13:05.770
Sourav Mishra: We implement basically custom blockchains. So if you have a business, need you implement a blockchain based on that particular need which no other public blockchain offers. You.

1233
02:13:06.430 --> 02:13:11.445
Gabor Szabo: Right? Okay, but what? What? What? Yeah. So, but what we are saying here is that,

1234
02:13:12.720 --> 02:13:18.130
Gabor Szabo: there is not like in the millions. It's like the thousands. The number of these blockchains.

1235
02:13:18.450 --> 02:13:28.660
Sourav Mishra: Yeah, it generally depends on a use case. So if you already have a blockchain that so kind of caters to some use case, then you don't really need to create a new blockchain that does the same thing.

1236
02:13:29.220 --> 02:13:31.695
Gabor Szabo: Okay. But do do people,

1237
02:13:32.650 --> 02:13:44.699
Gabor Szabo: Keep improving blockchain? So what is what is a besides what you do? Okay, so so that's fine. There are a few people like you who build blockchains from scratch

1238
02:13:45.730 --> 02:13:59.059
Gabor Szabo: right? But there are lots of jobs out there. It seems to be that the one of the most popular job titles in for us developers or the areas of of development is somewhere in in blockchain.

1239
02:13:59.950 --> 02:14:02.810
Gabor Szabo: So what what do these do all these developers do.

1240
02:14:04.155 --> 02:14:07.114
Sourav Mishra: It's not only building blockchains from scratch like there are

1241
02:14:07.900 --> 02:14:18.510
Sourav Mishra: plethora of job roles available. One of the most popular ones is a smart contract developer. So basically, you build smart contracts and deploy them to already existing blocks and platforms.

1242
02:14:19.210 --> 02:14:25.749
Sourav Mishra: Okay, ethereum uses solid contracts. Our polka dot can use. Let's say, inc, smart contracts. For example.

1243
02:14:26.917 --> 02:14:33.360
Sourav Mishra: anchor contracts. So that is, I think, by far the most popular role. If you talk about jobs.

1244
02:14:33.840 --> 02:14:35.570
Sourav Mishra: smart contact development.

1245
02:14:36.040 --> 02:14:36.800
Gabor Szabo: Okay.

1246
02:14:36.800 --> 02:14:45.019
Sourav Mishra: And then the there's a d app developer like who builds mobile and web applications on top of existing blockchains and use them as the backend.

1247
02:14:46.660 --> 02:14:51.900
Sourav Mishra: So it's not always only building blockchains from scratch, but also building on top of blockchains that do exist.

1248
02:14:52.740 --> 02:14:53.460
Gabor Szabo: Okay.

1249
02:14:53.460 --> 02:14:57.999
Sourav Mishra: But you can think of it as a different discipline of blockchains. Software blockchains.

1250
02:14:59.010 --> 02:14:59.660
Gabor Szabo: Right?

1251
02:15:00.610 --> 02:15:01.560
Gabor Szabo: Okay.

1252
02:15:06.050 --> 02:15:07.070
Gabor Szabo: okay.

1253
02:15:07.830 --> 02:15:12.480
Sourav Mishra: Okay. So I think we are done with the implementation. Let's try to run and see if it works.

1254
02:15:12.750 --> 02:15:13.170
Gabor Szabo: Okay.

1255
02:15:13.590 --> 02:15:14.730
Sourav Mishra: I do hope it works.

1256
02:15:22.610 --> 02:15:30.200
Sourav Mishra: This is the command to run it cross our fingers if it works.

1257
02:15:30.890 --> 02:15:31.610
Gabor Szabo: Mom.

1258
02:15:32.600 --> 02:15:37.129
Sourav Mishra: Okay. So now we have blockchain running on this address on this port.

1259
02:15:38.240 --> 02:15:44.559
Sourav Mishra: Again. It's hard coded, so it should not be hard coded. It should be fetching it from some envy or something, but it's hard coded. So let's just

1260
02:15:44.780 --> 02:15:50.219
Sourav Mishra: move on with that, and I will share my a wallet screen. No.

1261
02:16:08.560 --> 02:16:10.940
Sourav Mishra: I hope you can see my screen.

1262
02:16:13.353 --> 02:16:17.620
Gabor Szabo: Yes, I can see your editor right now.

1263
02:16:18.770 --> 02:16:20.590
Sourav Mishra: Let's open the wallet screen. Now.

1264
02:16:20.590 --> 02:16:21.439
Gabor Szabo: Hello! Everyone.

1265
02:16:22.740 --> 02:16:28.290
Sourav Mishra: So this is my wallet code that I wrote. It basically is a client that connects with the blockchain.

1266
02:16:28.660 --> 02:16:29.390
Gabor Szabo: Okay.

1267
02:16:29.590 --> 02:16:50.690
Sourav Mishra: Yeah, so there are various files here. So the command file basically defines the available commands like new command creates a new wallet list command. Let's list from the wallets that are available already. Balance command fetches the balance from the blockchain for that particular address and send command sends transactions, and the faucet, command, request money from the faucet.

1268
02:16:52.440 --> 02:16:56.339
Sourav Mishra: and then I have an errors. File where I have defined a custom error for wallet error.

1269
02:16:56.680 --> 02:17:04.960
Sourav Mishra: So these are the various errors that are defined as an enum, and then I have also implemented the corresponding string representation by using implementing the display

1270
02:17:05.090 --> 02:17:07.889
Sourav Mishra: trade for them for this particular webinar.

1271
02:17:08.209 --> 02:17:20.819
Sourav Mishra: and then I have added some conversions because I want to return only wallet errors. So even if it is an I/O error, I am converting it to some form of wallet error, so that I can just return wallet error. I don't need to cater 2 different types of errors.

1272
02:17:21.250 --> 02:17:28.319
Sourav Mishra: and then at the end I have a custom result type that returns a generic T when it's all right.

1273
02:17:28.420 --> 02:17:44.940
Sourav Mishra: But if it is an error, then it returns wallet error always. So this is the reason why I needed to do all of this, because these are potentially the errors that I will run into in my code, and if I do, then how do I convert them into wallet errors? So I have written the form. I implemented, the form tricks for all these particular errors.

1274
02:17:45.209 --> 02:17:49.000
Sourav Mishra: If it is strategation, error, then it's disemparts error for the volatile.

1275
02:17:49.240 --> 02:17:53.790
Sourav Mishra: If it is tonic, transfer error, then it is volatile connection failed error, and so on and so forth.

1276
02:17:55.660 --> 02:18:00.849
Sourav Mishra: And in the main file, I have basically implemented the command. So like, what happens if you.

1277
02:18:01.549 --> 02:18:07.339
Sourav Mishra: if a command is create wallet, then what should happen if a command is list wallet, then what should happen? So on and so forth.

1278
02:18:07.969 --> 02:18:13.919
Sourav Mishra: In the models I have defined some types. One of the types is keypad, that is, a private key and public keypad.

1279
02:18:14.160 --> 02:18:19.360
Sourav Mishra: And the other type is wallets. That is basically a hash map of a wallet name

1280
02:18:19.670 --> 02:18:21.439
Sourav Mishra: map to a key pair

1281
02:18:23.360 --> 02:18:29.799
Sourav Mishra: in the profile. It's again just pop more blockchain that we had defined earlier in our blockchain. Also.

1282
02:18:30.219 --> 02:18:35.529
Sourav Mishra: it just includes the photograph into our code and creates a model out of it.

1283
02:18:36.660 --> 02:18:42.004
Sourav Mishra: Storage dot rs, this load function it

1284
02:18:43.160 --> 02:18:50.850
Sourav Mishra: checks the memory, and if the wallet file already exists, then it loads it into memory, and the save function basically writes to the wallet file.

1285
02:18:52.070 --> 02:18:57.719
Sourav Mishra: The add wallet function adds a new wallet to the already existing wallet file. So wallet file looks something like this.

1286
02:18:58.410 --> 02:19:01.990
Sourav Mishra: It's adjacent, basically. So I've already created 2 wallets for you and me.

1287
02:19:06.040 --> 02:19:11.629
Sourav Mishra: And the get wallet function gets a wallet by the name from its collection.

1288
02:19:12.620 --> 02:19:16.259
Sourav Mishra: And the resolve address basically takes a name or key.

1289
02:19:16.520 --> 02:19:20.560
Sourav Mishra: and if the name matches with the wallet's name, then it returns it. If the

1290
02:19:20.940 --> 02:19:24.669
Sourav Mishra: publicly matches with the sum of I mean

1291
02:19:25.170 --> 02:19:30.289
Sourav Mishra: any of these public keys, then it returns. The public key doesn't matter what name it is.

1292
02:19:30.980 --> 02:19:41.639
Sourav Mishra: And finally, if it is not a wallet that we have saved on us, even by name or by the public key, if it is. But if it is any generic general valid public key.

1293
02:19:41.910 --> 02:19:45.690
Sourav Mishra: then also you can return it. It's valid. If it is neither, then return one.

1294
02:19:48.000 --> 02:19:53.756
Sourav Mishra: And finally, this wallet is the grpc implementation, the client implementation. This calls the functions

1295
02:19:54.340 --> 02:19:59.109
Sourav Mishra: so these there are some Async functions like this, get balance which calls the

1296
02:19:59.420 --> 02:20:03.269
Sourav Mishra: Rpc. Get balance from the on the on. The blockchain gets it on the blockchain.

1297
02:20:04.720 --> 02:20:08.040
Sourav Mishra: So yeah, that is the entirety of the wallet application.

1298
02:20:14.160 --> 02:20:15.360
Sourav Mishra: So let's

1299
02:20:16.280 --> 02:20:25.379
Sourav Mishra: done this. So actually, this is also available as upgrade on greater diode. So I have it installed. The package is called mock wallet.

1300
02:20:27.720 --> 02:20:31.549
Sourav Mishra: So I'm not gonna run the code. But I'm gonna use the no correct command.

1301
02:20:35.510 --> 02:20:36.430
Gabor Szabo: Okay.

1302
02:20:37.010 --> 02:20:38.489
Sourav Mishra: So let's create a new wallet.

1303
02:20:39.450 --> 02:20:42.130
Sourav Mishra: just to show how to create wallets on you.

1304
02:20:42.610 --> 02:20:46.010
Sourav Mishra: Then let's give some name. Let's say, quote, madam.

1305
02:20:48.270 --> 02:20:52.770
Sourav Mishra: and it says, a new wallet code method has been created, and if we check the wallets file.

1306
02:20:53.300 --> 02:20:57.669
Sourav Mishra: we can see that support map, and already there, with a corresponding publicly and privately.

1307
02:20:59.710 --> 02:21:04.049
Sourav Mishra: So now let's try to send some money first.st Take. Let's take a balance.

1308
02:21:06.800 --> 02:21:10.390
Sourav Mishra: VLNG. For let's check it for Gabor.

1309
02:21:11.106 --> 02:21:17.760
Sourav Mishra: As expected, you have 0 coins because it's a new model we have not yet transacted, or you have not received anything on the faucet.

1310
02:21:21.300 --> 02:21:23.770
Sourav Mishra: So let's request some money from the faucet.

1311
02:21:29.530 --> 02:21:34.919
Sourav Mishra: And this works, yeah. So you receive 1,400 coins in your volume.

1312
02:21:35.050 --> 02:21:40.470
Sourav Mishra: And if we go back to the blocks and implementation, we can see that some blogs are added.

1313
02:21:40.870 --> 02:21:49.060
Sourav Mishra: so adding transactions from faucet to pool, it's added 1,400 transactions. 1,400 coins are added, and then the faucet block is also created

1314
02:21:49.430 --> 02:21:57.369
Sourav Mishra: with the Hesters, so like whatever you do here, the blockchain gets it because there are Grpc connections.

1315
02:21:58.680 --> 02:21:59.510
Gabor Szabo: Okay.

1316
02:22:01.140 --> 02:22:02.710
Sourav Mishra: So now let's try to send money.

1317
02:22:03.510 --> 02:22:04.780
Gabor Szabo: Show me the balance.

1318
02:22:05.390 --> 02:22:06.669
Gabor Szabo: Show me my balance.

1319
02:22:06.970 --> 02:22:09.419
Sourav Mishra: Definitely, do you want it?

1320
02:22:10.660 --> 02:22:15.780
Gabor Szabo: Okay, we have 2, 8, 00 points. It twice.

1321
02:22:16.870 --> 02:22:17.740
Gabor Szabo: Yeah. Good.

1322
02:22:18.020 --> 02:22:21.320
Sourav Mishra: Now let's try to transfer some money, so send.

1323
02:22:22.230 --> 02:22:26.480
Sourav Mishra: is it? Let's in the board thought of.

1324
02:22:27.590 --> 02:22:29.240
Sourav Mishra: I'm gonna take most of your money.

1325
02:22:30.890 --> 02:22:33.700
Gabor Szabo: Well you were, for you were hard, so you could skip.

1326
02:22:34.720 --> 02:22:36.349
Sourav Mishra: Okay, fine. I'll take the 500,

1327
02:22:38.570 --> 02:22:44.579
Sourav Mishra: and so you can see. I entered the name Sora, but it said Invalid, because there is no address for Sara.

1328
02:22:44.790 --> 02:22:47.620
Sourav Mishra: and you can see that is called Saurav with Saurav Mishra.

1329
02:22:47.740 --> 02:22:49.430
Sourav Mishra: So we have to enter the name.

1330
02:22:50.060 --> 02:22:52.509
Sourav Mishra: and you can see transaction has been sent successfully.

1331
02:22:52.750 --> 02:22:57.400
Sourav Mishra: Okay, you can see it added the transaction to the pool.

1332
02:22:58.080 --> 02:22:58.660
Gabor Szabo: Yeah.

1333
02:22:59.930 --> 02:23:04.239
Sourav Mishra: So if you add one more transaction, let's say, do the same thing. But with this coins now

1334
02:23:05.370 --> 02:23:09.549
Sourav Mishra: the transaction is again sent successfully, and this is added.

1335
02:23:09.940 --> 02:23:11.990
Sourav Mishra: and a new block is mined.

1336
02:23:13.280 --> 02:23:13.860
Gabor Szabo: Yeah.

1337
02:23:16.470 --> 02:23:20.539
Sourav Mishra: So that is the whole implementation of our blockchain and the client.

1338
02:23:26.270 --> 02:23:27.290
Gabor Szabo: Okay.

1339
02:23:30.080 --> 02:23:32.689
Sourav Mishra: So there are like a lot of areas to improve on.

1340
02:23:33.370 --> 02:23:40.429
Sourav Mishra: But this is like I could strip it down to the least complexity, and I came up with this.

1341
02:23:41.730 --> 02:23:43.810
Gabor Szabo: Very nice, nice.

1342
02:23:43.810 --> 02:23:55.849
Sourav Mishra: I had removed some more features, then it might have become a really boring blockchain. So I tried to include all aspects of the blockchain while keeping it simple. So this is I could. What I could come up with. It took longer than expected to explain it and write it down.

1343
02:23:56.090 --> 02:23:58.169
Sourav Mishra: but I hope at least it was worth it.

1344
02:23:58.780 --> 02:24:04.059
Gabor Szabo: Yeah, yeah, I think I think I I learned a lot. I still need to. I mean, I

1345
02:24:04.670 --> 02:24:12.160
Gabor Szabo: still need to go over for over this again and again, I guess, until I it sinks in. But yeah, thank you very much.

1346
02:24:15.200 --> 02:24:21.549
Sourav Mishra: You can follow the read me, there are some areas where you can contribute. So if you like this project, then you can contribute this to this project.

1347
02:24:21.880 --> 02:24:30.560
Sourav Mishra: So towards the end there are some, maybe enhancements that can be done. There are some concession mechanisms that can be added like we have not added, for proof of stake or proof of authority, something like that.

1348
02:24:32.030 --> 02:24:41.769
Gabor Szabo: and then you made the changes you made here in this branch are they all available available on the main branch or, did you push this out, or.

1349
02:24:41.770 --> 02:24:52.160
Sourav Mishra: It's all available in the main branch, so I try to make it same as the main branch. So people who are following live can also be benefited, while people who are not following live and checking it out from the Github, they can also benefit.

1350
02:24:52.500 --> 02:24:54.210
Sourav Mishra: Okay.

1351
02:24:54.910 --> 02:24:55.740
Gabor Szabo: Excellent.

1352
02:24:57.420 --> 02:25:01.080
Gabor Szabo: Well, thank you very much.

1353
02:25:01.340 --> 02:25:02.360
Sourav Mishra: Anyone have a doubt.

1354
02:25:05.600 --> 02:25:06.520
Sourav Mishra: something.

1355
02:25:06.810 --> 02:25:08.519
Gabor Szabo: I'm overwhelmed. So

1356
02:25:09.980 --> 02:25:20.560
Gabor Szabo: so thank you very much for this presentation, and thank you. Everyone who who stayed and and well, there were not not many questions. At the beginning we were a few.

1357
02:25:21.160 --> 02:25:31.489
Gabor Szabo: and I hope that people who watch this video enjoyed it and please like the video and follow the channel, and below the video you'll find links to

1358
02:25:31.690 --> 02:25:41.380
Gabor Szabo: to the repository as well and to to Sorab, so you can contact him if you want to. I don't know. Work with him, or just ask him questions.

1359
02:25:41.880 --> 02:25:45.320
Gabor Szabo: and I think that's it right. Anything else.

1360
02:25:45.320 --> 02:25:51.520
Sourav Mishra: I sent my telegram. Id on chat. So, Radek, if you're interested, you can contact me on telegram, and we can chat if you want to chat. Talk about this project.

1361
02:25:52.483 --> 02:26:03.539
Gabor Szabo: Excellent, so I can, whatever you you like. I can share on on this web page, or I can link to your I don't know. I'm going to link to your Linkedin page and and whatever you like. Okay.

1362
02:26:03.960 --> 02:26:05.619
Sourav Mishra: Yeah, that works. Thank you so much.

1363
02:26:05.930 --> 02:26:11.730
Gabor Szabo: Yeah, thank you very much. Okay, so thank you. Thanks everyone. And see you next time. Bye, bye.

1364
02:26:12.120 --> 02:26:12.820
Sourav Mishra: Bye, bye.



