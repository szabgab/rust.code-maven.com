---
title: Rust at Work - conversation with Eli Shalom and Igal Tabachnik of Eureka Labs
timestamp: 2025-05-15T07:30:01
author: szabgab
published: true
show_related: true
description:
---


{% youtube id="X4ka4Q1AHTs" file="2025-05-27-rust-at-work-conversation-with-eli-shalom-and-igal-tabachnik-of-eureka-labs.mp4" %}


* Guests: [Eli Shalom](https://www.linkedin.com/in/elishalom/) & [Igal Tabachnik](https://www.linkedin.com/in/igaltabachnik/) of [Eureka Labs](https://eurekalabs.xyz/)
* Host: [Gabor Szabo](https://szabgab.com/)
* Language: English
* Location: Zoom

In this episode, we explore how Rust is powering infrastructure at Eureka Labs - a blockchain company operating in a low-latency, high-throughput environment.

Eureka Labs is pioneering next-generation block building on Ethereum. Their work focuses on advancing the logic of block construction to support more efficient execution and expand the functionality that can be packed into each block’s limited timeframe.

Eli and Igal will discuss how Rust has been instrumental in developing their high-performance block builder. Before founding Eureka, the team tackled large-scale engineering and algorithmic problems across industries.

If you have questions to Eli and Igal, please send it to [Gabor Szabo](https://szabgab.com/contact)


## Transcript

1
00:00:02.390 --> 00:00:19.680
Gabor Szabo: Hello and welcome to the code. Meet Maven meeting and podcast on the raw station. If you're listening to us, this is going to be a live recording, meaning that there are going to be. There are a few guests beyond or guest speakers.

2
00:00:19.690 --> 00:00:33.920
Gabor Szabo: There are a few guests who listen and can ask questions in the chat room. They can't speak, because that's only the 3 of us. And but those people who are present here, you're welcome to ask questions there.

3
00:00:35.060 --> 00:00:47.139
Gabor Szabo: Welcome, Ellie and Igal. So our guest this time is Ali Shalom, the CEO of Euray collapse, and Igal Tabachnik right.

4
00:00:47.140 --> 00:00:48.369
Igal Tabachnik: That's exactly right.

5
00:00:48.920 --> 00:00:51.712
Gabor Szabo: Okay, senior developer and

6
00:00:52.670 --> 00:01:17.559
Gabor Szabo: we are going to talk about your your company and rust, and how you use rust. So I think the 1st thing that I mean the 1st thing I think I should just say myself that I my name is Gabor Sabo, and I'm helping companies with rust and python, and then that's about me, that's enough. And now please introduce yourself. Let's start with Ellie, and then Igar, and please let's see.

7
00:01:18.010 --> 00:01:41.370
Eli Shalom: So thank you very much, Gabo, for inviting us to this. Podcast and even though you gave me the credits for being the CEO and the CTO. And we have another person as the CEO of the company, and my background is very technological. I come from. I spent most of my career in engineering roles. I did a lot of languages and not rust, and started from

8
00:01:41.550 --> 00:01:58.149
Eli Shalom: things like C sharp and C plus plus, and ended as a python developer, mostly for data science purposes and a lot of my career I did in analyzing code. I work for companies who analyze code both for quality and for security. So

9
00:01:58.310 --> 00:02:12.739
Eli Shalom: code and coding languages are a very great passion of mine, and the way we get to rust is very different from code analysis, and we'll get to it today. But it's a great passion of me and most of the people in the in the R. And D.

10
00:02:15.480 --> 00:02:16.260
Igal Tabachnik: Great.

11
00:02:16.370 --> 00:02:36.870
Igal Tabachnik: And I'll introduce myself. So also thank you very much for inviting us to the podcast so my name is Igal Tapachnik. I'm a software developer. And I've also had an interesting career spanning from beginning being Anet developer. And Ellie and I worked together many years ago in another company.

12
00:02:37.400 --> 00:03:06.440
Igal Tabachnik: and we reconnected recently. And we'll talk about that in a bit. So I spent sometimes also working in the area of code tools and developer tools. So building tools for developers which requires a very specialized set of skills. But in the recent years I've switched careers and worked primarily as a backend engineer in companies that primarily work on the Jvm. So my background in the last.

13
00:03:06.650 --> 00:03:17.700
Igal Tabachnik: I want to say, 8 or 9 years was doing backend on Scala the language. So my my journey to rust is also a bit

14
00:03:17.910 --> 00:03:20.130
Igal Tabachnik: different from maybe

15
00:03:20.260 --> 00:03:30.489
Igal Tabachnik: that of other people who come from maybe C plus plus background to rust. I come from more of a functional language, and we'll talk about that. Shortly

16
00:03:30.650 --> 00:03:51.360
Igal Tabachnik: and recently Ellie and I met and I joined Eureka. It didn't have to. It didn't, didn't take a lot to convince me to join Eureka primarily, because I knew that we're going to be developing in rust. And this is something I wanted to try. And the product itself is actually very interesting. And we'll talk about that in a bit.

17
00:03:52.510 --> 00:03:57.539
Gabor Szabo: Yeah. So as I understand, I mean, I I actually also work with Ellie. I'm not in

18
00:03:57.850 --> 00:04:00.480
Gabor Szabo: with him, but in the same company a couple of years ago.

19
00:04:00.920 --> 00:04:03.350
Gabor Szabo: so we know each other from there.

20
00:04:04.349 --> 00:04:17.759
Gabor Szabo: But I really would like to to hear a little bit more about the company. I mean, we haven't yet, so please enjoy what I know so far is it's a very young company. It's only exist a couple of months right.

21
00:04:18.680 --> 00:04:30.190
Eli Shalom: Yeah. So in fact, we started working on on founding Eureka last year. We started with a very open goal like to improve

22
00:04:30.220 --> 00:04:39.380
Eli Shalom: something in crypto or in blockchain, as a technology in general. And we are looking for a lot of areas where there are challenges which are both

23
00:04:39.410 --> 00:04:51.440
Eli Shalom: interesting in the technological aspect of them, and most importantly, to convince the investors that it's worth coming to the company to have something with real essence, something where where there's enough money

24
00:04:51.440 --> 00:05:10.680
Eli Shalom: today to work in. The reason I'm mentioning it is that a lot of companies in the blockchain deal with a greater challenge where they have to aim for the market, which will be there in a few years, and they have to speculate. And it's harder. And what we're working on is something which is active today there is a lot of

25
00:05:10.710 --> 00:05:34.230
Eli Shalom: activity both in terms of traffic and in money. And this is where we are. So what Eureka is doing is we're building a new block builder for ethereum networks. And so one of the things I'm going to do is to give a little bit of explanations, and I'm more than willing to accept questions. Gabble on anything around it.

26
00:05:34.480 --> 00:06:02.489
Gabor Szabo: Yeah. So I should play here the devil's advocate, I think and say that I don't know anything about this, and please explain. But it will be quite easy for me, because I really don't know anything about blockchain, so I will. So please also consider. I mean, I'm quite sure that in the audience here, and then later on, in the podcast there are going to be people who are experts in blockchain. But I'm also going to be a lot of people who don't know anything about it. So

27
00:06:02.790 --> 00:06:19.400
Gabor Szabo: as we go, please also explain a little bit about it. Do I understand correctly that actually, you started the company without having any specific product or service in mind. Just you want to say, Okay, improve something in the area of blockchain.

28
00:06:19.920 --> 00:06:42.040
Eli Shalom: That's the story we usually tell. It's not that we knew that it's going to be around block building, we knew, but it's going to be around one of the areas which suffer from over centralization. I'll elaborate on it in a second. But we weren't sure what exactly was going to be the product or what's going to be the and the length of the technology which we'll have to develop in order to

29
00:06:42.120 --> 00:06:50.409
Eli Shalom: achieve whatever it's going to be eventually it got to be something a little bit ambitious, challenging to develop.

30
00:06:50.560 --> 00:06:59.750
Eli Shalom: And and the reason for it is, and I'll have to explain a little bit what block builder is, and what block building is in blockchain.

31
00:06:59.750 --> 00:07:01.440
Gabor Szabo: Okay, please. Please. Go ahead.

32
00:07:01.440 --> 00:07:18.289
Eli Shalom: And so, blockchain as it sounds, it's a chain of blocks. So at every moment in the blockchain another block is added to the blockchain. And this is how the whole chain is being formed. So what does it mean to build a block or to add it to the chain?

33
00:07:18.290 --> 00:07:31.509
Eli Shalom: So in every moment in the blockchain. Some people want to send their transactions to the ethereum network. Sending a transaction means that someone wants to run a smart contract, which is

34
00:07:31.510 --> 00:07:44.579
Eli Shalom: a piece of code which can be verified before being executed, and can be verified about the actual execution it had, and it can be verified. So it means that if we have a smart contract.

35
00:07:45.040 --> 00:07:58.219
Eli Shalom: and which, for example, promises that we can swap assets between us, then nobody can change this code. And once we run this code, everyone can verify that, it ran exactly as the code shows.

36
00:07:58.360 --> 00:08:19.460
Eli Shalom: So that's something which sounds a little bit trivial or not very interesting, but when we look at a chain of transactions going into a single block, then they have a lot of side effects between them, because it's like a regular program. So when one transaction touches a variable

37
00:08:19.460 --> 00:08:39.220
Eli Shalom: in the ethereum network, then every transaction which comes after that will see exactly this side effect. So it means that ordering the transactions is a challenge. And this is where it's becoming interesting. So when we're looking at the problem of block building, it's just a problem of

38
00:08:39.620 --> 00:08:46.980
Eli Shalom: choosing a set of transactions and ordering them in the way which leads to the best or most valuable block.

39
00:08:47.620 --> 00:09:05.039
Eli Shalom: So this is something which is going to be with too many details, so we'll skip it. But the nice thing which is easy to explain is that building a block is an Np-hal problem. It's both equivalent to the permutation optimization problem. And it's also a knapsack problem.

40
00:09:05.040 --> 00:09:21.459
Eli Shalom: So we have to deal with Npr problem in order to be able to build a block. So this is the main premise of what we have to do. And the reason we're mentioning this and not anything else is. This is why we have to choose rust eventually.

41
00:09:21.570 --> 00:09:39.649
Eli Shalom: And so, if we have to summarize this in the most oversimplified way, and building a block is the challenge of ordering a set of transactions or set of actions, and trying to predict or getting to the most valuable order of those transactions.

42
00:09:39.750 --> 00:09:40.640
Eli Shalom: So

43
00:09:41.020 --> 00:10:02.810
Eli Shalom: if you have to look at it, you can look at it as a kind of a black box, and we give to the black box a set of transactions or a set of actions in a specific order, and we get a value. And if we change this order, we will get a different value. And what we're looking for is the most valuable ordering which we can create in order to build a block.

44
00:10:03.530 --> 00:10:08.489
Gabor Szabo: But who who builds the block? So I what I understood that there is the ethereum ethereum

45
00:10:09.370 --> 00:10:25.259
Gabor Szabo: network, or or I don't know. It's a virtual machine or the the service that runs on many computers. Okay, I read a little bit about it in the last couple of days. And isn't that doing the the ordering of these blocks.

46
00:10:25.620 --> 00:10:50.580
Eli Shalom: So the network itself is in charge of verifying that the blocks that the block builder created are, in fact valid, and can be satisfied as regular blocks which could be placed online and be checked by everyone else. So what we have is we have a few sets of very few block builders on ethereum. These are the entities which come before the transactions get to the blockchain itself.

47
00:10:50.770 --> 00:11:03.170
Eli Shalom: So we are off chain where, before the things get to the blockchain. And we get those all those inputs. We try to create a list of transactions in a way which is forming a potential block.

48
00:11:03.450 --> 00:11:27.369
Eli Shalom: And in every 12 seconds. In ethereum there is an auction, and every active block builder can propose the block to the network, and there is some kind of mechanism, how the block is being chosen, which practically says the most valuable block is being taken, and then the network goes to the validators and atchestrators who verify that the block is actually valid.

49
00:11:27.460 --> 00:11:36.649
Eli Shalom: but it respects everything on the smart contracts, and that all the results that it shows are, in fact valid for the execution the transactions had.

50
00:11:37.070 --> 00:11:57.490
Eli Shalom: So the block builder comes before the fury network, and before all the cool, decentralized entities it create. It creates a proposal for the network, and then the network chooses one of the proposals and verifies them with the nodes, validators, and other stakeholders.

51
00:11:58.100 --> 00:11:58.520
Gabor Szabo: Okay.

52
00:11:58.520 --> 00:12:17.119
Igal Tabachnik: So, if I may, I will also like to add so personally, I don't come from a background in blockchain. So I had to learn all this from scratch as well. Kind of a few months ago this was very new to me, and the way I understand it is very similar. It's very, very technical. But the idea is you have

53
00:12:17.420 --> 00:12:44.590
Igal Tabachnik: kind of like an Internet, a blockchain that's designed for executing code in the blockchain space called smart contracts. Smart contracts can be money transfers between different people, or there could be pieces of actual code like actual state that's being executed safely and cryptographically verifiable way. So the flow is that there are millions, or there are many, many

54
00:12:44.750 --> 00:12:59.239
Igal Tabachnik: hundreds of thousands of these transactions being created every second, and being placed in the network. What the blog builders do is they pick up those transactions from the ether pun intended from the network? We call them

55
00:12:59.240 --> 00:13:15.450
Igal Tabachnik: orders, and then the the goal of this block builder of what we do is like, Ellie said, is to try and create the best and the most valuable ordering of these transactions. So once this block containing all of these transactions.

56
00:13:15.760 --> 00:13:40.569
Igal Tabachnik: is created, we send it to those entities called the proposers. We propose the block, and then they can decide whether or not take this block. If they take this block, they can then publish it, and execute all of these transactions, and if not, every 12 seconds. This is the way the network works every 12 seconds. Everything repeats. So a new block is being created with new orders.

57
00:13:40.570 --> 00:13:57.229
Igal Tabachnik: etc, etc. So the idea is that we are the block builder, the block builder, which is what we do at Eureka is the entity that's responsible for actually turning those many of those transactions that are floating out there. Take them.

58
00:13:57.600 --> 00:14:02.410
Igal Tabachnik: order them in a way that they will be they. They will have the best chance of being picked up

59
00:14:02.810 --> 00:14:04.590
Igal Tabachnik: and executed.

60
00:14:05.210 --> 00:14:16.200
Igal Tabachnik: and and do it again and again and again, and so there are not too many entities out there that are called block builders. There are a couple of those. There are a couple of big names out there.

61
00:14:16.320 --> 00:14:32.749
Igal Tabachnik: and what we're trying to do is we're trying to. We're trying to to become yet another blockbuilder that's hopefully going to be powerful enough by doing some optimizations in a way that we create the blocks.

62
00:14:33.760 --> 00:14:36.079
Gabor Szabo: Okay, so basically, this log builder

63
00:14:36.680 --> 00:14:46.940
Gabor Szabo: will run or can run on on your your servers if I understand. So it's nothing really to do with the rest of the computers that are running the the blockchain.

64
00:14:47.180 --> 00:14:54.830
Gabor Szabo: They have some communication, but that it doesn't have to, and it can be either open source or proprietary right.

65
00:14:54.830 --> 00:15:17.350
Igal Tabachnik: Yeah. And that's kind of the point, the idea, the whole kind of the biggest reason why we chose rust. And I will just say it up front is, it's kind of, was a decision that was made for us. So there are some open source. Big, open source projects that propose give you building blocks to build your own block builder.

66
00:15:17.650 --> 00:15:36.219
Igal Tabachnik: So we essentially what we did, we forked that we, we took some existing open source infrastructure, and we forked it in a way that that we're adding our own modifications. But this, this entire infrastructure was written in rust.

67
00:15:36.970 --> 00:15:45.320
Igal Tabachnik: and so we kind of went with that. So the rust kind of has a strong presence in the blockchain space for historical reasons.

68
00:15:45.700 --> 00:15:52.610
Igal Tabachnik: But it's also very, very suitable, because in the end you're essentially, it's a server that gets deployed to the cloud

69
00:15:52.750 --> 00:15:59.920
Igal Tabachnik: like real actual machines that run in aws or whatnot. And essentially, it's a, it's a

70
00:16:00.220 --> 00:16:05.410
Igal Tabachnik: well, it's a network. It's a machine that listens to traffic on the, on the web.

71
00:16:05.720 --> 00:16:09.900
Igal Tabachnik: on the ethereum network. But it's essentially a service that runs out there.

72
00:16:10.060 --> 00:16:20.629
Gabor Szabo: So 2 2 questions. Come here. 1st of all, okay. So you forward. It is your code, or is it? Is it open source, or is it closed source, or how or what's going on with that.

73
00:16:21.440 --> 00:16:44.569
Eli Shalom: Is closed. Source we. If we see bugs or or improvements which are general to a general block builder we submit it back to the, to the original fork. But when we are developing new features, which are the new features, are, be a little bit beyond block building, and take advantage of what people can do with it. And so these things are not

74
00:16:44.690 --> 00:16:48.079
Eli Shalom: being opened open source.

75
00:16:48.470 --> 00:16:49.880
Eli Shalom: So we leave them closed doors.

76
00:16:49.880 --> 00:16:57.470
Igal Tabachnik: Yeah, okay, so the the license for it is, it's either Mit or Apache. So we we do submit.

77
00:16:57.470 --> 00:17:21.579
Igal Tabachnik: We do submit a couple of fixes upstream. So basically, that thing is being developed in the open, and we submit patches to it when we, when we can, those things that are going to be useful for for the general audience. But we kind of extend the product. Extend the platform in a way that's proprietary. And we right now it's not open source.

78
00:17:21.760 --> 00:17:37.200
Gabor Szabo: Okay? And then, so how? How do you earn money from this? Is it like, if your your block, if you're chaining, or whatever it's called. Your result is accepted. Then you earn some money. Do I understand correctly?

79
00:17:37.870 --> 00:17:54.070
Eli Shalom: Yeah, this is exactly how it works. And when a block builder wins an auction, and and this block builder gets a fee, it's varying between every block, but depending on how valuable you made it. Then this is how much fee the block builder can take.

80
00:17:54.070 --> 00:18:01.320
Gabor Szabo: Okay, so you have to be fast and clever. And yeah, basically, this, mostly these 2 things right?

81
00:18:01.885 --> 00:18:02.450
Igal Tabachnik: Exactly.

82
00:18:02.450 --> 00:18:03.030
Eli Shalom: That's.

83
00:18:03.030 --> 00:18:08.850
Igal Tabachnik: Primarily, primarily the reason why you would want to pick a language like rust to do this because you want to be both.

84
00:18:08.970 --> 00:18:13.350
Igal Tabachnik: Well, there are 3 things that you want to be. You want to be 1st and foremost correct.

85
00:18:13.470 --> 00:18:34.809
Igal Tabachnik: You need to. You need to ensure, because we're dealing with things that depend on. Well, they are order, sensitive, they're hard, but they also have to be verifiable in the end. There's a lot of verification steps and a lot of a lot of cryptographic hashes that go on, and those calculations need to be both very, very fast

86
00:18:34.970 --> 00:18:47.399
Igal Tabachnik: in terms of their execution. And also we have to be verifiably correct, which means that that when when the block that we create is going out to the network, we have to be able to

87
00:18:47.820 --> 00:19:11.519
Igal Tabachnik: ensure that it contains exactly what we said that it will contain. And this is where rusts, for example, type system makes certain guarantees that that ensures that the block will be created correctly and and verifiably. So. We have a bunch of tests around it. But also the type system ensures that the structs contain just the data that that is expected.

88
00:19:12.280 --> 00:19:30.020
Igal Tabachnik: And of course the performance because of it, because because rust is a very, very, it's a systems programming language in the end. So everything runs natively on the machine. There's no jit, there's no memory management. There's no garbage collection, so everything kind of runs very, very fast and

89
00:19:30.270 --> 00:19:35.999
Igal Tabachnik: in a way that that's expected of something that that sits in the network and deals with high traffic.

90
00:19:36.560 --> 00:19:39.359
Igal Tabachnik: The high deals with the high traffic essentially.

91
00:19:39.930 --> 00:19:42.810
Gabor Szabo: Was there any consideration or

92
00:19:43.050 --> 00:19:51.509
Gabor Szabo: so? Was there any consideration of using any other language, or it was so much given that that it it has. It was to be rushed

93
00:19:51.890 --> 00:19:54.480
Gabor Szabo: that you didn't even think about other language.

94
00:19:54.690 --> 00:20:05.739
Eli Shalom: So we did start with experimenting with other languages. So you, if surprisingly, we started with python, and we thought that we could just

95
00:20:05.980 --> 00:20:16.649
Eli Shalom: do all the heavy lifting on mathematical operations with numpy and stuff like that. But it was far from the reality and the reality. There needs to be a lot of

96
00:20:16.900 --> 00:20:29.679
Eli Shalom: real execution of code on Evm and Python will never get to the right performance. So this was an approach we had to ditch quite fast and the other competitor is go.

97
00:20:29.700 --> 00:20:58.249
Eli Shalom: So a lot of projects are based in Go, and also one of the largest block builders in ethereum, also implemented in go, and the reason we had to choose between these 2 is whether there is a main entity called the node, which holds all the information in locally for the ethereum network. So when building a block, there needs to be a lot of access to the storage of the network, and it has to be

98
00:20:58.250 --> 00:21:22.210
Eli Shalom: placed locally. And one of the things we realized that it's very efficient to write in the same language. And like the node we're walking with because they can share some code. They can share processes inside the same machine. And then they can share memory. So basically, the reason to choose rust was because the main node we're using is implemented in rust.

99
00:21:22.610 --> 00:21:29.960
Eli Shalom: So this leads a lot of the choices of companies to work in blockchain to work with rust because

100
00:21:29.970 --> 00:21:54.799
Eli Shalom: there's a main entity to work with. It can be communicated as a black box like in Http. Or it can. It has extensions where people can run things inside the same process of the node itself, and if we run our builder as close as we can to the node, and if we write it in last, and we can share the same memory, and then we can get the best in the best performance.

101
00:21:54.800 --> 00:21:58.282
Igal Tabachnik: Yeah. And also there are additional benefits. For example,

102
00:21:58.820 --> 00:22:20.249
Igal Tabachnik: the the node itself. It's called rus, RETH. So it's a rust implementation of the ethereum network. It's also open source, and it introduces a lot of libraries that we use internally, that provide you with primitives for the system, everything from holding, describing sums of money.

103
00:22:21.380 --> 00:22:24.480
Igal Tabachnik: and yes, exactly. RATH. Reth.

104
00:22:24.480 --> 00:22:29.280
Gabor Szabo: I just wrote the Free People listening to us. I just wrote it in the chat because I wasn't sure.

105
00:22:29.340 --> 00:22:29.990
Igal Tabachnik: Yeah, yeah.

106
00:22:30.247 --> 00:22:33.339
Gabor Szabo: And the. And so I'm going to put it on in the.

107
00:22:33.340 --> 00:22:40.419
Igal Tabachnik: In fact, I made a bunch of links of things that we're going to mention, and maybe we can put them in the end as like a show notes.

108
00:22:40.630 --> 00:22:42.520
Gabor Szabo: That would be a very good idea. Thank you.

109
00:22:42.960 --> 00:23:08.489
Igal Tabachnik: So, yeah, the idea is that a lot of libraries that can be reused like with primitives, with abstractions, with, for example, if the ethereum network implements, a lot of protocols which are all published. They're all kind of described in white papers. It's all fascinating. Everything is kind of documented, and there are a bunch of different implementations, and the most popular ones are in go and rust.

110
00:23:08.530 --> 00:23:23.440
Igal Tabachnik: And so the the rust implementations are really, they're very robust, and they're very, I would say, idiomatic to the rust language. So everything works as you would expect a real rust code to work. So, for example.

111
00:23:23.710 --> 00:23:46.129
Igal Tabachnik: you have a bounce checking, and you have smart macros. So, for example, you have even things like addresses. If you want to send, like everything in ethereum is like an address from and to. So you have to know who you're sending from and who you're sending to, whether it's money or anything else. And this is just a big hacks number.

112
00:23:46.290 --> 00:24:03.049
Igal Tabachnik: And so, in order for you not to spend any time parsing strings or just dealing with general validation of whether or not you were given a correct address. You just use a macro, for example, there's a macro in these building blocks called address bang.

113
00:24:03.050 --> 00:24:20.959
Igal Tabachnik: which in compile time validates that you that you are using correct address. So it alleviates a lot of headaches and a lot of kind of these small mistakes that you can get with languages that are not as robust. Probably for dealing with this like, you can probably implement

114
00:24:21.020 --> 00:24:25.869
Igal Tabachnik: any of these things in any other language, even like, obviously people do it in go.

115
00:24:25.920 --> 00:24:41.199
Igal Tabachnik: But the added benefit of rust from my perspective, as somebody who uses daily is that all of these little patterns that the language encourages you to do like going through safe methods and and implementing safe conversions

116
00:24:41.230 --> 00:25:00.830
Igal Tabachnik: ensures that you're less likely to make a mistake, especially when you're building payloads and parsing payloads at Runtime. You're more or less guaranteed that your execution will be flawless as long as you handle everything during compilation during design time of your program.

117
00:25:01.040 --> 00:25:12.020
Igal Tabachnik: And this is again. This is one of the primary reasons why, in the end we chose rust is that there's this added guarantee of safety that the language is built upon.

118
00:25:12.430 --> 00:25:16.090
Igal Tabachnik: It provides you with a lot of a lot of

119
00:25:16.220 --> 00:25:22.039
Igal Tabachnik: functionality, so to speak, that ensures that many, many of these things

120
00:25:22.470 --> 00:25:36.849
Igal Tabachnik: the compiler will handle for you, and if you're making a mistake, the compiler will aid you and tell you what exactly you're doing wrong, and so you will have much fewer runtime surprises, which is, in my opinion, is the the best possible thing that rost can do.

121
00:25:38.020 --> 00:25:52.429
Gabor Szabo: Right back to a little bit for my understanding. You mentioned Node. That there are there are these nodes, isn't network is is just these notes. So when I say when you say the at the room at the room.

122
00:25:53.190 --> 00:25:54.500
Igal Tabachnik: That. Yeah.

123
00:25:54.620 --> 00:25:57.690
Gabor Szabo: When you say at the room, it's basically just the notes right.

124
00:25:57.690 --> 00:25:58.280
Eli Shalom: Yes.

125
00:25:58.550 --> 00:26:08.170
Gabor Szabo: Okay, so and so the the and it's an open as much as I understand. That's an open source project, basically right? Written in rust.

126
00:26:09.080 --> 00:26:11.030
Gabor Szabo: So all these notes are written in rust.

127
00:26:11.420 --> 00:26:13.819
Eli Shalom: So there are many implementations of the nodes.

128
00:26:13.820 --> 00:26:14.630
Gabor Szabo: Okay.

129
00:26:14.630 --> 00:26:17.190
Eli Shalom: The most popular ones are in. Go and rust.

130
00:26:18.070 --> 00:26:25.150
Gabor Szabo: Okay? So the notes. So also there are different implementation of nodes. Not just the the block builders.

131
00:26:25.150 --> 00:26:30.219
Eli Shalom: Yeah, there, there could be infinite implementations for the for the for the nodes.

132
00:26:30.220 --> 00:26:35.350
Gabor Szabo: Okay. And then I guess there is some Api or some definition that that they must

133
00:26:35.640 --> 00:26:37.890
Gabor Szabo: and work this way.

134
00:26:38.280 --> 00:27:01.370
Igal Tabachnik: There's a spec basically on ethereum.org. You have the entire spec. All the white papers of many smart people wrote many years ago, and all of these things are just implementations, and yes, they have to agree on a common protocol and a common language. Essentially. In the end your payloads are encoded in. There's a special type of encoding.

135
00:27:01.940 --> 00:27:06.569
Igal Tabachnik: There's a special type of encoding of how the actual raw data looks

136
00:27:06.790 --> 00:27:16.910
Igal Tabachnik: on the on the wire, but every every implementation, whether it's go or or rust, or anything else. Really, I've seen. I've seen people do it in Javascript as well, of course.

137
00:27:17.200 --> 00:27:19.749
Igal Tabachnik: because everything can be done in Javascript.

138
00:27:20.320 --> 00:27:24.680
Igal Tabachnik: They all are able to to handle it and and talk to each other.

139
00:27:25.150 --> 00:27:50.189
Gabor Szabo: So how do? How do they manage upgrades? I mean, if 1 1 of these notes so if they want to introduce a new service, or I don't know whatever new something into the network. Then someone implements it in one type of the notes, and and then the others don't know about it so, or they are not implemented yet. So not not only talking about, not even

140
00:27:50.470 --> 00:27:52.530
Gabor Szabo: not talking about

141
00:27:53.060 --> 00:28:03.820
Gabor Szabo: rolling out to the same code base, to maybe hundreds or thousands of nodes. Which itself is a is a is a problem. But if they're each node might be in different

142
00:28:04.410 --> 00:28:06.589
Gabor Szabo: might be a different code base.

143
00:28:07.420 --> 00:28:18.589
Gabor Szabo: Then how can they make make progress? Or what happens if what I mean? Okay? So you add the new feature, then it can't work yet, because the others

144
00:28:19.010 --> 00:28:20.850
Gabor Szabo: don't understand it yet.

145
00:28:21.810 --> 00:28:44.400
Eli Shalom: Yeah. So on the network, nobody can add a new feature on their own, and the idea for which there are many nodes is to get to a majority, and the right thing is always what the majority says. So if you have just one node which decides to deviate from the others, and this node will be disregarded, because it will always be the minority, and nobody will

146
00:28:44.550 --> 00:28:52.350
Eli Shalom: take it seriously. Usually this node will get punished, and and the reputation will be harmed or slashed. I'm not exactly sure how they.

147
00:28:52.460 --> 00:29:02.120
Eli Shalom: being incentivized not to do it, but a node can't, can decide on it on its own, to change the behavior and.

148
00:29:02.570 --> 00:29:03.020
Gabor Szabo: Behind.

149
00:29:03.910 --> 00:29:04.670
Eli Shalom: Yeah, but.

150
00:29:04.670 --> 00:29:11.999
Gabor Szabo: Right. If if all of them are moving ahead to something, then then I also have to move, because otherwise I'll be different right?

151
00:29:12.000 --> 00:29:36.389
Eli Shalom: Definitely so the idea of the majority holds all the time. So once a big change needs to come to ethereum. Usually it comes from the ethereum foundation, and they promote it through something called eap. It's an improvement proposal for ethereum. It takes very long time to get to an agreement, and once it's done and the spec of the nodes

152
00:29:36.390 --> 00:29:42.940
Eli Shalom: changes, and then the community decides on a date in which all the nodes have to change to the new version.

153
00:29:43.950 --> 00:29:44.620
Gabor Szabo: Hmm.

154
00:29:44.620 --> 00:30:12.940
Igal Tabachnik: Yeah. So the idea? Yes, this is the protocol governed by the ethereum foundation and all the major upgrades to this to the way the protocol works has to be documented, agreed upon, discussed potentially for many years, until they finally pick up a code name which is usually a weird name of some sort, or maybe like a city. There was, I think there was something called London. Maybe I don't remember exactly. And yeah, they decide to switch.

155
00:30:13.040 --> 00:30:31.269
Igal Tabachnik: And so yeah, so those things don't just nobody can really add a feature to the ethereum network because it has to be. It's like, it's like, I don't know. It's like would be adding a new feature to the Internet that would not make real like to Tcp, IP. It would have to go probably through.

156
00:30:31.610 --> 00:30:49.609
Gabor Szabo: Yeah, yeah, yeah, those are the standards. Yeah. So you have to go with the standard. I mean, you could add the new service to the Internet, and you start doing with between 2 computers. But if if you want to change how the Tcp IP protocol works well, that good.

157
00:30:49.610 --> 00:30:50.100
Igal Tabachnik: Live.

158
00:30:50.490 --> 00:30:54.700
Gabor Szabo: Or how even what Http is, or whatever yeah, right?

159
00:30:54.700 --> 00:30:55.710
Igal Tabachnik: Yeah, yeah.

160
00:30:57.385 --> 00:30:58.580
Gabor Szabo: So

161
00:30:58.730 --> 00:31:12.979
Gabor Szabo: what else? I wanted to ask something else about this? Oh, yeah. Yeah. How many notes are there? This is something that I was. How many notes are there in the network like more or less? It's thousands or millions or.

162
00:31:13.360 --> 00:31:31.160
Eli Shalom: No, it's it's a lot. There are 1 million registered nodes which perform validation. It's something which forces them to be up and to be like a very active node. So there are just 1 million nodes of validators, and there are many 1 million of other nodes

163
00:31:31.300 --> 00:31:37.309
Eli Shalom: which are just out just collecting data, storing them locally like we do. So it's a few millions.

164
00:31:37.700 --> 00:32:04.510
Gabor Szabo: Okay. So the point is, or when I started to learn a little bit about about blockchain. And then I understood, okay, there's a majority decides. That's the point that more or less where I reached in my understanding. And then I was wondering, okay, fine majority decides. But if there is a wealthy person or a country that wants to overtake this whole thing. Then they can build.

165
00:32:04.730 --> 00:32:12.970
Gabor Szabo: set up a lot of these machines, and I was wondering how, how big, how big that needs to be in order to become a majority.

166
00:32:13.370 --> 00:32:18.000
Gabor Szabo: So I guess if it's a couple of millions, then then it's a lot.

167
00:32:18.350 --> 00:32:23.539
Gabor Szabo: And I guess these are not simple machines, right? So they they

168
00:32:23.640 --> 00:32:26.500
Gabor Szabo: very. They need to be very powerful machines, right?

169
00:32:26.870 --> 00:32:27.750
Gabor Szabo: The notes.

170
00:32:28.107 --> 00:32:57.040
Eli Shalom: They're not like the smallest or cheapest machines, but it's not like a like Aws machines of 5,000 bucks a month. Usually it's like medium sized machines with a lot of storage like holding the node takes a few terabytes of data, which is usually where most of the cost comes. And this is what makes it also very, very challenging to spin up a node. Take about 48 to 72 h until it becomes synced until it

171
00:32:57.602 --> 00:33:01.730
Eli Shalom: takes all the information from other nodes and verifies it.

172
00:33:02.530 --> 00:33:03.130
Gabor Szabo: Okay.

173
00:33:03.170 --> 00:33:15.490
Igal Tabachnik: Because because also, yeah, you have to remember that the the idea behind the blockchain in general is because it is immutable. Everything that happened is historical and cannot cannot ever change, because it was

174
00:33:15.490 --> 00:33:24.099
Igal Tabachnik: it was reached like a consensus was reached that this node, this this block, was placed on the network, and it will be there forever.

175
00:33:24.100 --> 00:33:46.479
Igal Tabachnik: In order for you to modify, you have to create a new one. And so basically, the latest version will be the current, but historically you can always look ahead, and every time you need to spin up a new server or a new node, or whatever you have to fetch all of the historical data from you have to pre-populate it. And this is what that means that it takes a lot of time and a lot of storage.

176
00:33:46.670 --> 00:33:54.349
Gabor Szabo: So the the existing chain that there is all the the chain of the blocks that already have. That's a couple of terabyte of data already.

177
00:33:54.770 --> 00:33:55.400
Eli Shalom: Yep.

178
00:33:56.420 --> 00:34:03.099
Gabor Szabo: And how long that has accumulated like 10 years. Right, I think, at the room, at the wrong.

179
00:34:03.509 --> 00:34:05.660
Eli Shalom: I would say probably. Yes, but I'm not sure.

180
00:34:06.120 --> 00:34:25.689
Gabor Szabo: So I was wondering, okay, it's just going to grow more and more. And then, okay, a lot of disk space is going to be used. My understanding is that basically, this is very similar to what? How the data of git looks like, except, of course, without branching.

181
00:34:26.370 --> 00:34:33.260
Gabor Szabo: Now, the ability to go back in storage.

182
00:34:33.860 --> 00:34:34.730
Gabor Szabo: Sorry.

183
00:34:34.730 --> 00:34:38.080
Eli Shalom: We don't have the way to go back like in in git. So versioning.

184
00:34:38.080 --> 00:34:46.250
Gabor Szabo: So indeed you could you could you? Could you could remove all the recent commits and and whatever here we I mean

185
00:34:47.020 --> 00:34:52.670
Gabor Szabo: the same here. You could do this if you could convince all of enough notes right.

186
00:34:52.670 --> 00:34:52.975
Eli Shalom: Yep.

187
00:34:53.750 --> 00:34:57.129
Gabor Szabo: Basically. And okay.

188
00:34:57.570 --> 00:35:07.400
Gabor Szabo: interesting. So let's get back to rust a little bit. And and you, so how did you get started with rust?

189
00:35:09.780 --> 00:35:11.510
Igal Tabachnik: Do you mean me, or Ellie?

190
00:35:11.740 --> 00:35:13.870
Gabor Szabo: One after another.

191
00:35:14.580 --> 00:35:22.220
Igal Tabachnik: All right. I think I'll start then. So, like I mentioned earlier, my my journey to Rust was a bit different. So over the past.

192
00:35:22.360 --> 00:35:52.110
Igal Tabachnik: I want to say. Almost 10 years I've been dealing with well, dealing. Dealing is a choice word. I've been working primarily with Scala and through Scala, which is a language on the Jvm kind of it's this hybrid, object-oriented, functional language I discovered functional programming and functional programming became something that I really fell in love with, and so I later learned Haskell and other kind of languages that belong to that family.

193
00:35:52.630 --> 00:36:20.429
Igal Tabachnik: and through functional programming I discovered many of the benefits that kind of were pioneered in functional languages, and then picked up by every other programming language in the world. So, for example, things like virtual threads or green threads, as they're known in other languages. They were 1st introduced in Haskell, and then they were pioneered in Scala, and now Java has them, and of course, rust have them in

194
00:36:20.430 --> 00:36:24.529
Igal Tabachnik: basically Tokyo is the implementation of virtual threads

195
00:36:24.680 --> 00:36:44.340
Igal Tabachnik: and the whole notion of immutability and working with immutable data and transformations, like in C sharp, they would be called link. In other languages they would be called working with collections and lambdas and iterators. There was kind of all of these features were kind of

196
00:36:44.540 --> 00:36:57.339
Igal Tabachnik: I wouldn't say pioneered, but they were heavily, heavily used in functional languages primarily, because you don't have as many variables and and and mutation going on. So you have chains and transformations of data.

197
00:36:57.820 --> 00:36:58.973
Igal Tabachnik: And so

198
00:37:00.680 --> 00:37:25.849
Igal Tabachnik: unfortunately, I want to say, like, I'll be frank, that unfortunately, Scala is notoriously hard to teach because of that, because you can use it as a kind of a Java with this different syntax. But this does not really give you that much of a benefit, especially today when you have languages like Kotlin, or other things that work on the Jvm. That don't necessarily require the full power of a language like Scala.

199
00:37:26.060 --> 00:37:34.979
Igal Tabachnik: and so many, many companies that were previously using Scala. They're kind of not really starting new projects on that. So they're kind of moving away.

200
00:37:35.020 --> 00:37:59.460
Igal Tabachnik: And so I was actually looking to do some more Scala, and it was becoming challenging. And so my options were either to go kind of into the mainstream and do maybe a little bit of Kotlin, or even typescript. And there are not a whole lot of choices for doing backend engineering, but most of it revolves today around

201
00:37:59.460 --> 00:38:05.950
Igal Tabachnik: Java and Kotlin and typescript on the back end. There's some python, probably.

202
00:38:06.180 --> 00:38:35.609
Igal Tabachnik: or I really wanted to look at this language called rust, because what I knew of rust is that it was yes, it was kind of a type, safe, very performant language that was created by Mozilla for a certain purpose. But what I knew of rust is that it was designed in a way that it borrowed a lot of features from functional languages. In particular. If you look at the way that rust is designed, it borrows a lot of ideas from, for example, from Haskell.

203
00:38:35.630 --> 00:38:40.380
Igal Tabachnik: which is which is known to be like a pure functional language.

204
00:38:40.460 --> 00:39:10.010
Igal Tabachnik: But in rust, for example, the derivation mechanism. So the way that you attach essentially behavior to structs with the derived keyword. This is an exact copy of a feature from Haskell. It's called type classes, but this is a feature that was that exists in Haskell and was adapted to many other programming languages. And, in my opinion, this is one of the most amazing features that rust has, because it allows you to precisely define behavior to a set of

205
00:39:10.010 --> 00:39:17.199
Igal Tabachnik: a set of structure, a set of traits, essentially, traits become behaviors, and you can apply them

206
00:39:17.640 --> 00:39:30.200
Igal Tabachnik: externally to the data. So, whereas you have in many object oriented languages, you are encouraged to kind of create behavior and data together in the same class.

207
00:39:30.580 --> 00:39:51.219
Igal Tabachnik: in function, languages in rust. Specifically, you are encouraged to keep them separate. So you have structs that hold just the data, and you have traits and implementations of those traits as behaviors which can be attached and reattached and and defined, and most importantly verified at compile time.

208
00:39:51.790 --> 00:40:08.670
Igal Tabachnik: So 1 1 such example is that something I've encountered yesterday. So I was refactoring a piece of code, and I needed to store 2 new fields, or at least one new field of type float. I needed the floating point number to be stored in a configuration somewhere.

209
00:40:09.040 --> 00:40:16.560
Igal Tabachnik: I got a compilation error that said I cannot use, derive Eq. Derive equality

210
00:40:16.650 --> 00:40:40.509
Igal Tabachnik: for a type F. 64. And this makes complete sense. You cannot. Computers do not like comparing floating point numbers. This is an impossible problem. Now, there are many, many programming languages that are happily going to let you compare 2 floating point numbers, and the end result will probably not be what you need. What you expect, so you will have some issue with Runtime.

211
00:40:40.620 --> 00:40:52.500
Igal Tabachnik: but Rust was smart enough to figure out that you cannot derive equality for the entire config class, because the float type F, 64 does not implement the IC trait.

212
00:40:52.710 --> 00:41:02.510
Igal Tabachnik: And this is why you get a compilation error, and it makes perfect sense. And the point is that in rust I found that many, many of these things are like this.

213
00:41:02.840 --> 00:41:03.675
Igal Tabachnik: So

214
00:41:04.520 --> 00:41:19.460
Igal Tabachnik: I'm just going to say that that people who are coming to Russ. They hear stories about this compiler and the borrow checker being in your way constantly, and you and you have to kind of fight the borrow checker, so to speak, to, to get

215
00:41:19.460 --> 00:41:35.550
Igal Tabachnik: to get to the point. But I think that's the wrong way of looking at it. I think the compiler is there obviously to prevent invalid behavior invalid programs undefined behavior. So this is one of the core principles of rust is to be safe. The safety is achieved by

216
00:41:35.960 --> 00:41:39.689
Igal Tabachnik: it's preventing you from doing things that will obviously not work.

217
00:41:40.240 --> 00:41:58.120
Igal Tabachnik: and many, many smart people created it. And I've seen many talks by the creators of Rust, by Grayden Hoare, who talks about the kind of the history and why they did things the way that they did. But the main idea is that rust borrows many of those features from

218
00:41:58.310 --> 00:42:08.630
Igal Tabachnik: languages that have strong and static type systems that kind of are in your way. But in order for you to to prevent you from doing kind of the wrong thing.

219
00:42:09.390 --> 00:42:35.839
Igal Tabachnik: And finally, I'm going to say that one such mention about the borrow checkers. This is something I heard from a friend who does not do any rust. He's a scholar programmer. But he asked me, don't you get to fight the borrow checker a lot, and I would say that this notion of fighting the borrow checker is kind of a meme. It's kind of a myth in the rust world, because it is being sold as this kind of

220
00:42:36.140 --> 00:42:49.220
Igal Tabachnik: this kind of novelty feature that the rust compiler has. It has this notion of a borrow checker. But the idea there is that because it has manual memory management, you don't want to be doing things that are wrong.

221
00:42:49.440 --> 00:43:10.099
Igal Tabachnik: So if you want to pass a mutable variable to another function. You have to explicitly allow it to to be to be passed as mutable because somebody can modify it under your feet. And the scholar, the rust compiler is smart enough to detect such cases and tell you, no, you're doing something very, very wrong.

222
00:43:10.280 --> 00:43:12.960
Igal Tabachnik: and so it's not there to to kind of

223
00:43:13.640 --> 00:43:22.490
Igal Tabachnik: prevent you from doing stuff. It's there to guide you of saying, Okay, I know what you're trying to do, but there are ways of doing it, and you better, so better stick to the ways.

224
00:43:22.640 --> 00:43:33.460
Igal Tabachnik: And finally, the last kind of it's more of a philosophical thing that I'm going to say is that there's this notion again, coming from functional languages. It's it's a thing that's called.

225
00:43:33.900 --> 00:43:50.620
Igal Tabachnik: It's based on a very good talk that I watched many years ago. It's this notion of constraints, liberate and liberties constrain. So the idea is that if you don't have any constraints and you can do just whatever you want like in in

226
00:43:50.670 --> 00:44:13.329
Igal Tabachnik: other programming languages. Javascript. So if you don't have any constraints whatsoever, so you can do just whatever you want, which means you can do it in any way that you want in any style that you want, in any order that you want, probably. And then what you have is down the line. You will have problems. You will have runtime issues. You will have refactoring challenges, and you will have many, many things that we know and love about

227
00:44:13.420 --> 00:44:14.949
Igal Tabachnik: to languages like Javascript.

228
00:44:15.560 --> 00:44:31.210
Igal Tabachnik: Now, if you have a system that's constrained, it does not let you do what you want, but it rather expects you to follow its exact kind of guidelines and its exact path that it guides you through, that there are very, very few ways in which you can implement something. So

229
00:44:31.800 --> 00:44:46.129
Igal Tabachnik: this notion of of constraints. Liberate is is that when you are constrained kind of it sounds paradoxical, but when you are constrained by by the the system that you are building it by the type system, by the the borrow checker or whatnot.

230
00:44:46.350 --> 00:44:52.860
Igal Tabachnik: You are then free to kind of implement it in the only way that will work. And so you don't have to worry about that.

231
00:44:53.200 --> 00:45:05.420
Igal Tabachnik: And that's for me. That's just one of the the most tremendous benefits of using a language like rust, and also I'll have to give credit where credit is due. Also languages like Scala and languages like

232
00:45:05.730 --> 00:45:21.399
Igal Tabachnik: Haskell, of course, and there are some some other languages that on the Microsoft side there's F sharp. Many, many functional languages embrace this principle, and, of course, type system really helps. So if you don't have types to guide you. You have to write a lot of tests.

233
00:45:21.520 --> 00:45:48.959
Igal Tabachnik: and we can debate whether or not writing tests is a fun activity. Now that ais can do it for us. But I would say that types from my perspective come before tests. They can prevent many, many of invalid behaviors even making writing these tests unnecessary for things that cannot possibly compile. Okay, and with that I'm going to stop talking for now. And, please, if you have any, follow ups, I would.

234
00:45:48.960 --> 00:45:58.789
Gabor Szabo: Yeah. So before letting Ellie also say, I wanted to ask you, how did you solve that problem with the floating point, comparing floating numbers?

235
00:45:58.790 --> 00:45:59.520
Gabor Szabo: Oh.

236
00:45:59.670 --> 00:46:21.149
Igal Tabachnik: Well, well, the point is, I needed to store a percentage a multiplier for a percentage. I ended up storing it as just as just a decimal number of, I just switched it to a U to U, 64, I used a different multiplier, essentially. But yeah, it's just one of these things.

237
00:46:22.280 --> 00:46:23.040
Gabor Szabo: Okay.

238
00:46:23.360 --> 00:46:24.090
Gabor Szabo: Good.

239
00:46:25.170 --> 00:46:32.530
Igal Tabachnik: So I changed. Yeah, I changed the data type. Essentially, I didn't try to hack around the problem. I just, I solved it using a different data type.

240
00:46:35.920 --> 00:46:56.269
Eli Shalom: So I guess my version is going to be a little bit more dull, but it's going to go through the same main theme of the language guiding us towards the right thing. So when we started working with Russ, it was the 1st time we coded in the language, and both my partner and I didn't code.

241
00:46:56.340 --> 00:46:57.240
Eli Shalom: and

242
00:46:57.290 --> 00:47:18.670
Eli Shalom: in that intensity for a very long time, especially me as a data science, I didn't do such things for a very, very long time. And when we started working on the software it was something which did a lot of things in parallel. We got a lot of data from many sources in parallel. We had to do a lot of computations in parallel and store them back to the same sources in parallel.

243
00:47:18.670 --> 00:47:28.339
Eli Shalom: And one of the things we started feeling at the beginning is that we're fighting the Borough checker all the time, and that it's just interfering with our work.

244
00:47:28.450 --> 00:47:46.360
Eli Shalom: Eventually, like probably something which happens to most rust developers. We started to figure out what's the right thing to do when we need to clone an object when we have to show it with an arc, and when we have to use a lock to show the object between different components

245
00:47:46.360 --> 00:47:58.600
Eli Shalom: and one of the things which really impressed us after the 1st month was that it was a very large system in regards to the amount of parallelization it did, and it had no locks at all

246
00:47:58.730 --> 00:48:19.469
Eli Shalom: in any software, in any other language I use. I was going to write and use a mutex in many places to to synchronize, and here, because the language just encourages the developers to think about who owns each object, it just led us to write it in a way which didn't require any lock.

247
00:48:19.470 --> 00:48:29.090
Eli Shalom: and the performance was great, not just because it was rust, also because it had no locks, and we didn't have to wait in any place which we wouldn't have happened

248
00:48:29.520 --> 00:48:31.859
Eli Shalom: in probably any other language we used.

249
00:48:33.460 --> 00:48:40.839
Gabor Szabo: Very interesting, very interesting. Okay, so let's go further with Rust. You said that

250
00:48:41.020 --> 00:48:48.880
Gabor Szabo: part of the reason you selected rust is because, or part of the advantages that you have from rust is that you have a lot of libraries

251
00:48:49.010 --> 00:48:56.080
Gabor Szabo: crates that you can already use because they are part of the the node. Basically right?

252
00:48:56.230 --> 00:48:58.810
Igal Tabachnik: So those are given.

253
00:48:59.070 --> 00:49:13.529
Gabor Szabo: That are part of the node, and you're going to use it. And and whatever I mean, even even there might be, the question is, is valid, but I'm quite sure that you have also all kind of other crates that you are using. I mean, every rust project uses hundreds of crates.

254
00:49:13.530 --> 00:49:13.930
Igal Tabachnik: Right.

255
00:49:13.970 --> 00:49:21.484
Gabor Szabo: How do you? How do you? How do you? How do you select the create? How do you make sure?

256
00:49:22.500 --> 00:49:23.889
Gabor Szabo: it's quality?

257
00:49:24.050 --> 00:49:30.290
Gabor Szabo: Yeah. Well, that's a very good question. I will say that at least until now.

258
00:49:30.560 --> 00:49:57.839
Igal Tabachnik: Because because the the platform itself that we that we are basing upon it was already kind of it, more or less, batteries included. It has everything that we would need to to kind of build upon upon it. It's an SDK, essentially so. I I don't remember if we needed to add an external crate other than possibly, I think it was like

259
00:49:58.190 --> 00:50:24.179
Igal Tabachnik: some data types for that have to do with with ethereum protocols like we, we needed to to bring in one more protocol that we needed to support, and I think there was a crate for that. But but it's just a matter of you know, Googling, for rust, and whatever it is you're looking for and looking at top. 2 results. Now, I would say that most of the the crates that are used. They are used mostly by reputation.

260
00:50:24.440 --> 00:50:42.159
Igal Tabachnik: so I wouldn't say necessarily number of stars. But it's more like a word of mouth thing that if you. For example, if you're building a cli tool and you need a command line argument, parser. So you would probably use clap. Why? Because everybody uses clap. And this is like in every book and every tutorial.

261
00:50:42.540 --> 00:50:57.739
Igal Tabachnik: And so, if you need, obviously, if you need a synchrony, you would use Tokyo, and I know that there are other implementations, but they are maybe not as not as as powerful, or maybe not as as popular.

262
00:50:57.970 --> 00:51:16.860
Igal Tabachnik: Maybe they have their uses, but Tokyo seems to be kind of the I wouldn't say Standard Library, but it's kind of the de facto thing that everybody else uses, and of course you need to think about interoperability. So to answer the question, I would say, if we needed to, we would probably choose it by reputation. So

263
00:51:17.120 --> 00:51:41.230
Igal Tabachnik: how popular it is, whether or not it's registered in the official website. Can I even pull it from like? Can I even add it to my toml file and just rebuild and everything will compile. Or do I need to add some 3rd party, github Urls to my, to my configs, to pull it from source stuff like that.

264
00:51:41.230 --> 00:51:44.499
Gabor Szabo: You encounter encounter Chris like that, I mean I.

265
00:51:44.500 --> 00:51:49.170
Igal Tabachnik: But I suppose that there are private implementations somewhere that do not are not published.

266
00:51:49.340 --> 00:51:53.340
Igal Tabachnik: I, personally haven't encountered that yet, but I'm.

267
00:51:53.340 --> 00:52:04.929
Gabor Szabo: Item number that I encountered is that that you can't just Pip installs the latest, or from single app only. But you have to go to that there I encountered. But maybe that's fortunately, because

268
00:52:05.550 --> 00:52:09.919
Gabor Szabo: I don't know, because Python is older and more and.

269
00:52:09.920 --> 00:52:18.490
Igal Tabachnik: And rusted becomes very easy, because version pinning is very important, like you have the log files, and and I think I think

270
00:52:18.760 --> 00:52:36.550
Igal Tabachnik: from many like package management software that I've used over the years. I think Ross did it really great because it it kind of works. And and it tells you what the latest versions are. It figures out mismatched versions between like, if you have dependencies between crates.

271
00:52:37.540 --> 00:52:51.030
Igal Tabachnik: the rust will tell you, and enabling features independently. So if you, for example, you needed to enable Tokyo for a certain crust, if it supports it, then they have certain crate.

272
00:52:51.820 --> 00:52:52.710
Igal Tabachnik: I think.

273
00:52:53.560 --> 00:53:17.999
Igal Tabachnik: like I think, that they did a really good job, like rust, in my opinion, is one of the most well engineered languages that provides you with developer productivity. There are very few like the only one I can possibly think of that that does at least as good job is a language called elm, which is a web language. That's also kind of functional, and it also has its roots in

274
00:53:18.000 --> 00:53:28.659
Igal Tabachnik: Haskell, like syntax, but its package management and its errors are just amazing. It will tell you exactly what happened and how to solve it.

275
00:53:28.710 --> 00:53:32.250
Igal Tabachnik: and, in fact, kind of jumping is an aside.

276
00:53:32.470 --> 00:53:47.559
Igal Tabachnik: Rust's errors are notoriously yes, they can be overwhelming sometimes, but they're notoriously so detailed is that you have AI tools today that just figure all the errors out and explain to you what went wrong

277
00:53:47.560 --> 00:54:09.469
Igal Tabachnik: and just fix it for you. This is something I encounter daily, even if I don't want it. Sometimes I do encounter an issue with borrowing or stuff like that, or forgetting to clone something. So sometimes I want to get my task done. So I don't really want to think about it. So I just press the little icon

278
00:54:09.470 --> 00:54:27.019
Igal Tabachnik: in my intelligent, it tells me fix with AI, and it just does. Why? Because it has a lot of context for what went wrong, and the Llms are now smart enough to kind of feed this context. Read this context and just figure it out and give me all the information. So I don't have to Google. I don't have to read docs.

279
00:54:27.160 --> 00:54:31.639
Igal Tabachnik: It's it's it's it's really an amazing flow that

280
00:54:32.450 --> 00:54:39.589
Igal Tabachnik: it's just incredible to me. I've been doing software for many years now, and and I don't remember anything remotely

281
00:54:39.920 --> 00:54:45.259
Igal Tabachnik: advancing in this in this incredible rate, like like AI and AI, assisted tools.

282
00:54:46.620 --> 00:54:49.220
Gabor Szabo: About the AI. Let's.

283
00:54:50.300 --> 00:54:51.550
Igal Tabachnik: It was topic to topic.

284
00:54:51.550 --> 00:54:52.030
Igal Tabachnik: Yep.

285
00:54:52.950 --> 00:54:55.389
Gabor Szabo: No, no, I'm I'm glad we're talking about AI. That's it.

286
00:54:55.470 --> 00:55:08.559
Gabor Szabo: Okay. So I wanted to ask about AI, if you're already talking about this, how much do you use? AI, which AI tools do you have you experienced? With which one do you feel the best

287
00:55:09.740 --> 00:55:13.190
Gabor Szabo: for? Especially for us? Development? Of course.

288
00:55:13.620 --> 00:55:15.750
Igal Tabachnik: Sure, Eddie, you want to go first.st

289
00:55:15.750 --> 00:55:17.450
Eli Shalom: No, no, that's yours.

290
00:55:17.740 --> 00:55:41.310
Igal Tabachnik: So I will kind of make a jab at Ali that he's the he's the AI person in the company, because the amount of time he spends explaining things to Chatgpt. It's amazing. But I will mention this from from my perspective, as somebody who's like a developer that's being introduced to AI assisted tools. So

291
00:55:42.200 --> 00:55:53.800
Igal Tabachnik: I'll skip to the end. Today I'm preferring to use mostly coding agents, such as Claudecod, and recently a new tool called Amp. From

292
00:55:54.180 --> 00:55:56.380
Igal Tabachnik: I always forget the name

293
00:55:57.030 --> 00:56:22.470
Igal Tabachnik: sourcegraph, which is a company that also they make products for understanding and analyzing and navigating code a lot so. But now they came up with their own agentic. AI. And this is something that's really that took an incredible kind of it exploded with incredible rate in the past couple of months, really from my perspective.

294
00:56:22.710 --> 00:56:34.319
Igal Tabachnik: But to to go back to AI 1st when Google's co-pilot was introduced. Yes, it was nice. It was kind of a predictive code completion thing. So

295
00:56:34.320 --> 00:56:55.570
Igal Tabachnik: many people enabled it, as long as they didn't have to pay for it. So after it became a paid product, some people stayed with it. Some people didn't. I had value from it, because when I was working with Scala it helped me to fill in the boilerplate. Scala does have some boilerplate that you have to do, and so I paid out of my own pocket

296
00:56:55.690 --> 00:57:00.830
Igal Tabachnik: initially for Github copilot, just to keep it working

297
00:57:01.230 --> 00:57:14.799
Igal Tabachnik: and to keep churning up this boilerplate. Eventually we switched to a company subscription, and then the tools became smarter and smarter, and I started to realize that they do

298
00:57:15.040 --> 00:57:34.220
Igal Tabachnik: understand your code better every time. So they keep this context. I needed an explanation from the Resident AI expert, Ellie. He needed to explain to me how what are tokens and how all of this stuff works. And it's it's really, truly incredible how Llms essentially are built.

299
00:57:34.960 --> 00:58:04.160
Igal Tabachnik: But the idea, the idea there is that the more context you have, the more code of your own you can shove into the AI the better results the better predictions you're going to have. And predictions are what AI's do. They do not understand the code themselves. They understand what is the next likely token to emit, so most likely they will emit the correct thing. And so over the months over the last, I don't know. Maybe a couple like

300
00:58:04.210 --> 00:58:10.089
Igal Tabachnik: I want to say 5 or 6 months that I've been using AI with rust, I see much less

301
00:58:10.370 --> 00:58:31.970
Igal Tabachnik: almost none of hallucinations that typically come with with using Llms. So, for example, Llms, like even like copilot, or or even tools from Chatgpt. They don't necessarily. When they don't know something they will just make something up. They will make Api that does not exist, and they will. They will just suggest you to do things that that are just

302
00:58:32.080 --> 00:58:33.859
Igal Tabachnik: impossible, for example.

303
00:58:34.080 --> 00:58:46.399
Igal Tabachnik: And since switching, trying different models, and and lately I've been using Cloud's model a lot sonnet 3.7. And then. Now last week they released 4.0.

304
00:58:46.940 --> 00:59:10.759
Igal Tabachnik: And so ever since switching to that, I've been seeing, much less hallucinations. And it really does solve the issues. And also in a way that's straight to the point. No, not too many explanations. But then they just give me actionable solutions to, for example, compilation errors, or sometimes they just want to. For example.

305
00:59:10.940 --> 00:59:38.189
Igal Tabachnik: let's see, I just, for example, even I want to to create a struct that derives whatever debug and clone it. Just sometimes, instead of typing out the struct the code, I will just type create a struct for whatever deriving all these things, and it will just think for 2 seconds and just do it for me. So yes, I find AI used to be an incredible assistant, and I do think that we are at least in the beginning of

306
00:59:39.150 --> 00:59:42.900
Igal Tabachnik: I don't even I don't even want to say paradigm shift, because

307
00:59:43.140 --> 00:59:55.960
Igal Tabachnik: I think it's too big of a too big of a term for this. But I really don't remember anything kind of anything like this in in the recent years that that also explodes in

308
00:59:56.230 --> 01:00:08.099
Igal Tabachnik: in in the way that what's the word I'm looking for kind of improves every single time every week. There's some sort of marginal, some sort of major improvement that just

309
01:00:08.750 --> 01:00:14.759
Igal Tabachnik: I wouldn't. I wouldn't. I don't know if I would be able to go working without AI assisted tools anymore.

310
01:00:15.050 --> 01:00:20.020
Igal Tabachnik: because it's just when you figure out the flow. It's just a huge time saver.

311
01:00:20.140 --> 01:00:28.180
Igal Tabachnik: for example, I do. Instead of Googling. I don't remember the last time I googled anything. I use perplexity, which is another AI kind of

312
01:00:28.290 --> 01:00:56.580
Igal Tabachnik: kind of AI aggregator that does searches for you and scans many websites and gives you actionable to-do lists. So every time I need to search for something, I don't go Google it. I just go to perplexity or sometimes chat gpt when I need the dialogue, and I find myself using using it more and more. Now, of course, all of these tools cost money, and I'm sometimes happy for my personal use. I'm happy to pay for these things because I do get the value from them.

313
01:00:56.670 --> 01:01:16.760
Igal Tabachnik: It wasn't like that, with many kind of developer assisted tools. Funnily enough, Ellie and I used to work for a tooling company and selling tools to other developers is very hard. Tooling companies are notoriously difficult to sell like one. The biggest kind of success I can think of is probably Jetbrains.

314
01:01:16.890 --> 01:01:19.869
Igal Tabachnik: which which is able to sell ids.

315
01:01:20.000 --> 01:01:24.850
Igal Tabachnik: But there are now talks about ideas being a thing of the past.

316
01:01:25.460 --> 01:01:33.830
Igal Tabachnik: Now, whether or not it's a hyperbole I don't know, but I do find many, many compelling reasons to

317
01:01:34.300 --> 01:01:57.649
Igal Tabachnik: shift the loop to outside of the Id. The Id is there to just read through the code, maybe use some syntax highlighting and do maybe analyzing of the work that was done, but the main part of the work is being done by AI, which is exciting and scary at the same time. I'm sorry I kind of went into answering

318
01:01:57.860 --> 01:02:07.440
Igal Tabachnik: other imaginary set of questions, but but that's that's where I stand. I really like AI, and I do feel that it brings a lot of value to my day to day.

319
01:02:09.640 --> 01:02:11.689
Gabor Szabo: Andy, would you like to add add something to this.

320
01:02:12.000 --> 01:02:26.939
Eli Shalom: So one thing, which is, was one of the pains a year ago. So when we started working with rust and chatgpt, and we felt with both of us, my partner and I don't know rust. We coded a lot in rust, but we don't know rust.

321
01:02:27.060 --> 01:02:47.020
Eli Shalom: which was the 1st language I encountered this. It's also the 1st language I learned after the explosion of Llms. So this was like something which was very troubling, and less than a year after that. And so today was the 1st time I had a ticket which was done solely by an agent.

322
01:02:47.030 --> 01:02:57.879
Eli Shalom: So my whole thing, my whole work in the ticket was just to copy paste the the Jira ticket into Amp code, and that's it. And the implementation was the right. In the 1st place.

323
01:02:58.380 --> 01:03:00.220
Eli Shalom: it went online.

324
01:03:00.560 --> 01:03:05.040
Eli Shalom: Even, we have an AI in Github which also reviews the Pr.

325
01:03:05.160 --> 01:03:10.389
Eli Shalom: It gave a small comment, fixed it by itself. And that's it. I didn't do anything in the ticket

326
01:03:10.510 --> 01:03:34.190
Eli Shalom: other than copy pasting the the Jira ticket into the into the prompt. This was something which was exciting, so I still don't know. How do I feel about the worry from last year we but I want no rust, because I don't know rust. Well, probably there are many, many good developers who do it way better than me. But

327
01:03:34.390 --> 01:03:42.630
Eli Shalom: maybe I don't need to know rust that well in the future, because an agent will complement everything which I don't know how to do.

328
01:03:42.870 --> 01:03:43.840
Eli Shalom: So

329
01:03:44.240 --> 01:03:54.539
Eli Shalom: I wonder what will be the the conclusion in a year or 2 from now. But it's a very different experience than anything in the previous 2 decades. In coding.

330
01:03:57.220 --> 01:04:04.529
Gabor Szabo: Very interesting. So I'm not sure if the next question is even relevant anymore. I wanted to ask you.

331
01:04:04.650 --> 01:04:11.599
Gabor Szabo: I'm guessing, or I was guessing earlier that you're looking for more developers being a startup.

332
01:04:12.050 --> 01:04:15.900
Gabor Szabo: And then I wanted to ask you, okay, what would

333
01:04:16.030 --> 01:04:22.909
Gabor Szabo: a person who wants to apply to or or get picked up by you.

334
01:04:23.640 --> 01:04:38.950
Gabor Szabo: What could what should they invest in in if they had like one month now to prepare, or 1 2 months to to be a better candidate for you. But I am wondering now, do you even really need more people, or you're just

335
01:04:39.340 --> 01:04:43.590
Gabor Szabo: sit down, ride the tickets, and then you click 2 buttons, and it's done.

336
01:04:43.590 --> 01:04:46.150
Igal Tabachnik: The official term for this is vibe coding.

337
01:04:46.340 --> 01:04:47.040
Gabor Szabo: Yeah.

338
01:04:47.040 --> 01:04:49.540
Igal Tabachnik: Semi-official.

339
01:04:49.810 --> 01:04:50.570
Igal Tabachnik: Yep.

340
01:04:50.890 --> 01:05:15.139
Eli Shalom: So there is a very good question regarding this. So there's 1 thing, but many tickets are still very hard to develop. They're conceptually hard algorithm behind them to implement are not the trivial, and they're new by nature, and I believe it. An agent or any of them won't be able to implement them end to end at the beginning. So someone with deep understanding of

341
01:05:15.370 --> 01:05:29.939
Eli Shalom: coding engineering infrastructure will still be very relevant. And there's something which Eagle uses a lot when we're talking about rust developers, they're self selecting. So not many people go to learn and develop in rust by themselves.

342
01:05:30.090 --> 01:05:42.489
Eli Shalom: And like, probably I'm an exception, because I didn't have to choose it last. Chose me. But many people just go there and and and decide they want to be Russ. And usually these are people with a great

343
01:05:42.560 --> 01:05:52.600
Eli Shalom: passion for coding internals. Understanding. It's not a language we people come because it's very elegant, or the code looks very nice in the syntax.

344
01:05:52.660 --> 01:06:20.370
Eli Shalom: so I guess one of the things is that people who want to join the team are people who really really love rust and have passion to it, and want to learn more of it and do a lot. Another aspect which is probably unique to us is that the context of ethereum development is also quite challenging, because practically what we're doing is we're developing or working on something which executes a code of another, of

345
01:06:20.410 --> 01:06:35.029
Eli Shalom: of another virtual machine of another language. So one of the things which are very important are people who also like coding in general, and some experience in ethereum is very helpful like to understand the Protocols, how it works, how the virtual machine works.

346
01:06:35.080 --> 01:06:40.820
Eli Shalom: It gives a lot of advantage and reduces a lot of the pain in the work.

347
01:06:41.320 --> 01:06:56.140
Gabor Szabo: Okay? So I guess the answer would be, now that okay, so if there is a rust developer and she or he wants to join your team because they really like the idea. Then they should invest in learning at their own, and and try to

348
01:06:56.380 --> 01:06:58.719
Gabor Szabo: learn that, for example, right.

349
01:06:58.720 --> 01:07:01.000
Eli Shalom: Yeah, that's 1 of the things which helps most.

350
01:07:01.000 --> 01:07:06.730
Gabor Szabo: Okay, that's sort of like the internal things would, would

351
01:07:07.050 --> 01:07:25.100
Gabor Szabo: would like contributing to open source project. Be something that you look at the people. So if I go and contribute to open source projects, would that be something like a good point for me, or you don't care about that too much?

352
01:07:25.300 --> 01:07:38.319
Eli Shalom: So we give it a little bit of good points, especially to anyone who works in open source. It reflects something of the excitement and passion of the of the developer. But the main thing we saw that, like real experience, like

353
01:07:38.640 --> 01:07:53.309
Eli Shalom: having been post, the fight of the borrow checker is usually where people have more advantage with their experience. So some experience in real work with rust is is the thing we will mostly looking for.

354
01:07:55.720 --> 01:08:09.910
Gabor Szabo: Okay. And then, finally, I would like to change a little bit the direction of the audience. And, by the way, we didn't have any question from the audience. So the people who are present. So if you would like to ask anything, just please type in chat.

355
01:08:10.100 --> 01:08:11.210
Gabor Szabo: Awesome.

356
01:08:13.780 --> 01:08:31.690
Gabor Szabo: But I wanted to ask. That means, from my point of view, the last question we are still waiting for the for the guest is to the audience of other managers or entrepreneurs who try to set up their own startup company.

357
01:08:32.590 --> 01:08:33.690
Gabor Szabo: How would you?

358
01:08:34.189 --> 01:08:37.149
Gabor Szabo: What would you tell them when, when selecting

359
01:08:37.350 --> 01:08:40.720
Gabor Szabo: rust or not rust? I mean for you. It was a little bit

360
01:08:41.540 --> 01:08:52.639
Gabor Szabo: less of a choice, though you did try, python, but what what point should they consider? Better to use rust or not?

361
01:08:53.229 --> 01:09:05.659
Eli Shalom: So from my point of view, the main challenge is the lack of talent. So there are very few people, and locally, but also globally, and we've enough experience in in rust

362
01:09:05.759 --> 01:09:25.809
Eli Shalom: and men. And because a lot of those people joined many startups, especially in the in in our area, in blockchain companies. So the and the fight and competition over over talent is is very difficult. So it means that it's going to be a price that the startup is going to pay. So

363
01:09:25.859 --> 01:09:48.229
Eli Shalom: if a startup can choose to take a different language like a simpler one, even C-shop, then the chance to finding developers more easily is significantly higher, and going with rust means that at least for the next year or 2, and getting talent is going to be challenging.

364
01:09:48.369 --> 01:10:04.939
Eli Shalom: So that's the main thing. I'd say we like the language. We found that the ecosystem is rich enough to support almost everything we need. So it's like very like that. We it fills everything we need. Just we don't have enough people to

365
01:10:05.029 --> 01:10:25.899
Eli Shalom: we to be honest. We just managed to hire all the people we needed for the team, but it was we had to sweat a lot, for it was more challenging than we expected, and also I'm not sure that in regards to choosing it as a language to just a general back end and

366
01:10:26.139 --> 01:10:31.279
Eli Shalom: service system. I'm not sure that that's the right language to use like more

367
01:10:31.499 --> 01:10:40.379
Eli Shalom: infrastructure, like processes, things with which needs to work with high load probably justified a little bit better.

368
01:10:41.230 --> 01:11:03.209
Igal Tabachnik: Yeah, I actually do want to add, because I think we kind of discussed this question ahead of time. And also me personally, I was responsible. In the previous company I was responsible for training and mentoring Scala developers. Now Scala and Rust are pretty much in the same boat, with regards to scarcity of talent.

369
01:11:03.320 --> 01:11:22.239
Igal Tabachnik: It's a difficult language to pick up. And and and there are many, many way way. Kind of more forgiving languages out there. So why, why specifically rust, or why specifically, Scala. So I guess if you have needs now again rust was developed. So I'm going to say like

370
01:11:22.680 --> 01:11:46.709
Igal Tabachnik: 2 things. 1st of all, if you don't have ras talent and and but you really need to. You can hire, and you. You should hire quality engineers with a good resume and good kind of yes, open source. Open source contributions are always kind of welcomed. But we're generally looking for people who creative thinkers and problem solvers and and people who can, just

371
01:11:46.710 --> 01:11:55.199
Igal Tabachnik: they know how to learn things, and I am pretty sure they can pick up any language fast.

372
01:11:55.400 --> 01:12:10.420
Igal Tabachnik: So the goal is, make your own kind of invest. So this is what we're trying to do. We're trying to also invest in training resources in material for making our own rust developers eventually.

373
01:12:10.530 --> 01:12:17.490
Igal Tabachnik: whether or not we need to buy books or or courses, or do internal kind of sessions and and stuff like that. So

374
01:12:17.510 --> 01:12:46.230
Igal Tabachnik: this is all part of the growing of the company that all companies in early stage should invest in. But I will say, however, the negative, if there's 1 kind of why not pick rust is exactly like Eddie mentioned. I think that if your primary product is just a back end web server, you need to handle like web requests or serve websites. I might think that rust

375
01:12:46.450 --> 01:13:14.219
Igal Tabachnik: might be a slight overkill for that. Yes, you can. You can use an Http framework and serve it. But there's going to be way. Too much friction for getting things done quickly. I mean, you would much rather have a Jvm. Based language like Kotlin or Scala, obviously, or even typescript. There are many, many great big companies that do backends on typescript today, and it just works. And it's very, very fast, I would say, if you're a company that develops servers.

376
01:13:14.500 --> 01:13:43.670
Igal Tabachnik: products like that have to run and serve a lot of connections and a lot of be under high loads all the time. Things like Redis, for example, which is also written in rust, or being rewritten in rust. There are many other companies that I know that develop these tools, or if you primarily need to work on a commodity or a commodity hardware. For example, if you need to deploy products like browsers, for example, to end customers

377
01:13:43.730 --> 01:13:55.769
Igal Tabachnik: who might not have very fast machines. You might consider a language like rust or a native language that will compile to a small executable, and be very, very fast.

378
01:13:55.930 --> 01:14:07.660
Igal Tabachnik: But of course you could pick a language like C sharp for that. Again, there are. There are trade trade-offs in in many of them, from my perspective and kind of to. To conclude this.

379
01:14:07.940 --> 01:14:09.759
Igal Tabachnik: I would say, it's

380
01:14:10.040 --> 01:14:14.730
Igal Tabachnik: in the end you have to decide as a manager, as as an engineering manager. What's kind of

381
01:14:14.890 --> 01:14:21.590
Igal Tabachnik: you have to pick one out of the 2 options, whether you prefer correctness

382
01:14:21.770 --> 01:14:25.259
Igal Tabachnik: or do you prefer well anything else

383
01:14:25.330 --> 01:14:31.819
Igal Tabachnik: like how important it is to to have a bug-free experience for you like. If you place it higher up.

384
01:14:31.880 --> 01:14:51.289
Igal Tabachnik: then you should probably pick a language like rust that will prevent you from having runtime surprises if I say so, if you use it correctly. Of course there's many gotchas in rust like, if you use it incorrectly like, if you unwrap everywhere, which is the wrong way of using rust. But many people do, because it

385
01:14:51.290 --> 01:15:10.150
Igal Tabachnik: it lets them compile the code. So no unwrapping is the exact wrong thing to do. And until people realize that they will be using rust wrong and they won't get any benefit from it. They will see that it's actually it's not protecting them from anything. They will have runtime panics.

386
01:15:10.280 --> 01:15:29.199
Igal Tabachnik: They need to learn to understand that unwrapping is not what you do, for instance, and it's a learning curve, like, like with many languages. I think rust has an additional learning curve, because it introduces a couple of novel ideas that do not exist in many languages, like people coming from Java, or even C, plus plus.

387
01:15:29.320 --> 01:15:40.979
Igal Tabachnik: They don't necessarily know of all of these mechanisms, for, you know, separating data with behavior, for example, derivation or or things like

388
01:15:42.030 --> 01:15:48.610
Igal Tabachnik: even the macro system works would work differently, I suppose, like, I said, there's learning curve you have to invest in it, and

389
01:15:48.810 --> 01:15:52.030
Igal Tabachnik: it's it's a it's a matter of of a trade-off.

390
01:15:52.300 --> 01:16:08.530
Igal Tabachnik: But but the bottom line is that if you make servers, you probably want to look into rust. If you make general backend servers like serving websites, you would likely do better with using a less difficult language to to master.

391
01:16:10.990 --> 01:16:16.950
Gabor Szabo: Okay, thank you very much. I ran out of questions. And

392
01:16:17.050 --> 01:16:30.999
Gabor Szabo: and our audience is rather silent today. And do you have anything else that you wanted to talk about, that you think that I should have asked earlier, and I didn't.

393
01:16:33.340 --> 01:17:00.219
Igal Tabachnik: Well, 1st of all, I just wanted to thank you again for inviting us. This is really fun, and I like to geek out about this stuff. Sometimes it's very difficult for me to stop talking, and so hopefully. The reason there are no questions is that I inadvertently answered them all hopefully or not. We'll find out, maybe from feedback on this episodes when it comes out.

394
01:17:00.350 --> 01:17:01.250
Igal Tabachnik: And

395
01:17:02.060 --> 01:17:13.169
Igal Tabachnik: but but I think we covered a lot. I think the questions were great. And I'm looking forward for this podcast to run for many, many years and have many, many great guests. And

396
01:17:13.210 --> 01:17:33.689
Igal Tabachnik: maybe before we leave, I will provide our socials, and how to reach us. If there are any, follow up questions I would be more than happy to answer even mail or or Twitter X, or even even just a private private zoom. That's also always welcomed. I'm happy to talk about these things.

397
01:17:34.230 --> 01:17:39.800
Gabor Szabo: Okay, sir, you want to say them, or we'll just add them to.

398
01:17:39.800 --> 01:17:45.270
Igal Tabachnik: Maybe I will. She should probably add them, because my my nicknames are weird. I have to spell them out.

399
01:17:45.530 --> 01:17:48.909
Gabor Szabo: Okay, Ellie, you wanted to add something.

400
01:17:49.340 --> 01:18:05.709
Eli Shalom: And so I don't have anything to add, and just have to say that I'm really excited, that a new community of rust is being built and people are attracted to it. It's a very exciting language, very exciting technology being built with it in many projects. And we're very happy that this forum exists.

401
01:18:07.240 --> 01:18:16.319
Gabor Szabo: Well, I I really thank you very much for for being here, and and all kind of really fun, information, and and insight you we got

402
01:18:16.440 --> 01:18:26.519
Gabor Szabo: and thank you for the audience, who was listening silently here and thank you for everyone who is listening, who is listening to the podcast

403
01:18:26.690 --> 01:18:29.589
Gabor Szabo: there are going to be

404
01:18:29.980 --> 01:18:36.770
Gabor Szabo: show notes is all kind of links, so please follow them, and thank you very much. Goodbye. Bye-bye.

405
01:18:36.770 --> 01:18:37.739
Eli Shalom: Thank you. Bye.

