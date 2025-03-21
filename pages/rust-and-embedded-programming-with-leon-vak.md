---
title: Rust and embedded programming with Leon Vak
timestamp: 2025-03-21T14:30:01
author: szabgab
published: true
show_related: true
description:
#tags:
#    - embedded
---

{% youtube id="kh24SgUIid4" file="2025-03-20-rust-and-embedded-programming-with-leon-vak.mp4" %}

In this online session [Leon Vak](https://www.linkedin.com/in/leon-vak-24445922/), who gave 3 excellent presentations about Embedded programming and Linux kernel programming with Rust at Abra, is going to show us how he uses Rust to write programs for an embedded device.

![Leon Vak](images/leon-vak.jpeg)


## Transcript

1
00:00:01.900 --> 00:00:21.539
Gabor Szabo: So Hello, and and welcome to the Code Maven and Meetup Group. And if you're watching the video, then the Code Maven Youtube, Channel and my name is Gabor Sobo. I usually teach rust and python, and how companies introduce these languages or introduce testing for these languages, and I also organize the

2
00:00:21.950 --> 00:00:50.339
Gabor Szabo: Both the rust and the python meetup groups here in Israel. And I also have this Code Mavens Group, where I try to organize presentations both in rust and in python, in English, in zoom. So we can reach all the international community. And we can also invite people. If you look at the history. You can find all kind of other videos from other people about all kind of interesting subjects in these 2 languages.

3
00:00:50.812 --> 00:00:59.000
Gabor Szabo: Is that said, I am really happy, Leon, that you agreed to give this presentation. I think this is the 3rd time you're already giving this or so so

4
00:00:59.000 --> 00:00:59.780
Leon: Yeah.

5
00:01:00.216 --> 00:01:26.380
Gabor Szabo: And but this time it's in English. So we'll have it also for the international community and recorded. And thank you very much everyone who joined us. As I said earlier, you can feel free to ask questions in the chat, and if you don't want to speak up, and that's it, I think. Now the stage is yours.

6
00:01:26.980 --> 00:01:27.570
Leon: Alright!

7
00:01:27.670 --> 00:01:31.940
Gabor Szabo: Share your screen. You can talk without sharing your screen whatever you like.

8
00:01:32.420 --> 00:01:46.970
Leon: All right, all right, all right, we're there the right. So let me just share the screen again.

9
00:01:49.430 --> 00:01:52.930
Leon: Alright. So you guys will be

10
00:01:53.900 --> 00:01:56.700
Leon: able to see my screen very soon

11
00:01:56.700 --> 00:01:58.690
Gabor Szabo: Yeah, we see the embedded summer camp

12
00:01:59.020 --> 00:02:06.749
Leon: Okay, okay, okay, okay, very good. So right? So this this meetup was a

13
00:02:07.140 --> 00:02:10.999
Leon: was created as a part of 3.

14
00:02:11.860 --> 00:02:18.329
Leon: 3 separate meetups we have arranged in our company called Abra.

15
00:02:18.470 --> 00:02:28.430
Leon: and this was the the specific one we're talking about right now is the rust in embedded systems, and

16
00:02:29.150 --> 00:02:31.490
Leon: and it was a

17
00:02:31.970 --> 00:02:47.240
Leon: it was the second one. The 1st one was Linux Kernel Development, which was not related to rust at all. The second one was the rust in embedded systems, and the last one was rust in Linux Kernel.

18
00:02:49.070 --> 00:02:59.470
Leon: This series was quite interesting for me to prepare and to present, and I hope you'll find it.

19
00:03:00.025 --> 00:03:02.050
Leon: at least a bit amusing.

20
00:03:02.460 --> 00:03:03.780
Leon: M.

21
00:03:04.210 --> 00:03:07.209
Leon: So, right, we?

22
00:03:09.926 --> 00:03:13.219
Leon: yeah. 1st of all, I my name is Leon.

23
00:03:13.410 --> 00:03:14.240
Leon: Right?

24
00:03:14.350 --> 00:03:19.780
Leon: That's enough. And you know, we, the rest people.

25
00:03:20.060 --> 00:03:24.990
Leon: We we have. We clearly have some issues right?

26
00:03:25.662 --> 00:03:32.320
Leon: And you will. I guess by now you already are aware about

27
00:03:32.580 --> 00:03:41.690
Leon: our little problems. But you know that's why we're here today. So I hope you'll and Joyce

28
00:03:42.611 --> 00:04:03.329
Leon: we will be talking about rust for embedded and we will mention how rust for embedded systems for bare metal, embedded systems is not the same as rust for you know, when you develop in user space like Linux windows, whatever

29
00:04:04.181 --> 00:04:10.899
Leon: we will mention the fact that rust provides your

30
00:04:11.640 --> 00:04:21.590
Leon: gives you or lets you develop in a full. It's very supporting ecosystem for embedded and specifically for bare metal.

31
00:04:22.340 --> 00:04:31.600
Leon: And we will be questioning just a little bit rust versus C and C plus plus in the embedded. Just to touch on that, because

32
00:04:32.060 --> 00:04:33.629
Leon: it's hard not to.

33
00:04:33.940 --> 00:04:41.539
Leon: And what we will not be doing today, we will not be learning what embedded development is. I

34
00:04:41.750 --> 00:04:45.869
Leon: hope that you guys have any well, just

35
00:04:46.020 --> 00:05:03.269
Leon: any sort of idea about it up to now. And we will not be mentioning concepts like serial port and the leads, the leds switching on and off, and so on. Right? I assume that you

36
00:05:03.410 --> 00:05:09.889
Leon: are aware about these things, and we are not going to be learning the rust language.

37
00:05:10.300 --> 00:05:14.369
Leon: It's not about learning rust language. I hope you will

38
00:05:14.620 --> 00:05:18.459
Leon: see what the presentation is about.

39
00:05:18.977 --> 00:05:31.299
Leon: There are not many slides here. We'll just speak very quickly about a few slides here like 3, 4 slides, and then we will switch to a hands on a demo

40
00:05:32.256 --> 00:05:45.963
Leon: which is which has a little issue today. Because, there is one thing that's not working for some reason, but we will get over it. And right. So let's

41
00:05:46.770 --> 00:05:47.819
Leon: go ahead.

42
00:05:49.640 --> 00:05:52.310
Leon: What are we?

43
00:05:52.600 --> 00:05:56.299
Leon: Well, I I cannot say we're all familiar with that, but

44
00:05:57.121 --> 00:06:23.460
Leon: many of us are well familiar with it, with the good old dead old days when you were quite locked, with a development under a specific platform and a specific ide, and the specific debugger. And whatever zylogue is a good example, I used to work with it once. And yeah, so we had an ide just for it.

45
00:06:23.800 --> 00:06:30.909
Leon: Special debugger probe, and everything is quite unique for this platform.

46
00:06:31.950 --> 00:06:48.280
Leon: And so yeah, so it's all about the Ics, the chips eventually. So the chip was from Zylogue, and the Id was developed by them, and the compiler and the debugger. Everything is just.

47
00:06:48.690 --> 00:06:50.650
Leon: you know, very custom.

48
00:06:52.730 --> 00:07:02.630
Leon: And now we have rust, and this is very, very different from C, because

49
00:07:03.420 --> 00:07:18.349
Leon: well, we have the next slide. So why rush 1st of all, we will be moving from vendor login to freedom. We are really capable of developing whatever we want without any.

50
00:07:18.650 --> 00:07:26.759
Leon: I am any code supplied by the vendor entirely.

51
00:07:27.510 --> 00:07:34.549
Leon: and we will use any id that we like I used. I'm used to work with

52
00:07:35.420 --> 00:07:40.020
Leon: this code, but you can use any id

53
00:07:40.290 --> 00:07:50.280
Leon: of choice. You don't need an idea. You can use just the code editor. And for the hardcore guys you can do everything in them.

54
00:07:50.980 --> 00:07:52.250
Leon: M.

55
00:07:52.770 --> 00:07:58.590
Leon: And what's good about rust compared to the bare metal C programming.

56
00:07:58.840 --> 00:08:09.209
Leon: It's the power of modern language in embedded programming. It's just incredible what stuff you can do in bare metal. If you

57
00:08:09.820 --> 00:08:18.950
Leon: you know, if you if you're used to bare metal programming in C, then the switch to rust would just knock your mind off.

58
00:08:19.580 --> 00:08:27.890
Leon: M, yeah. And why? Rust, of course, unmatched safety concurrency. And what about the performance?

59
00:08:29.070 --> 00:08:36.760
Leon: Well, we really need to talk about it. Because that's 1 of the main questions people always ask about.

60
00:08:36.860 --> 00:08:43.650
Leon: You know why use rust and not c not c plus plus in embedded development.

61
00:08:43.990 --> 00:08:51.310
Leon: So and many times everything boils down to performance.

62
00:08:52.700 --> 00:09:00.640
Leon: I will give you the tools to measure your own and performance requirements.

63
00:09:00.810 --> 00:09:04.850
Leon: So let's Luke over here.

64
00:09:05.620 --> 00:09:10.329
Leon: You have the I'll try to make it slightly bigger

65
00:09:10.985 --> 00:09:16.219
Leon: you have the benchmark game. Right? This is under Debian.

66
00:09:16.990 --> 00:09:22.079
Leon: Okay, debian.net benchmark game. You can find it easily on the net.

67
00:09:22.440 --> 00:09:23.900
Leon: Okay, no worries.

68
00:09:24.050 --> 00:09:31.630
Leon: And and you have certain comparisons of rust versus

69
00:09:32.470 --> 00:09:36.419
Leon: other languages. So let's take C, for example.

70
00:09:37.040 --> 00:09:41.829
Leon: right? And you have the comparison to Gcc.

71
00:09:42.210 --> 00:09:47.400
Leon: and you have different algorithms here. Everything is very, very, you know.

72
00:09:47.620 --> 00:09:57.069
Leon: Open. So if you click on the specific algorithm, you get the all kind of measurements.

73
00:09:57.690 --> 00:10:03.419
Leon: And you can get to the algorithm itself, the implementations, everything is here. It's all.

74
00:10:03.790 --> 00:10:06.780
Leon: It's all open source and available.

75
00:10:08.360 --> 00:10:16.690
Leon: So when you compare rust to C with Gcc. For example, you can see the you know

76
00:10:17.360 --> 00:10:19.880
Leon: the different runs here.

77
00:10:20.400 --> 00:10:48.149
Leon: and you can sort it. You can do a lot of things here, but eventually you will see that rust is not better than C. And C is not better than rust depending on an algorithm and on the tests performed in some, in some C would be quicker in some rust will take the 1st places.

78
00:10:48.350 --> 00:10:49.969
Leon: It's just, you know.

79
00:10:50.210 --> 00:10:53.650
Leon: It's all right. They are quite the same eventually.

80
00:10:53.880 --> 00:11:00.679
Leon: so no need to worry about performance. The performance is, how do you define it?

81
00:11:00.870 --> 00:11:04.079
Leon: I suppose it's as good as it gets.

82
00:11:04.890 --> 00:11:13.699
Leon: at least, you know, using modern compilers and systems, you can instead compare it to the C. Lang.

83
00:11:13.840 --> 00:11:18.220
Leon: And once again, the picture doesn't change much.

84
00:11:19.720 --> 00:11:28.969
Leon: so feel free to go to this site and and check it for yourself. I will not be teaching you what's

85
00:11:29.140 --> 00:11:30.960
Leon: the best for you.

86
00:11:31.390 --> 00:11:36.540
Leon: You can compare it to any language, and with different algos and whatever.

87
00:11:37.260 --> 00:11:48.149
Leon: So enough with that, I suppose. It's quite clear right now, why does it start again?

88
00:11:48.560 --> 00:11:54.029
Leon: Okay, sorry. Okay, so this is about the performance.

89
00:11:54.780 --> 00:11:59.740
Leon: you know, talking about safety, concurrency, and whatever you.

90
00:12:00.560 --> 00:12:02.379
Leon: I guess that by now you're

91
00:12:02.730 --> 00:12:12.649
Leon: trust shines on these sides and way way better than C. So if the performance is the same, so why choose any other language?

92
00:12:14.003 --> 00:12:17.730
Leon: Oh, yeah. By the way, this is Ferris.

93
00:12:19.090 --> 00:12:23.570
Leon: Just so you know this is the mascot. His name is Ferris.

94
00:12:24.299 --> 00:12:37.200
Leon: Any idea why, he's called Ferris? It's very simple, Ferro, like rust, like whatever programmers jokes.

95
00:12:38.160 --> 00:12:47.950
Leon: M, all right. So what and how the bare metal programming is different.

96
00:12:50.400 --> 00:12:55.889
Leon: Well, eventually, when you write any program for

97
00:12:56.540 --> 00:13:03.409
Leon: see for for for any OS like Windows or Linux, or whatever.

98
00:13:03.860 --> 00:13:14.139
Leon: Eventually your code will swap to using C interfaces because the operating systems.

99
00:13:14.320 --> 00:13:23.300
Leon: not all of them, but most of them are written with C, and your code will just

100
00:13:25.230 --> 00:13:29.020
Leon: run C code eventually under the hood.

101
00:13:29.890 --> 00:13:33.620
Leon: Here is an example using the foreign function interface.

102
00:13:34.460 --> 00:13:40.610
Leon: This is this comes from the rust. It's just an random example

103
00:13:40.730 --> 00:13:51.059
Leon: and rust lang rust. 3 master library, Std. Src. OS unix Fs. So this is a file system library

104
00:13:51.360 --> 00:13:52.810
Leon: for unix.

105
00:13:53.030 --> 00:13:57.880
Leon: And this is just one function from it. Make dear function.

106
00:13:58.300 --> 00:13:59.600
Leon: What does it do?

107
00:14:01.040 --> 00:14:06.579
Leon: It simply calls the lip. See? Make deer on this machine.

108
00:14:07.380 --> 00:14:18.040
Leon: This is quite clear. And because these things, these things are implemented on the Lipsy level, and

109
00:14:18.400 --> 00:14:20.120
Leon: everything boils down to that.

110
00:14:21.810 --> 00:14:25.730
Leon: There is certain questions that one may ask.

111
00:14:25.900 --> 00:14:29.319
Leon: If it all boils, boils down to C, then why use rust?

112
00:14:29.910 --> 00:14:31.759
Leon: Well if you can.

113
00:14:32.030 --> 00:14:34.520
Leon: Right? If you can write

114
00:14:34.620 --> 00:14:44.480
Leon: big apps with lots of business logic and feel safe on memory issues and everything else rust helps you with.

115
00:14:44.930 --> 00:14:49.319
Leon: Then I think you've done a good job here.

116
00:14:50.070 --> 00:14:58.979
Leon: anyway. A foreign function. Interface is an interesting topic to read about Ffi. You can find it easily on the network.

117
00:14:59.450 --> 00:15:03.720
Leon: And, by the way, rust is an open source language.

118
00:15:04.651 --> 00:15:07.539
Leon: Just try to find

119
00:15:07.720 --> 00:15:14.089
Leon: the code for make deer in lip C. It will take you some time, I promise you that

120
00:15:14.190 --> 00:15:20.950
Leon: it's not that easy with rust. You just get there almost immediately with any search on the network.

121
00:15:23.330 --> 00:15:28.359
Leon: So yeah, so you are programming in rust. However.

122
00:15:28.710 --> 00:15:34.490
Leon: you're getting to see interfaces on the OS abis.

123
00:15:36.270 --> 00:15:41.220
Leon: The rest for embedded is very different on these regards. Well.

124
00:15:41.350 --> 00:15:47.960
Leon: once again, when I say rust for embedded here, I mean rust for bare metal embedded systems. All right.

125
00:15:48.310 --> 00:15:54.669
Leon: So as you can see at first, st you see. Well, you guys, I don't know. Some of you may

126
00:15:54.830 --> 00:16:01.030
Leon: have experience with Russ. Some not. I will explain. Some basics. Don't worry about it. So

127
00:16:02.130 --> 00:16:05.690
Leon: you have the no Std. And no main.

128
00:16:06.030 --> 00:16:07.530
Leon: Why is that?

129
00:16:07.870 --> 00:16:10.739
Leon: Well, 1st of all, there is no standard library.

130
00:16:11.010 --> 00:16:13.529
Leon: There is no standard library under

131
00:16:13.900 --> 00:16:17.330
Leon: bare metal programming. You don't have any

132
00:16:18.100 --> 00:16:24.949
Leon: special library behind everything. You don't have the Lib. C. Or stuff to help you with.

133
00:16:25.070 --> 00:16:38.669
Leon: There are certain standard stuff there. There is a small standard library built into your code now, when you compile with no Std. But these are just basics

134
00:16:39.190 --> 00:16:48.579
Leon: and and no main, because, you know you're you feel free to select your entry point. For example, here. It's called my main.

135
00:16:48.760 --> 00:17:01.990
Leon: and it's it has the entry syntax here that marks it as entry point for this function. So when it builds, the Linker will arrange

136
00:17:02.130 --> 00:17:05.830
Leon: and the start of our code over here.

137
00:17:06.000 --> 00:17:10.809
Leon: So what you see here is some basic code for

138
00:17:12.598 --> 00:17:16.859
Leon: for a bare metal program in rust.

139
00:17:17.300 --> 00:17:20.852
Leon: Apart from the main, you have the

140
00:17:21.579 --> 00:17:28.689
Leon: the core panic panic info stuff which will be used for this function

141
00:17:28.800 --> 00:17:38.690
Leon: because we need to provide the panic function. This is the only function that we really have to provide apart from the entry function.

142
00:17:38.990 --> 00:17:41.750
Leon: And why do we need to provide panic.

143
00:17:42.000 --> 00:17:44.152
Leon: People familiar with

144
00:17:45.350 --> 00:17:54.397
Leon: with programming under an operating system are used to error handling to all kind of

145
00:17:55.560 --> 00:18:08.500
Leon: all kind of failures that are treated, whether in well, we, we are talking about runtime stuff right now. So at Runtime your program can fail in different ways.

146
00:18:08.990 --> 00:18:13.149
Leon: But in a bare metal embedded system.

147
00:18:13.650 --> 00:18:19.890
Leon: You don't have this luxury. You need to handle your panics by yourself.

148
00:18:20.370 --> 00:18:26.830
Leon: and when your code is about to panic you need to provide a handler to go into.

149
00:18:27.700 --> 00:18:32.799
Leon: and I'll just give you just just a thought. You know. Why.

150
00:18:33.690 --> 00:18:39.370
Leon: Why is this here? Why, the inline? Never for the panic handler

151
00:18:42.840 --> 00:18:48.959
Leon: any ideas, why cannot this function be inlined?

152
00:18:51.310 --> 00:18:58.189
Leon: The the reason for that is that this function will be called by a pointer.

153
00:18:58.650 --> 00:19:05.659
Leon: This function sits somewhere in the memory, and in case the CPU performs some illegal function

154
00:19:06.430 --> 00:19:19.190
Leon: like dividing by 0 or any other failure, then it will just jump over to the panic function. Now, inside panic function, it's up to you how to handle it.

155
00:19:19.590 --> 00:19:30.769
Leon: It's bare metal. You have no OS to look after. So you it's up to you. You can reset the platform, for example, or you can just do nothing and get stuck in the loop

156
00:19:30.950 --> 00:19:32.130
Leon: whatever you want.

157
00:19:32.590 --> 00:19:38.470
Leon: So this here is a very basic program for the cortex, M.

158
00:19:38.810 --> 00:19:39.839
Leon: And stuff.

159
00:19:40.280 --> 00:19:48.750
Leon: Now, how the things work with rust for bare metal programming with embedded rust for bare metal.

160
00:19:48.910 --> 00:19:54.790
Leon: 1st of all, you have the hardware, right? We all know Leds.

161
00:19:55.210 --> 00:20:01.379
Leon: And how do you control this hardware? Well, you can use direct register access.

162
00:20:02.410 --> 00:20:05.150
Leon: This is an example you use unsafe.

163
00:20:05.650 --> 00:20:12.039
Leon: and you write volatile, and this is the address

164
00:20:12.260 --> 00:20:18.120
Leon: right? And then you read volatile. Ptr. Read volatile from the same address.

165
00:20:18.250 --> 00:20:24.930
Leon: and then you perform an operation on it just to light up the relevant led.

166
00:20:25.060 --> 00:20:32.269
Leon: and then you write it back. So you read the value you modify it, and then you write it back to the memory.

167
00:20:32.650 --> 00:20:41.775
Leon: To do that you need to understand very well what you're doing, because this is just an address. It's just a re register in the

168
00:20:42.370 --> 00:20:52.548
Leon: in the in your mcu. So you you need to understand very well what you're doing here you need to read the the data sheets and

169
00:20:53.350 --> 00:20:55.329
Leon: figure out everything for yourself.

170
00:20:56.400 --> 00:21:07.640
Leon: The next level just above it is the pack, the peripheral access rate, this looks very different.

171
00:21:08.400 --> 00:21:13.090
Leon: 1st of all, you use the relevant pack.

172
00:21:13.260 --> 00:21:18.689
Leon: In this case I gave an example for an Stm. 32 F. 4.

173
00:21:19.070 --> 00:21:26.679
Leon: This is all open source and everything very accessible. You can check out the code inside whatever you want.

174
00:21:26.960 --> 00:21:31.870
Leon: So there you apply to Gpio

175
00:21:32.200 --> 00:21:36.800
Leon: a number A, which is the one of the Gpio sets.

176
00:21:36.980 --> 00:21:41.510
Leon: and then you have the output data register, and then you modify it

177
00:21:41.960 --> 00:21:46.250
Leon: alright, which is a method in Svd. To rust, which will

178
00:21:46.390 --> 00:21:48.939
Leon: we'll just mention in a second

179
00:21:49.844 --> 00:21:58.560
Leon: and then you just use the Odr 5 in this case, and you clear a bit.

180
00:22:00.030 --> 00:22:03.510
Leon: This makes more sense than what we've seen below.

181
00:22:04.100 --> 00:22:06.560
Leon: Now, what's the Svd to rust?

182
00:22:08.230 --> 00:22:23.299
Leon: well, most Mcu manufacturers provide Svd files that define the whole usage of their Gpios or the pins of the of the Mcu.

183
00:22:23.420 --> 00:22:33.619
Leon: And you can use the Svd to rust a tool to create the the packs.

184
00:22:33.720 --> 00:22:45.400
Leon: Okay, so packs can be created automatically, mostly. By just receiving the proper Svd file for the platform. So if you have your own

185
00:22:45.590 --> 00:22:56.750
Leon: like, you developed your own silicon, or a just have something that is not currently supported. As long as you have the Svd file, you're okay.

186
00:22:57.210 --> 00:23:03.900
Leon: You can get more information about it in Svd. To rust. Just type it on the net, and you'll find out.

187
00:23:05.430 --> 00:23:08.480
Leon: So this is nicer, but we can do better.

188
00:23:09.970 --> 00:23:19.200
Leon: One level above it is the hardware obstruction layer. How we'll know this this name

189
00:23:19.700 --> 00:23:25.870
Leon: and the how is way more convenient for us, as you can see here, we use the how.

190
00:23:25.970 --> 00:23:32.570
Leon: and we have the sample usage like Gpioa PA. 5,

191
00:23:32.800 --> 00:23:36.689
Leon: which was the Odr 5 over here.

192
00:23:36.970 --> 00:23:42.570
Leon: and maybe probably number Gpi. Number 5 over here

193
00:23:42.830 --> 00:23:51.129
Leon: into push, pull output. Okay? So we we 1st configure it to push, pull output. Then we set it to high.

194
00:23:51.480 --> 00:23:53.120
Leon: and that's it.

195
00:23:54.380 --> 00:24:01.140
Leon: Alright. So here, this is very, very straightforward, very convenient to use, and you have it all

196
00:24:01.530 --> 00:24:09.579
Leon: in the relevant hull crate like Stm. 32 f. Or xxl.

197
00:24:11.133 --> 00:24:31.049
Leon: But then you have the board support packages which are fantastic. The difference between how and the board support package is that the how allows you to work with the Mcu. So you select the Mcu and then you use its how.

198
00:24:31.200 --> 00:24:35.940
Leon: But board support packages is for is clearly for a board

199
00:24:36.597 --> 00:24:42.970
Gabor Szabo: Sorry for to interrupt you. I have a question. If it's okay, business layers.

200
00:24:44.680 --> 00:24:45.680
Gabor Szabo: Can you hear me?

201
00:24:45.840 --> 00:24:46.780
Leon: Yeah, yeah, I'm with you.

202
00:24:47.020 --> 00:24:48.340
Gabor Szabo: So so

203
00:24:49.230 --> 00:25:01.900
Gabor Szabo: are these examples really related, so meaning that there, there is the the lowest level the direct register access. You have this hexadecimal number?

204
00:25:02.030 --> 00:25:08.819
Gabor Szabo: Is the Odr actually correct? The Odr corresponds to that or so is there a name for basically

205
00:25:08.820 --> 00:25:09.600
Leon: Yes.

206
00:25:09.600 --> 00:25:10.090
Gabor Szabo: We'll go ahead

207
00:25:10.090 --> 00:25:15.680
Leon: Yes, yes, yes, under the hood. It works exactly like it's shown here.

208
00:25:15.970 --> 00:25:30.180
Leon: all right. Eventually everything boils down to these these commands, but they are just wrapped in layers of wrappers that allow you to use them in the easiest way possible.

209
00:25:31.220 --> 00:25:41.329
Gabor Szabo: Okay. So basically, the the what you can see then the on the pack level operation is basically doing the same reading and shifting and writing back

210
00:25:41.580 --> 00:25:42.779
Leon: Yes, exactly.

211
00:25:43.630 --> 00:25:45.360
Gabor Szabo: Is the clear bit? Okay.

212
00:25:45.510 --> 00:25:45.840
Leon: Yep.

213
00:25:45.840 --> 00:25:46.859
Gabor Szabo: Okay. Thank you.

214
00:25:47.660 --> 00:26:05.250
Leon: So the difference between the hull and the Bsp. Is because Bsp is about the board. So the actual board, like like this one over here. This is a board. It's a specific board that you can get and use.

215
00:26:05.370 --> 00:26:08.629
Leon: So if you have evaluation boards.

216
00:26:08.860 --> 00:26:13.760
Leon: many of these evaluation boards like the nucleo over here.

217
00:26:14.840 --> 00:26:25.820
Leon: You have the Bsp available for them. Now, why is it better to use the Bsp eventually, if you're using a

218
00:26:27.230 --> 00:26:33.810
Leon: an evaluation board. It's because here, for example, we can define an led.

219
00:26:34.920 --> 00:26:37.339
Leon: So when we define an led.

220
00:26:38.380 --> 00:26:52.280
Leon: and of course we configure it as appropriate, then you. You just have the on method for the led, because we know that this is an led. It's not just a Gpio.

221
00:26:52.630 --> 00:26:59.930
Leon: because we know that behind this specific Gpio on this specific board, we absolutely have an led.

222
00:27:00.220 --> 00:27:05.510
Leon: so we can configure it and just use the specific led with just an own command.

223
00:27:05.840 --> 00:27:08.540
Leon: and it makes the life very easy.

224
00:27:09.520 --> 00:27:10.465
Leon: And

225
00:27:11.550 --> 00:27:29.480
Leon: also, if you have your own board, if you develop your own board, then you can use some basic or some similar Bsps to define your own Bsp, and work, you know, like a king with this board very easy interface.

226
00:27:29.950 --> 00:27:37.040
Leon: So these are the layers of working with and

227
00:27:37.580 --> 00:27:43.445
Leon: and these are the layers of working with rust,

228
00:27:44.820 --> 00:27:47.700
Leon: With mcus in bare metal.

229
00:27:48.920 --> 00:27:51.620
Leon: Now, you know.

230
00:27:53.400 --> 00:28:02.520
Leon: And I use C. The C language in embedded development for for too long.

231
00:28:03.760 --> 00:28:12.279
Leon: And then, when I started, you know, considering this meetup, I just

232
00:28:13.250 --> 00:28:21.029
Leon: did a test. I just went to the Internet, you know, on Google and wrote cross compilation setup in C,

233
00:28:22.340 --> 00:28:26.969
Leon: and the 1st thing it brought me to was

234
00:28:27.290 --> 00:28:45.490
Leon: stack overflow cross compilation requirements. I have some basic knowledge about compiling with C need to get a couple of generic cross compilation questions answered. In my case, I'm trying to whatever and this and that, and there were lots of sections like that, and no real good explanation.

235
00:28:45.900 --> 00:28:53.320
Leon: If you do the same in rust, you immediately arrive to the right place, to the right

236
00:28:53.540 --> 00:29:00.600
Leon: website and to the right page. That explains you exactly how to build everything you need.

237
00:29:01.230 --> 00:29:09.680
Leon: And this is just fantastic. This is why rust is a modern programming language. Unlike C,

238
00:29:12.120 --> 00:29:16.249
Leon: we can talk a lot about that, but we will, we will not right now.

239
00:29:16.510 --> 00:29:22.510
Leon: Still, the picture is clear. I clearly feel the difference here.

240
00:29:24.060 --> 00:29:33.189
Leon: Yeah, this is just some fun poster I made for explaining my feelings.

241
00:29:33.420 --> 00:29:37.449
Leon: Yes, C. Is quite confusing at times.

242
00:29:37.590 --> 00:29:45.570
Leon: Rust is way more straightforward, so you can be ready to go. Oh, sorry to rust in 5 min.

243
00:29:45.800 --> 00:29:47.190
Leon: maybe less.

244
00:29:47.400 --> 00:29:53.539
Leon: It's all up to you. You install the cross compiler and set up a project that's all.

245
00:29:53.920 --> 00:30:00.120
Leon: So rust up target, add you add your own target depends on what you work with.

246
00:30:00.400 --> 00:30:03.190
Leon: Then you create the project.

247
00:30:04.090 --> 00:30:15.740
Leon: Then you go into your project. Then you cargo. Ed, for example, in this case it's cortex, and there are different targets. We will mention Rtt. Very soon.

248
00:30:16.700 --> 00:30:21.559
Leon: and that's about it. You're ready to to work

249
00:30:22.310 --> 00:30:27.499
Leon: now, you code it. You set it up. You set it up for the target, and then

250
00:30:28.450 --> 00:30:30.860
Leon: then the fun part begins, because.

251
00:30:31.310 --> 00:30:38.230
Leon: unlike with C or any other language, well, most of other languages I'm familiar with.

252
00:30:39.939 --> 00:30:44.520
Leon: You check it, build it, flash it, debug it.

253
00:30:44.730 --> 00:30:49.999
Leon: and even more stuff you can do with it like Rtt. Eat

254
00:30:50.160 --> 00:30:54.119
Leon: and so on, just with one single command.

255
00:30:54.510 --> 00:30:56.409
Leon: It's cargo and bed.

256
00:30:57.010 --> 00:31:19.039
Leon: You don't have to use it. But this is a wonderful command. And once again I'm mentioning this is a great ecosystem to start working with embedded development with bare metal. It's cargo embed. You specify the target or you can specify it in the configuration file. And that's it. You just type cargo embed, and it's already there.

257
00:31:19.250 --> 00:31:20.770
Leon: We'll see it in a second.

258
00:31:21.470 --> 00:31:30.469
Leon: So cargo is really, I really believe that it's the marvel of modern development. It's incredible.

259
00:31:30.650 --> 00:31:36.530
Leon: You, it allows you to create new projects very quickly with git built in.

260
00:31:36.680 --> 00:31:44.309
Leon: you add and manage crates and utilities. You have a simplified, efficient builds because well.

261
00:31:44.540 --> 00:31:55.150
Leon: when you use it, you know the integrated testing framework inside automatically generate the documentation. You want seamless, embedded workflow.

262
00:31:55.370 --> 00:32:00.199
Leon: very easy, manage multiple related packages.

263
00:32:00.440 --> 00:32:01.370
Leon: There are.

264
00:32:02.690 --> 00:32:12.529
Leon: Cargo is a fantastic tool, especially for the beginners. It allows you to do everything quickly and very, very efficiently.

265
00:32:12.650 --> 00:32:20.600
Leon: You really should read into the cargo abilities, because well, I do believe many of you have used cargo in past, but

266
00:32:21.710 --> 00:32:29.580
Leon: many of us rust developers are not quite aware about the whole list of

267
00:32:30.330 --> 00:32:34.820
Leon: things you can do with cargo. It's a very nice reading.

268
00:32:36.020 --> 00:32:41.759
Leon: So before we jump to some code, any questions.

269
00:32:43.920 --> 00:32:44.930
Leon: That's nice.

270
00:32:46.000 --> 00:32:49.639
Leon: Oh, that's from the meetup. We had refreshments there.

271
00:32:53.060 --> 00:32:55.549
Leon: Because I'm going to show you some code.

272
00:32:58.880 --> 00:32:59.880
Leon: And.

273
00:33:00.630 --> 00:33:17.039
Leon: by the way, you can check out all the examples here because we will. We don't have much time for the examples. But you can use this repo I created just for the sake of this meetup. I will.

274
00:33:19.280 --> 00:33:26.410
Leon: Here is the chat. I will put it through right here, so you can access it all right.

275
00:33:27.280 --> 00:33:34.150
Leon: Good. So we have this code over here.

276
00:33:34.640 --> 00:33:39.439
Leon: and let's let me see. All right. So we have the 1st example here. Now.

277
00:33:39.440 --> 00:33:42.209
Gabor Szabo: Enlarge a little. Sorry. Can you enlarge a little bit? The font

278
00:33:42.630 --> 00:33:46.599
Leon: No, of course I can. Just a second

279
00:33:47.380 --> 00:33:50.020
Gabor Szabo: It's embedded. You can't enlarge the phones

280
00:33:50.925 --> 00:34:01.540
Leon: Alright, so we're good all right. So

281
00:34:02.650 --> 00:34:11.650
Leon: let's see. So this is the 1st example. I believe this. It will be the most important one we have the main

282
00:34:11.909 --> 00:34:15.960
Leon: here. So, as we said, no Std and no main.

283
00:34:16.449 --> 00:34:21.187
Leon: then we import. We use a cortex, m,

284
00:34:22.199 --> 00:34:24.449
Leon: and we don't really use it.

285
00:34:24.560 --> 00:34:29.120
Leon: Why do we do this, then, for default, critical section implementation.

286
00:34:29.820 --> 00:34:37.310
Leon: And when we are not using the micro bit create. We will be working with this M.

287
00:34:37.420 --> 00:34:48.369
Leon: With this board called Micro bit. It's from. It's by BBC, you can check it out and you have some. I believe you. You even have links in the in the repo.

288
00:34:48.550 --> 00:34:54.909
Leon: and this is just a board with few Leds and stuff on it, and

289
00:34:55.219 --> 00:34:58.379
Leon: and it's very easy to use. Very cheap.

290
00:34:58.550 --> 00:35:00.650
Leon: Very nice for beginners.

291
00:35:01.624 --> 00:35:14.789
Leon: So yeah. So we have the when we are not using the micro bit crate, and we don't hear. So we are not like using hull or a a Bsp.

292
00:35:15.130 --> 00:35:22.549
Leon: but we are using cortex M, so we are applying directly to the CPU of cortex. M,

293
00:35:22.700 --> 00:35:27.840
Leon: we're using the entry point and we have the rtt.

294
00:35:28.455 --> 00:35:33.849
Leon: the panic. Rt target simply brings us the panic section that we don't.

295
00:35:34.422 --> 00:35:44.300
Leon: We don't need to create it manually, it does it for us. So when you use this panic, then it will just add the handler function for you.

296
00:35:44.560 --> 00:35:54.579
Leon: and then Rt, rtt, well, I don't remember the abbreviation, but Rtt is just the Prince.

297
00:35:54.700 --> 00:36:06.210
Leon: This is a protocol by seger, and used for debugging over different debuggers, and you will immediately see how it works.

298
00:36:06.620 --> 00:36:09.840
Leon: So we have our entry function.

299
00:36:10.540 --> 00:36:13.079
Leon: We have the Rtt. Init print.

300
00:36:13.810 --> 00:36:22.350
Leon: so initialize it for printing print a hello message to the rt, just like our print line. This is the function.

301
00:36:22.620 --> 00:36:26.960
Leon: and that's it. And then we're stuck in the loop forever.

302
00:36:27.230 --> 00:36:32.170
Leon: Now, how do we actually build it and work with it?

303
00:36:32.670 --> 00:36:33.870
Leon: It's quite easy.

304
00:36:34.090 --> 00:36:45.180
Leon: I go into the example. One directory am cargo embed.

305
00:36:46.970 --> 00:36:50.850
Leon: This is everything we need to just build it and run it.

306
00:36:51.120 --> 00:36:57.670
Leon: It it it built it, flashed it, and ran it just for us.

307
00:36:57.850 --> 00:37:05.830
Leon: So we have. Hello from micro bit, and this is the output of the Rtt. Interface

308
00:37:08.330 --> 00:37:09.789
Leon: as easy as that

309
00:37:10.490 --> 00:37:12.560
Gabor Szabo: It's connected with a USB, right?

310
00:37:13.360 --> 00:37:18.359
Leon: Yeah, it's connected with USB, and it creates 2 serial ports on the system.

311
00:37:18.870 --> 00:37:27.330
Leon: Okay, so this is all. Going through a serial Port M.

312
00:37:27.630 --> 00:37:33.040
Leon: Maybe you can even see the port over here. No, you cannot, whatever.

313
00:37:33.850 --> 00:37:43.270
Leon: Right? So now let's take something more interesting. You know we are all embedded guys. We like seeing Leds

314
00:37:43.660 --> 00:37:45.580
Leon: on and off, right?

315
00:37:46.250 --> 00:37:52.639
Leon: So here we have the matrix. And now here I already start using the micro bit crate.

316
00:37:52.980 --> 00:37:54.820
Leon: This is the Bsp.

317
00:37:55.310 --> 00:38:01.509
Leon: okay, so it allows me to use the board in the in a very, very easy way.

318
00:38:02.030 --> 00:38:08.190
Leon: For example. Well, you you can see just some sort of array over here.

319
00:38:08.940 --> 00:38:13.669
Leon: and this is a kind of irrelevant function. We will not go into it.

320
00:38:13.910 --> 00:38:16.500
Leon: But then, what what do we do here.

321
00:38:16.620 --> 00:38:19.689
Leon: Just look how simple the code is.

322
00:38:20.020 --> 00:38:25.289
Leon: Board. Take. Okay, so we initialize the board.

323
00:38:25.420 --> 00:38:28.700
Leon: Then we initialize a timer because we need a timer here.

324
00:38:28.920 --> 00:38:31.920
Leon: So we use the timer 0 of this board.

325
00:38:32.380 --> 00:38:44.860
Leon: and we need the display, the display in this case, because this is bsp, so it has a display, for, as you can see, here, you have the Leds and metric matrix of Leds.

326
00:38:45.410 --> 00:38:54.430
Leon: So now you just loop through the matrix just by creating display show, and then

327
00:38:56.140 --> 00:39:04.089
Leon: and that's about it. You just have the display show function that you display the the array. So let's

328
00:39:05.680 --> 00:39:10.350
Leon: let's run it. No, not here. City X.

329
00:39:11.400 --> 00:39:14.850
Leon: Example 2 cargo and bed.

330
00:39:22.560 --> 00:39:25.330
Leon: Alright. Now, what do we see here?

331
00:39:26.340 --> 00:39:27.910
Leon: Hey?

332
00:39:28.960 --> 00:39:31.879
Leon: This is the best moment in my career.

333
00:39:33.870 --> 00:39:43.460
Leon: Alright! So we have successfully and created a lit carousel.

334
00:39:44.030 --> 00:39:49.200
Leon: Now I want to show you just a bit inside of how the things work

335
00:39:49.330 --> 00:39:54.880
Leon: and what what files do you need to use to make this all happen.

336
00:39:56.330 --> 00:40:02.489
Leon: Unlike with simple rust that you use with an operating system.

337
00:40:03.190 --> 00:40:09.560
Leon: Here, apart from your main, you have a few things. Let's start with the standard things.

338
00:40:09.680 --> 00:40:11.739
Leon: cargo.com.

339
00:40:12.240 --> 00:40:19.399
Leon: It just includes the libraries and everything. I mean the crates we have here. Everything is quite standard.

340
00:40:19.580 --> 00:40:28.324
Leon: Alright. Don't forget about the features using cortex, M. Critical section inline awesome. And so you need to

341
00:40:30.190 --> 00:40:31.620
Leon: to to get them in.

342
00:40:32.120 --> 00:40:39.670
Leon: But then you have 2 additional files which are very important.

343
00:40:39.900 --> 00:40:41.900
Leon: You have the build dot Rs.

344
00:40:42.510 --> 00:40:55.919
Leon: the build dot. Rs script is quite standard. It's not special for every project you don't have to change it, but you might want to change it eventually. I will not just go through everything here.

345
00:40:56.380 --> 00:41:01.980
Leon: It's quite clear, but not a very obvious from the start.

346
00:41:02.430 --> 00:41:09.490
Leon: It simply makes sure that your Linker would will take this Linker script.

347
00:41:09.790 --> 00:41:20.240
Leon: because, as usual, with a embedded developer embedded development on Mcus. You need to specify some

348
00:41:20.870 --> 00:41:29.070
Leon: initial data for the Mcu to run. So here it's quite simple. You have the flash memory, and you have the RAM

349
00:41:29.680 --> 00:41:30.770
Leon: very easy.

350
00:41:32.200 --> 00:41:40.989
Leon: and the build dot Rs simply makes sure that this is the file that's used by the Linker.

351
00:41:42.590 --> 00:41:50.390
Leon: and the last thing. Oh, not the last one, actually. But let's look at it.

352
00:41:50.530 --> 00:41:53.070
Leon: The embed dot tunnel.

353
00:41:53.450 --> 00:42:03.250
Leon: This file is actually specifies everything. What everything you do with cargo embed the cargo embed command

354
00:42:03.640 --> 00:42:05.270
Leon: deals with this file

355
00:42:05.420 --> 00:42:12.060
Leon: so you can set up the default protocol here, like currently your probe is single wire debugging.

356
00:42:13.300 --> 00:42:18.189
Leon: You can also specify Jtag. It depends on your debugger. You just specify it over here.

357
00:42:18.980 --> 00:42:25.529
Leon: Then you have the general settings for the target, like, for example, you need to specify what

358
00:42:25.750 --> 00:42:27.119
Leon: chip you're using.

359
00:42:27.640 --> 00:42:35.550
Leon: So in our case, this is the Nordic chip. Nrf, whatever you need to get this information from the

360
00:42:37.110 --> 00:42:39.649
Leon: from the Gp you're using right.

361
00:42:40.210 --> 00:42:43.210
Leon: and you so you can specify chip models.

362
00:42:43.340 --> 00:42:50.520
Leon: The default reset operation here is that you can halt after flashing.

363
00:42:50.900 --> 00:42:52.740
Leon: Why is it useful?

364
00:42:53.130 --> 00:42:58.649
Leon: When is it useful to halt after flashing and stop the program from running.

365
00:42:59.100 --> 00:43:00.969
Leon: It's during your debug.

366
00:43:01.710 --> 00:43:02.670
Leon: Okay?

367
00:43:03.850 --> 00:43:10.769
Leon: And then, you have the default. Rt, which is either enabled or not.

368
00:43:11.040 --> 00:43:18.009
Leon: For example, in this case I don't use Rtt. At all. There was no rtt in my code.

369
00:43:19.190 --> 00:43:20.230
Leon: any of it.

370
00:43:20.760 --> 00:43:23.859
Leon: So why did I enable Rtt, anyway?

371
00:43:24.180 --> 00:43:34.470
Leon: Because and why do I initialize it, anyway? Because for debugging, I still use Rtt Target target. And

372
00:43:35.020 --> 00:43:39.589
Leon: yeah, the panic target. So if I get any panic. It would print me

373
00:43:39.830 --> 00:43:44.049
Leon: something on rtt, so I will know I have a panic in my code.

374
00:43:44.880 --> 00:43:53.840
Leon: but I can drop it. Okay. So, for example, if I change it here to false, and I

375
00:43:55.410 --> 00:44:02.730
Leon: and I build it again and run it, you will see that Rtt. Is no longer enabled on start.

376
00:44:04.360 --> 00:44:09.620
Leon: Okay, the the the board is still running, of course.

377
00:44:10.340 --> 00:44:11.280
Leon: That's it.

378
00:44:12.080 --> 00:44:13.550
Leon: M.

379
00:44:14.150 --> 00:44:18.950
Leon: You also have the default. Gdb, and note this. It's enabled.

380
00:44:19.590 --> 00:44:21.680
Leon: The Gdb. Is enabled here.

381
00:44:22.400 --> 00:44:40.040
Leon: and and you can specify the port. Let's get to Gdb. No worries and default flashing. Of course you can enable the default flashing, but you can. If you have a different flashing algorithm or program, you just specify it here. No worries at all. It's all very configurable.

382
00:44:40.960 --> 00:44:44.720
Leon: Now let's look on the

383
00:44:46.080 --> 00:44:47.630
Leon: Gdb stuff.

384
00:44:47.940 --> 00:44:56.499
Leon: So we are here, and I will quickly. Start the debugging.

385
00:44:58.290 --> 00:45:04.270
Leon: so I will just copy and paste some commands. It will not annoy you, M.

386
00:45:04.940 --> 00:45:09.579
Leon: I'm running the Gdb multi arc with my target. Executable.

387
00:45:11.820 --> 00:45:20.709
Leon: Alright. So I'm inside. Gdb, now with my target. Now I want to connect to my remote, which is by default

388
00:45:20.830 --> 00:45:22.119
Leon: on this port

389
00:45:22.630 --> 00:45:25.209
Gabor Szabo: We're doing a lot of the phones here, too. Please

390
00:45:25.210 --> 00:45:28.329
Leon: I don't know. Maybe let's see. Yes.

391
00:45:28.860 --> 00:45:31.209
Leon: Now I will need to.

392
00:45:32.620 --> 00:45:33.360
Leon: Yeah.

393
00:45:34.060 --> 00:45:34.670
Gabor Szabo: Thanks.

394
00:45:37.100 --> 00:45:39.890
Leon: All right, M.

395
00:45:40.370 --> 00:45:44.254
Leon: And now and now I can.

396
00:45:45.290 --> 00:45:55.719
Leon: I believe it does it by by itself. But I source the Gdb. In. It doesn't matter I will just configure the dashboard a little bit.

397
00:45:57.350 --> 00:45:58.480
Leon: Okay.

398
00:45:58.630 --> 00:46:06.120
Leon: And let's have a and didn't I?

399
00:46:08.620 --> 00:46:11.169
Leon: That's kind of strange.

400
00:46:11.970 --> 00:46:14.659
Leon: Okay. No, wait.

401
00:46:16.691 --> 00:46:20.200
Leon: Yes, let's start it again.

402
00:46:21.370 --> 00:46:25.559
Leon: Oh, yes, right? Right? Right? Right? So that's the problem.

403
00:46:25.970 --> 00:46:31.509
Leon: Let's see. Where am I? Target?

404
00:46:33.570 --> 00:46:34.175
Leon: Yeah.

405
00:46:35.900 --> 00:46:38.220
Leon: Okay, debug.

406
00:46:39.460 --> 00:46:43.780
Leon: And hmm.

407
00:46:47.940 --> 00:46:51.570
Leon: okay, all right, all right. All right.

408
00:46:51.740 --> 00:46:54.710
Leon: So where were we in

409
00:47:02.520 --> 00:47:06.030
Leon: and breakpoint?

410
00:47:06.910 --> 00:47:11.299
Leon: Yeah, the program is not being run.

411
00:47:11.740 --> 00:47:13.739
Leon: What's going on?

412
00:47:16.180 --> 00:47:25.700
Leon: This is what happens when you try to show something quickly the exact format error.

413
00:47:30.730 --> 00:47:32.510
Leon: That's strange.

414
00:47:34.050 --> 00:47:36.960
Leon: Let's try something else.

415
00:47:53.600 --> 00:47:57.630
Leon: Something is wrong about it.

416
00:47:58.020 --> 00:48:14.219
Leon: Well, I will not. Continue with this direction, because I will waste all your time, and I still want to show you stuff. But, generally speaking, debugging is quite easy, because you have the debugging built in already.

417
00:48:14.540 --> 00:48:20.050
Leon: Alright, so you can simply connect and debug your code.

418
00:48:20.250 --> 00:48:26.780
Leon: And it's all very convenient, because you don't need to even add another line over there.

419
00:48:27.060 --> 00:48:45.689
Leon: And one more thing to mention is that in config dot tunnel you can specify how to work with. Your specific configuration like in this case. I've configured it all arm and target OS, and so on. Use

420
00:48:46.100 --> 00:48:49.860
Leon: the Linker script the specific Linker script.

421
00:48:50.398 --> 00:48:57.150
Leon: That's quite easy, or you can provide your own specific target if you want really up to you.

422
00:48:57.290 --> 00:49:06.119
Leon: So these are the main files that you have here. We mentioned the embed, the tumul, the most important file for cargo embed.

423
00:49:06.240 --> 00:49:15.049
Leon: We have the cargo standard and the bill dot Rs. Which selects the Linker script.

424
00:49:15.250 --> 00:49:17.070
Leon: and the Linker script itself.

425
00:49:17.690 --> 00:49:22.539
Leon: That's everything we need to get going, and it's all available on the net.

426
00:49:23.751 --> 00:49:37.039
Leon: Just another example. Here, let's say and then see the and let's take

427
00:49:37.200 --> 00:49:46.100
Leon: example for the accelerometer and cargo embed.

428
00:50:02.390 --> 00:50:03.220
Leon: Yep.

429
00:50:03.820 --> 00:50:10.549
Leon: So now we have the values from the accelerometer on this board, so you can see the values

430
00:50:10.820 --> 00:50:13.559
Leon: are changing in the 3 directions.

431
00:50:14.420 --> 00:50:22.879
Leon: And yeah, and how you achieve this thing quite easily.

432
00:50:23.400 --> 00:50:30.470
Leon: And you have the the code right here in front of you.

433
00:50:30.640 --> 00:50:38.522
Leon: So, and you have the from microbeat you get the

434
00:50:39.874 --> 00:50:45.109
Leon: twi interface, the uart interfaces, timers, and everything.

435
00:50:45.710 --> 00:50:54.900
Leon: Then in this case you initialize the board itself as usual. Board. Take, then, you initialize timer.

436
00:50:55.500 --> 00:50:57.290
Leon: You have the serial.

437
00:50:57.820 --> 00:51:06.070
Leon: You are new, as you can see, very everything is very straightforward here, right? It's the simplest thing you can think about to

438
00:51:06.220 --> 00:51:09.709
Leon: initialize a serial interface.

439
00:51:09.960 --> 00:51:15.809
Leon: and then you set up the I square C interface the the 2 wire interface

440
00:51:16.310 --> 00:51:21.359
Leon: right also, as you can see, very easy for supply frequency, and that's about it.

441
00:51:21.750 --> 00:51:27.540
Leon: And then here, in this case, it has a sensor called Lsm. 303,

442
00:51:27.890 --> 00:51:34.160
Leon: and for which we have a separate crate, which is totally a totally unrelated crate.

443
00:51:34.330 --> 00:51:36.749
Leon: But it provides us access.

444
00:51:36.890 --> 00:51:38.139
Leon: Who the censor?

445
00:51:39.340 --> 00:51:44.559
Leon: We initialize the sensor. We set the accelerator mode, and so on.

446
00:51:44.830 --> 00:51:52.399
Leon: and then, for we delay for 50. We sleep for 50 ms.

447
00:51:53.610 --> 00:51:56.240
Leon: and then we have the acceleration.

448
00:51:56.960 --> 00:52:02.539
Leon: We are reading the acceleration in one command, one simple command over I square C,

449
00:52:04.090 --> 00:52:09.739
Leon: and that's about it. We write it to the serial line, and then we r print. Line it as well.

450
00:52:10.110 --> 00:52:16.700
Leon: Now I wanted to show you the prints from the serial line. However.

451
00:52:16.800 --> 00:52:22.220
Leon: it seems like I have some issue here

452
00:52:25.250 --> 00:52:35.769
Leon: at the moment, because when I connect to the serial line I see nothing, and that's the quite weird.

453
00:52:36.060 --> 00:52:45.029
Leon: I don't know. I see. M. 0. This is the this is the interface, but

454
00:52:45.470 --> 00:52:51.270
Leon: I see nothing on it. So hmm!

455
00:52:53.740 --> 00:52:59.090
Leon: I think I know why I think I know why

456
00:53:04.080 --> 00:53:07.490
Leon: PTYA. CM. 0 am

457
00:53:12.823 --> 00:53:13.456
Leon: sorry.

458
00:53:20.710 --> 00:53:30.349
Leon: I think I know why DTYA. CM. 0, and then.

459
00:53:35.780 --> 00:53:36.870
Leon: and

460
00:53:44.330 --> 00:53:47.530
Leon: I think it was 1,200.

461
00:53:48.090 --> 00:53:49.350
Leon: Let's see.

462
00:53:51.610 --> 00:53:52.690
Leon: Okay?

463
00:53:56.540 --> 00:54:03.749
Leon: Oh, hey, I got training. So it was configured for a different bowed rate or wrong bout rate.

464
00:54:04.890 --> 00:54:09.540
Leon: So you can see that now it prints to the serial interface

465
00:54:09.780 --> 00:54:14.180
Leon: I'm connected over minicom, and then you can also see.

466
00:54:14.400 --> 00:54:17.849
Leon: And you can also see it over here.

467
00:54:18.340 --> 00:54:21.130
Leon: and I closed it. So I need to

468
00:54:21.460 --> 00:54:24.279
Leon: suppose I need to rerun it. But whatever?

469
00:54:24.690 --> 00:54:35.689
Leon: And yeah, so in this way. As you can see, printing to the serial port

470
00:54:36.410 --> 00:54:40.840
Leon: could not be simpler than with rust here, and

471
00:54:41.040 --> 00:54:51.537
Leon: and I remember my initial projects in C with embedded with St. Micro electronics Mcus, and with

472
00:54:52.360 --> 00:54:57.460
Leon: ziologue and such. It was a nightmare here, you know.

473
00:54:58.880 --> 00:55:08.409
Leon: very easy and straightforward. So my my point in this presentation, because we need to wrap up.

474
00:55:09.610 --> 00:55:13.886
Leon: The main thing I'm trying to

475
00:55:15.580 --> 00:55:17.929
Leon: to to show here is that

476
00:55:18.290 --> 00:55:22.880
Leon: 1st of all, the trust is a very modern

477
00:55:23.080 --> 00:55:28.349
Leon: language and ecosystem for developing for bare metal.

478
00:55:29.260 --> 00:55:32.559
Leon: It's very convenient. It's very

479
00:55:34.510 --> 00:55:36.099
Leon: It's very mature.

480
00:55:36.630 --> 00:55:46.920
Leon: It's not like. It's something that you need to, you know. Spend your nights on configuring.

481
00:55:47.380 --> 00:55:51.720
Leon: It's mature. You just install it and run it. It's very straightforward.

482
00:55:53.900 --> 00:55:58.460
Leon: more. Moreover, if you need.

483
00:55:58.900 --> 00:56:02.860
Leon: you know, in in my view, if you want to. To.

484
00:56:03.340 --> 00:56:09.739
Leon: To begin working with bare metal programming today with mcus.

485
00:56:10.350 --> 00:56:12.470
Leon: I think that there is no

486
00:56:12.730 --> 00:56:19.819
Leon: reason to choose C over rust at all, because rust will

487
00:56:19.970 --> 00:56:25.420
Leon: rust and cargo will provide you everything you need get going, and

488
00:56:25.580 --> 00:56:35.359
Leon: in a in a safe memory, safe way. So why bother even dealing with sea and whatever?

489
00:56:35.540 --> 00:56:44.768
Leon: So if you have a new project for bare metal machines. There is absolutely no reason to choose

490
00:56:45.730 --> 00:56:49.169
Leon: to choose C over us. That's about it.

491
00:56:50.090 --> 00:56:57.130
Leon: So, Gabbara. I'm quite done. I have. I didn't show everything, but we were. We're out of time

492
00:56:58.190 --> 00:57:01.009
Leon: You you guys can use the repo to

493
00:57:01.120 --> 00:57:03.370
Leon: to check out the rest of the things

494
00:57:04.890 --> 00:57:19.930
Gabor Szabo: Okay. So I'll 1st of all, thank you very much. I just checked. I had this micro bit that I bought like a year ago, and I still didn't do anything with it. It's version 2. So I'll have to see if if this code I see that

495
00:57:19.930 --> 00:57:24.675
Leon: Well, well, this for version 2, as you can see.

496
00:57:26.080 --> 00:57:34.010
Leon: over here, I think. I put it year. Yes.

497
00:57:34.220 --> 00:57:47.210
Leon: So over here you can we work for version 1 1.5. Use with this one for the version, 2 works with 7

498
00:57:47.370 --> 00:57:48.439
Leon: v. 7

499
00:57:48.650 --> 00:57:49.390
Gabor Szabo: Okay.

500
00:57:49.390 --> 00:57:50.559
Leon: That's the difference.

501
00:57:51.280 --> 00:58:05.380
Leon: And by the way, you can find a lot about it on the Internet, it's all there. You just type in. Oh, by the way, I want to even show you how it works. You see, you go into rust for embedded.

502
00:58:06.060 --> 00:58:11.460
Leon: and you reach the embedded Rust book, I really.

503
00:58:11.670 --> 00:58:25.830
Leon: I really like the the documentation in Rust for embedded because when I started with embedded programming there was no such thing for C. There are plenty of C books.

504
00:58:26.020 --> 00:58:31.130
Leon: but nothing close to to stuff like this.

505
00:58:31.940 --> 00:58:38.819
Leon: So you have the embedded rust manuals here for everything, you know

506
00:58:39.110 --> 00:58:45.689
Leon: whatever. And then you have the bare metal rust section, which is very specific

507
00:58:45.860 --> 00:58:54.510
Leon: with all the installations and explanations about how to set things up for different operating systems, and so on.

508
00:58:55.420 --> 00:59:05.319
Leon: And then, for example, the discovery here section, it's all about specifically about the micro bit

509
00:59:06.040 --> 00:59:13.420
Leon: alright. This is the everything here in discovery is about micro bit. And here is just another example of

510
00:59:13.600 --> 00:59:15.490
Leon: working with Stm.

511
00:59:15.980 --> 00:59:23.190
Leon: 32 specific board. Everything is just like with all the examples and everything

512
00:59:23.500 --> 00:59:26.470
Leon: very clear, very straightforward, and so on.

513
00:59:29.860 --> 00:59:30.840
Gabor Szabo: Excellent.

514
00:59:32.050 --> 00:59:46.409
Gabor Szabo: So thank you very much if anyone has any questions who wants to still ask it during the presentation. But we didn't have, through all the presentation. Maybe you have something now, and do go ahead and ask.

515
00:59:46.800 --> 00:59:47.960
Gabor Szabo: and

516
00:59:48.960 --> 01:00:14.850
Gabor Szabo: if not, then we are going to shut down the the recording, and then you can, because you are here. You have this privilege that you can stay around, and then we can have a continue the discussion. If you're watching on the Youtube. Then you have the privilege to click the like button and follow the channel. And so, Leon, thank you very much for this presentation, and thanks everyone for

517
01:00:14.960 --> 01:00:22.680
Gabor Szabo: and for attending, and so hope to see you soon at a different presentation.

518
01:00:22.960 --> 01:00:23.830
Gabor Szabo: Bye, bye.

