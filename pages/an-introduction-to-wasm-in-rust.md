---
title: An introduction to WASM in Rust with Márk Tolmács
timestamp: 2025-01-16T10:30:01
author: szabgab
published: true
description:
---

In this virtual event [Márk Tolmács](https://marktolmacs.com/), the author of [wasm2map](https://crates.io/users/mtolmacs) will introduce us to WASM using Rust.

This is part of the [Rust Maven live](/live) series.

WebAssembly (or WASM) is a very new concept brought to life by the ever-increasing need for performance and complexity management, with the additional promise of letting adopters re-use their existing solutions in system-level languages, like C and C++ (and Rust!). While the promises are mostly true, compiling and running WebAssembly requires some preparation and awareness of its limitations. In this short pair programming session, I walk you through the compilation of your first  WebAssembly program, run it in your browser, access the DOM,  interface with JavaScript, and finally run the code in nodejs via the use of the WebAssembly System Interface (WASI).

Mark is a veteran IT professional who created complex software solutions for over 20 years (most notably at Ustream and IBM). The wide range of product and technical expertise he accumulated led him to his current endeavor consulting on the hardest challenges software companies face. He first stumbled upon Rust 5 years ago. He took advantage of the power and elegance of this newcomer of a system-level language to deliver custom tooling in various projects with great success.


![Márk Tolmács](images/mark-tolmacs.jpeg)

* [A Gentle Introduction to WebAssembly in Rust (2025 Edition)](https://dev.to/marktolmacs/a-gentle-introduction-to-webassembly-in-rust-2025-edition-1iac)
* [Vite](https://vite.dev/)
* [The repository](https://github.com/szabgab/wasm-on-web-2025-01-15-with-mark-tolmacs)


{% youtube id="kdHfOhBdv1E" file="2025-01-15-rust-english-wasm-with-mark-tolmacs.mp4" %}


## Transcript

1
00:00:01.910 --> 00:00:04.709
Gabor Szabo: Okay, so welcome to the

2
00:00:05.000 --> 00:00:33.690
Gabor Szabo: rust Maven, meet up or meeting, or whatever we have. And this is the we are uploading it to the Code Maven channel on Youtube, this video that contains various other types of videos as well. So it's rust python, some pearl. And and so welcome to the meeting. I'm really happy that there are like 15. We are like 15 people here and welcome to people who are watching the video.

3
00:00:33.820 --> 00:00:45.829
Gabor Szabo: My name is Gabor. I usually teach rust and teach python and some version control and test automation and help companies introduce test automation.

4
00:00:45.960 --> 00:01:11.640
Gabor Szabo: And recently, I started this, I used to record videos and and do these live sessions, but I thought it was much better to invite guests who know way way more things than I do, and let them explain things, and even teach me things. So I found Mark, and I was really happy that I can learn some webassembly from him.

5
00:01:11.770 --> 00:01:24.870
Gabor Szabo: So, Mark, welcome, please. Actually, I think I'm sharing the screen instead of instead of talking into the camera. So I'm going to stop the share.

6
00:01:25.330 --> 00:01:35.025
Gabor Szabo: And hopefully, you can see me now, so Hi and I know it's your turn, mark. Please introduce yourself, and then go from there.

7
00:01:36.770 --> 00:01:40.278
Márk Tolmács: Yeah, sure. Thank you. I don't know if

8
00:01:41.570 --> 00:02:01.670
Márk Tolmács: I need to talk much about myself. But just in a couple of sentences. I actually am working with multiple technologies. Rust is one of them, albeit not recently, unfortunately, but previous to my current engagements.

9
00:02:01.670 --> 00:02:21.850
Márk Tolmács: I spent a lot of time figuring out how Webassembly can be used in the browser in sort of an efficient way or in a in terms of development experience. I started to work on a crate which helped creating source maps for webassemblies which is

10
00:02:21.900 --> 00:02:27.219
Márk Tolmács: not something currently present in the rust webassembly ecosystem.

11
00:02:27.490 --> 00:02:44.400
Márk Tolmács: and since then I've worked on many other tooling projects with rust. Right now, thanks to you, I'm here to introduce everybody to basically Webassembly and do it with rust

12
00:02:44.840 --> 00:02:56.769
Márk Tolmács: and see how exactly we can take it to a level where you can actually get started building real applications and real solutions with rest in Webassembly.

13
00:02:57.982 --> 00:03:07.047
Gabor Szabo: Nice. Okay, I have not. I don't know much about Web assembly, some little theory of it. I just wanted to say also that?

14
00:03:07.850 --> 00:03:12.204
Gabor Szabo: yeah. Yeah. So let's actually share the screen. And then I

15
00:03:15.250 --> 00:03:18.140
Gabor Szabo: just making sure that I'm sharing the right screen

16
00:03:19.200 --> 00:03:22.930
Gabor Szabo: hopefully, you're seeing your article right? So I'm putting yes.

17
00:03:22.930 --> 00:03:33.239
Gabor Szabo: this in the in the chat for anyone who would like to get take a look at the the article. Mark sent it to me

18
00:03:34.190 --> 00:03:56.170
Gabor Szabo: couple of days ago, and obviously I postponed everything, and I started to go read it only just today, and he asked me to do some installations before we get to the to the video. So you might also need to to do some of the the things. If you'd like to follow this, or later later on, you would like to implement the same things.

19
00:03:56.270 --> 00:04:11.977
Gabor Szabo: So what we, what I have is I installed all this stuff, including setting up. Let me enlarge the font a little bit, including setting up this repository. This wasm on web and

20
00:04:12.890 --> 00:04:30.790
Gabor Szabo: I opened visual studio code on that in that project, and I even opened the file because I couldn't figure out why I don't see it large enough. But anyway, we are here and and that's it. Yeah. I have no idea what what I did. I just followed you.

21
00:04:31.177 --> 00:04:40.089
Márk Tolmács: Cool. I'm gonna walk you through but before we do that, let's just spend a couple of minutes and talk about

22
00:04:40.360 --> 00:04:59.969
Márk Tolmács: what exactly is the significance of Webassembly. Why, exactly, Webassembly? The new kid on the block is the solution proposed by many companies and nonprofits, and that will lead us to our example project, which you just open and everybody can see.

23
00:05:00.626 --> 00:05:02.289
Márk Tolmács: So 1st of all.

24
00:05:02.630 --> 00:05:21.519
Márk Tolmács: Webassembly, the the promise of Webassembly is basically a runtime, which is platform independent. That means it's not like a program you use like an Exe a windows program which you can only run on windows and x 86,

25
00:05:21.570 --> 00:05:44.230
Márk Tolmács: but you have to have a separate binary which can only run on Mac OS, and a separate 3rd binary to run on Linux, etc. Etc. So this is basically Webassembly's promise. You would have just one compiled binary. It can run anywhere. And obviously, if you're experienced with the history of software development.

26
00:05:44.868 --> 00:05:55.479
Márk Tolmács: You know that this is not the 1st time this is attempted. One notable example, probably the older ones in the audience will remember, is Java.

27
00:05:55.520 --> 00:06:21.150
Márk Tolmács: the Java virtual machine which promised the same thing. Unfortunately, it was probably too soon, and even though Java is very prominent in some of the areas, and sectors in it like banks and other solutions. It really is not relevant anymore for us. The other one, significant and notable before Webassembly, Wasnet microsoft.net.

28
00:06:21.220 --> 00:06:26.802
Márk Tolmács: It's still sort of actively maintained and actively

29
00:06:27.830 --> 00:06:47.069
Márk Tolmács: basically cared for by Microsoft. They basically built their full platform on.net, and it will not go away. But it's still limited. So basically, mostly windows. Even if you can run it on unix operating systems easily, people don't tend to use it. It's it's a very closed space.

30
00:06:47.170 --> 00:06:49.120
Márk Tolmács: So.

31
00:06:49.600 --> 00:07:07.660
Márk Tolmács: and and honestly, one of the the aspects of.net which hurts.net pretty much, in my opinion, is that they don't really have a web run time so they can't run.net applications in web and support many, many use cases so that limits their

32
00:07:07.880 --> 00:07:33.939
Márk Tolmács: potential use cases. So this is where Webassembly fits into the whole idea. In 2,017 couple of companies and nonprofits came together and figured they might need to do something, but it has to be open, not just not in terms of open source exclusively. But what I mean is openly specified and openly maintained by a consortium. So that's Webassembly.

33
00:07:34.040 --> 00:07:58.510
Márk Tolmács: And honestly, it seems that it worked for the World Wide Web. And it's probably going to work for Webassembly. But I want to point out that Webassembly is still very, very young in its lifecycle, and it has a lot of problems, even though you'll see today that you can get started in rust and whatever you can probably do the same thing in CC. Plus and get great results.

34
00:07:58.550 --> 00:08:11.409
Márk Tolmács: But we will also talk about some of the nitpicky problematic things which you might run into early in your Webassembly career and hopefully how to solve it. So it's not the best

35
00:08:11.530 --> 00:08:32.639
Márk Tolmács: developer experience, but it's usable, which is good enough for us most of the time. So what exactly is Webassembly other than a free and open standard? It's basically an instruction set and eyes are. Basically, it's a 32 bit instruction.

36
00:08:32.640 --> 00:08:35.269
Gabor Szabo: Just a second, just a second. I have a question actually, today.

37
00:08:35.270 --> 00:08:35.909
Márk Tolmács: Good.

38
00:08:36.283 --> 00:08:50.819
Gabor Szabo: Please. And you know, I hopefully, you know how to raise your hand in in not like personally, but in zoom. So please raise your hand if you have ever written any code in assembly, in any assembler.

39
00:08:55.500 --> 00:08:57.250
Gabor Szabo: Okay, there is one.

40
00:08:57.700 --> 00:09:10.299
Gabor Szabo: I mean, I I wrote like 40 years ago assembler. And I call 2, 2 people, 2, 3 people. Okay. So there are a few people who wrote assembler. That's nice.

41
00:09:11.010 --> 00:09:12.885
Gabor Szabo: Oh, wow! Okay,

42
00:09:13.950 --> 00:09:21.600
Márk Tolmács: Those are long forgotten memories for me. But yeah, I'm old enough to remember and even use some assembly.

43
00:09:22.522 --> 00:09:27.347
Márk Tolmács: But yeah, it's actually a not a

44
00:09:28.100 --> 00:09:45.879
Márk Tolmács: an unexpected comparison, because Webassembly is indeed sort of that level of complexity as assembly was for the native platforms. It actually is the same as basically an instructions and architecture. It's just

45
00:09:46.300 --> 00:09:48.530
Márk Tolmács: in a virtual machine. Basically.

46
00:09:48.780 --> 00:10:11.799
Márk Tolmács: you would say, so. It's basically a virtual machine. But it's I'm hesitant to call it a virtual machine, because it lacks a lot of functionalities. People usually associate with virtual machines like there is no garbage collection, anything high level. It really is very comparable to what you would expect from a very simple assembly.

47
00:10:11.800 --> 00:10:21.229
Márk Tolmács: Powered architecture and and instruction set, basically just what what you would use. Obviously the simplification is.

48
00:10:21.310 --> 00:10:44.210
Márk Tolmács: can be powerful in terms of like execution, speed, and performance, even though again, as I mentioned, it, runs in an isolated virtual machine, so the speed will never compare really, ever in the future. It won't be able to compare to like runtimes on the native platform so like native binaries. But that's not the point.

49
00:10:44.210 --> 00:11:04.459
Márk Tolmács: The point is accessibility is that it runs on that primarily but you can use it basically as a virtual machine and run Webassembly code on your desktop as well, even though that's very much in the early phases. Basically it works. It can do system calls whatever basically A/C

50
00:11:04.530 --> 00:11:09.930
Márk Tolmács: or assembly code, translated to binary or compared to binary, could do.

51
00:11:10.190 --> 00:11:14.710
Márk Tolmács: We will see that at the end of the the event today.

52
00:11:15.410 --> 00:11:32.034
Márk Tolmács: So okay, I don't want to spend more time on the details. If you're interested in some more coherent introduction into what exactly Webassembly is, the article is a great place to start. Try to explain everything you will need

53
00:11:33.050 --> 00:11:49.190
Márk Tolmács: if you want to understand what Webassembly is. But today we only need to understand how to use the tools at our disposal to build something on Webassembly. So, as Kabor mentioned, the 1st step is setting up the tooling

54
00:11:50.690 --> 00:11:53.459
Márk Tolmács: you will need the Javascript

55
00:11:53.720 --> 00:12:10.569
Márk Tolmács: Nodejs tool space and the rust tool space. We are going to actually build a project with Nodejs and browser-based code Javascript typescript and rust at the same directory, the same source

56
00:12:10.670 --> 00:12:39.260
Márk Tolmács: route just to demonstrate that rust. And basically, Webassembly and Javascript typescript can coexist inside a single project. If that's what you want, you don't have to separate them. Okay? So the key part here, as Gobor mentioned is that I'm going to use yarn. The package manager for Javascript to demonstrate this project or the demonstrate the

57
00:12:39.390 --> 00:12:51.859
Márk Tolmács: and goal. And in this case I use vite, which is a popular framework, just because it's easier to set up just to just save on time. You don't have to use vite.

58
00:12:52.130 --> 00:12:55.250
Márk Tolmács: You can use any other framework if you have.

59
00:12:55.250 --> 00:12:57.360
Gabor Szabo: How do you spell the name? Sorry!

60
00:12:57.650 --> 00:12:58.380
Márk Tolmács: Right.

61
00:12:59.280 --> 00:12:59.950
Gabor Szabo: Like.

62
00:12:59.950 --> 00:13:01.870
Márk Tolmács: VITE fight.

63
00:13:02.490 --> 00:13:06.200
Gabor Szabo: BIT, like, okay.

64
00:13:06.670 --> 00:13:16.629
Márk Tolmács: It's it's the framework which I prefer to demonstrate with, because it's easy and very robust, has a lot of plugins which we will see, we will need

65
00:13:16.910 --> 00:13:17.450
Márk Tolmács: and.

66
00:13:17.450 --> 00:13:20.819
Gabor Szabo: If I search for it, I say byte framework.

67
00:13:21.000 --> 00:13:25.109
Márk Tolmács: No, no, not not BV, like vector.

68
00:13:29.370 --> 00:13:30.240
Márk Tolmács: yep.

69
00:13:30.560 --> 00:13:31.360
Gabor Szabo: Okay.

70
00:13:33.940 --> 00:13:36.540
Márk Tolmács: It's wide dev, most likely.

71
00:13:36.740 --> 00:13:37.260
Gabor Szabo: Okay.

72
00:13:37.260 --> 00:13:39.080
Márk Tolmács: Yep, that's that's the one.

73
00:13:40.600 --> 00:13:46.460
Márk Tolmács: So that yarn command you shown in the article. And with what we discussed

74
00:13:47.080 --> 00:13:51.569
Márk Tolmács: is basically just a quick one liner to

75
00:13:51.910 --> 00:14:10.379
Márk Tolmács: create a project folder, download a template with typescript because we're going to use typescript. Most serious people, I think, use typescript nowadays. It will be named Wasm on the web. That's you can change it to whatever you want. We just today call it

76
00:14:10.670 --> 00:14:15.950
Márk Tolmács: wasn't on the web, and the rest of it is basically just setting up git

77
00:14:16.250 --> 00:14:25.969
Márk Tolmács: because we will need get in a moment. Setting up rust. We're gonna use stable rust. Can you scroll a little bit further down.

78
00:14:25.970 --> 00:14:40.870
Gabor Szabo: You just as much as I remember here. There's the setup of the git repository, and then there are instructions to set up rust. I already had rust on my system, so I didn't have to number of cargo installs like I run this.

79
00:14:40.870 --> 00:15:00.219
Márk Tolmács: Yeah. But the key part here is that we are going to cross. Compile your machine doesn't run on Webassembly. You have to cross, compile your rust code into Webassembly. So 1st of all, this is the key part, Kabor, highlighted. You have to add, this target was on 32 unknown unknown

80
00:15:00.340 --> 00:15:07.439
Márk Tolmács: to your target list on rust app. So it downloads the standard library for for the rust

81
00:15:07.640 --> 00:15:12.850
Márk Tolmács: ecosystem. So you can compile basically

82
00:15:13.280 --> 00:15:17.845
Márk Tolmács: what you would expect for rust like string.

83
00:15:20.158 --> 00:15:24.290
Márk Tolmács: basically the the what's it called

84
00:15:25.351 --> 00:15:33.380
Márk Tolmács: the annotations macros you would expect basically compiled down to Wasm 32. So it will be ready for you.

85
00:15:33.480 --> 00:15:47.840
Márk Tolmács: Yes, there are a couple of tools. These are optional, and I'll put it in the article because we will use some of these tools just to show you what's in the inside of whatever artifacts we build.

86
00:15:48.030 --> 00:16:12.790
Márk Tolmács: you don't have to use them if you don't want to. The only exception is wasm pack. The last one in cargo install wasn't pack we're going to use wasm pack because it's a custom builder. It uses cargo in the back in the back end, but makes your life with Webassembly and rust way way easier. It wouldn't be possible to write this article without wasmpeck

87
00:16:13.434 --> 00:16:21.885
Márk Tolmács: although it would have been more educational. But I wanna get you through most of this in in 45 min. So

88
00:16:22.380 --> 00:16:42.720
Márk Tolmács: the rest. Oh, there is one more thing which is cargo generates. I just want to point it out. That's going to be we're gonna use cargo, generate to just download the template for rust the rust part of the project which I will ask to do right now.

89
00:16:42.860 --> 00:16:43.660
Márk Tolmács: So.

90
00:16:43.660 --> 00:16:45.289
Gabor Szabo: So I should have this one.

91
00:16:45.520 --> 00:16:57.650
Márk Tolmács: Yes, but you have to step one directory out, because, you see, the name wasn't on. Web is going to be a directory, so you need to overlay basically the rust template on the byte template.

92
00:16:58.110 --> 00:17:00.189
Gabor Szabo: Right? Yeah, I have to be inside. It was not.

93
00:17:00.190 --> 00:17:06.639
Márk Tolmács: No, no, never mind. Yes, yes, you're right. You're right, I forgot. Yes, so you have to be inside. Hopefully, hopefully, you have kids.

94
00:17:07.940 --> 00:17:14.240
Gabor Szabo: So here, here, this is the folder I already set up. I think I already ran the the previous command, so the one that

95
00:17:14.240 --> 00:17:19.670
Gabor Szabo: yes, appear the yarn install, and and now I just paste this one.

96
00:17:20.609 --> 00:17:21.939
Márk Tolmács: Hopefully. It will work.

97
00:17:21.940 --> 00:17:27.159
Gabor Szabo: There is some green thing there. I oh, that's just from from the zoom.

98
00:17:28.328 --> 00:17:30.619
Márk Tolmács: So if you open up.

99
00:17:30.830 --> 00:17:35.730
Gabor Szabo: This. Just let me show you this one is something from Zoom. I have no idea. What is that.

100
00:17:36.500 --> 00:17:44.268
Márk Tolmács: Me neither. So if you open up Ws code, not a Vs code. Hopefully, you will see

101
00:17:45.050 --> 00:17:50.070
Márk Tolmács: in the directory structure if you open up the the explorer.

102
00:17:50.070 --> 00:17:51.920
Márk Tolmács: Well, I I like the second.

103
00:17:52.060 --> 00:17:53.600
Gabor Szabo: What if I run? 3?

104
00:17:55.390 --> 00:17:57.122
Márk Tolmács: Well, I don't know if it's helpful.

105
00:17:57.370 --> 00:18:01.019
Gabor Szabo: Way. Too much way. Too much stuff here. Not useful. Okay.

106
00:18:01.020 --> 00:18:08.110
Márk Tolmács: Yeah, most of these are just dependencies for typescript and vite. So it's probably easier if you show it in Vs code.

107
00:18:08.220 --> 00:18:21.460
Márk Tolmács: So what happened here with the overlaying of the rust project. 1st of all, we have a lib. Dot rs, and the utils dot Rs. These are going to be our rust source codes

108
00:18:21.870 --> 00:18:28.909
Márk Tolmács: again at this point. Why not main, Dot? Rs, you would expect you would ask. Sorry you would ask.

109
00:18:29.040 --> 00:18:53.869
Márk Tolmács: due to how Webassembly works. You can only build libraries for Webassembly. You cannot build like self containing self-running applications. It has to be a library project every time. Therefore, we have the Rs. Here, and we got a goody we're going to talk about in the utils. Rs. Which is not necessary at all.

110
00:18:54.220 --> 00:19:01.330
Márk Tolmács: but it sets a panic handler which is not present by default on Webassembly, and and

111
00:19:01.410 --> 00:19:12.779
Márk Tolmács: in this setup but the whole. You will see the package soon. The the bind bind generation, the binding generation code and was on pack brings this

112
00:19:12.860 --> 00:19:35.670
Márk Tolmács: addition a nice addition which would let us actually see on the console error. So the browser console. If the rust code panics. Otherwise you wouldn't see anything. It just doesn't work. So I highly recommend setting up this panic hook. But it is done for us here. If you open up rs, you will see how exactly it happens. I think.

113
00:19:37.650 --> 00:19:56.969
Márk Tolmács: yeah. So if you see the part like that it it is defined as a MoD utils that it's just a module. It basically will be picked up automatically because of the naming because of the name of the the function in utils dot Rs, so as long as you have

114
00:19:57.080 --> 00:20:08.179
Márk Tolmács: this function present in your code, and it's not nested in anything else, it will be picked up as the panic handler automatically and will be used.

115
00:20:08.530 --> 00:20:10.510
Márk Tolmács: Okay, so

116
00:20:13.500 --> 00:20:24.090
Márk Tolmács: what else? Yes. So we have the cargo tumble file which actually contains the key tool. Can you open the cargo? Yes, thank you.

117
00:20:24.480 --> 00:20:27.830
Márk Tolmács: So contains the the key.

118
00:20:29.545 --> 00:20:45.360
Márk Tolmács: dependency, which we will need. It's called on the line on Line 14 wasn't been. This is the. This is where the magic happens. Basically, this is a bind generator. If a binding generator, if you're familiar with C, maybe rust.

119
00:20:46.930 --> 00:20:58.365
Márk Tolmács: Basically foreign function call interfaces. Usually generated with these bind generators. Wasm has a very similar

120
00:20:59.940 --> 00:21:08.780
Márk Tolmács: sort of interface which is foreign to rust. This is where the wasm binding package comes in. It automatically generates these bindings for us.

121
00:21:08.860 --> 00:21:37.640
Márk Tolmács: and later down the road we will see that there are additional packages, which are sort of like the in the rest ecosystem. They are called Sys packages. You will see that there is a Web Sys package, which contains all the goodies for all the Javascript browser built in functions as well. So you can even call Javascript functions sort of browser, based Javascript functions with ease from rust. We'll see it very soon.

122
00:21:37.820 --> 00:21:39.550
Márk Tolmács: and the rest is not really.

123
00:21:39.550 --> 00:21:49.910
Gabor Szabo: There's a there's a comment from from Gregory that you can do binaries in Wasm 32, but usually for non-standard environments like IoT or blockchains

124
00:21:50.360 --> 00:21:56.860
Gabor Szabo: and engine is specifically for a lib binding to a typescript interface that loads this lib.

125
00:21:57.640 --> 00:22:08.314
Márk Tolmács: Yes, we are. We are talking about browsers in this context, specifically, in the 1st half, I will hopefully, we will have time. I will mention that there are

126
00:22:08.830 --> 00:22:17.409
Márk Tolmács: currently options to run it outside of the browser through what's called wasi.

127
00:22:17.540 --> 00:22:22.449
Márk Tolmács: But that's not really a well developed interface.

128
00:22:22.590 --> 00:22:30.630
Márk Tolmács: So let's focus on the browsers. First, st because that's something which we can use today and get quick results.

129
00:22:31.000 --> 00:22:33.631
Márk Tolmács: But thanks for the comments. Okay,

130
00:22:34.180 --> 00:22:45.120
Márk Tolmács: what's next, I think we should get started and just hopefully build our project. 1st of all, can you try to run? Wasm back

131
00:22:45.860 --> 00:22:51.999
Márk Tolmács: in the console? Wasn't back build and dash, dash, dash, dev!

132
00:22:52.410 --> 00:22:53.860
Márk Tolmács: Can you try that, please?

133
00:22:53.860 --> 00:23:02.200
Márk Tolmács: Sorry in the call console? Yes, just wasm. Dash back. Wasm back, especk.

134
00:23:02.900 --> 00:23:07.420
Márk Tolmács: not backpack. Wasn't pack like packaging. Yes, built.

135
00:23:09.040 --> 00:23:10.210
Gabor Szabo: Build.

136
00:23:10.210 --> 00:23:13.629
Márk Tolmács: Yes, that like dash, dash, dev!

137
00:23:15.520 --> 00:23:21.020
Márk Tolmács: So it's it's cargo build. But it's wasn't back build in this case and in in dev mode. Just run it.

138
00:23:21.330 --> 00:23:29.120
Márk Tolmács: And in the explore section, hopefully, you will see that it will create for us a ready-made

139
00:23:29.270 --> 00:23:35.060
Márk Tolmács: Javascript and Bmgs ready package, which is important.

140
00:23:37.680 --> 00:23:41.680
Márk Tolmács: Okay, so can you show us the the file structure

141
00:23:47.120 --> 00:23:52.120
Márk Tolmács: it's in in a new directory. It should be in a new directory. It's called Pkg.

142
00:23:52.260 --> 00:23:54.010
Márk Tolmács: Probably you need to scroll up.

143
00:23:56.470 --> 00:23:57.040
Gabor Szabo: Okay.

144
00:23:57.040 --> 00:24:14.069
Márk Tolmács: We have a Pkg Directory. So this is a self-contained Npmgs package. You can even publish it as it is right now. It has a package, Jason with the metadata, everything you need to get get it running

145
00:24:14.070 --> 00:24:40.929
Márk Tolmács: and calling it from any other Npmgs package or Javascript or typescript packages. One other thing I would like to point out. And this is why I wanted to use typescript is that you have type definitions and you will see that we will automatically generate typescript type definitions for our rust types that we export. That's the wasm and web underscore. Bg, dot wasm, dot d dot ds.

146
00:24:40.930 --> 00:24:51.759
Márk Tolmács: that's basically a type definition for typescript. It will automatically be picked up and hopefully will be easy for you to use and figure out

147
00:24:51.820 --> 00:24:53.910
Márk Tolmács: what kind of types are exported.

148
00:24:54.680 --> 00:25:02.590
Gabor Szabo: Sorry. Wait a second. I'll try to. I just put here notes at least the comments.

149
00:25:03.070 --> 00:25:04.323
Márk Tolmács: Okay, so

150
00:25:05.550 --> 00:25:12.980
Márk Tolmács: that's great. Now, we would need to use this wasm package, because if you take a look@lib.rs.

151
00:25:16.152 --> 00:25:22.069
Márk Tolmács: hopefully, you'll see that there is. There's already a greet function exported for us.

152
00:25:22.070 --> 00:25:22.640
Gabor Szabo: Yeah. Here.

153
00:25:23.077 --> 00:25:32.709
Márk Tolmács: So we can actually get started right now. Can you open up main dot? Ts, just that's defining main dot, ts.

154
00:25:32.710 --> 00:25:34.820
Gabor Szabo: Oops. Wrong. Something.

155
00:25:35.290 --> 00:25:37.240
Gabor Szabo: Main. Ps, okay.

156
00:25:37.240 --> 00:25:51.130
Márk Tolmács: Yes, this is our entry point for the typescript part of the application. Just to verify that everything works in the console. You can run yarn space dev.

157
00:25:56.600 --> 00:26:08.900
Márk Tolmács: This will only do what it does for the typescript part. We are not yet wired rust and the typescript part together. So if you take a look at the the local host.

158
00:26:09.360 --> 00:26:12.119
Márk Tolmács: URL it post it posted here each.

159
00:26:12.120 --> 00:26:14.840
Gabor Szabo: And I think you said that I need chrome for this.

160
00:26:15.040 --> 00:26:15.790
Márk Tolmács: Yes.

161
00:26:17.070 --> 00:26:20.719
Gabor Szabo: Oh, it's so, this one, specifically, it's gonna be a.

162
00:26:23.430 --> 00:26:27.149
Márk Tolmács: Specifically, we'll need it for a later step. But Chrome is

163
00:26:27.360 --> 00:26:31.889
Márk Tolmács: fine for now, or any other browser, we will actually.

164
00:26:32.190 --> 00:26:35.009
Gabor Szabo: Well, okay, I'll just open in in chrome.

165
00:26:35.170 --> 00:26:45.170
Márk Tolmács: Perfect. I mean, you can see that it works. So let's hook in the the rust part. So if you go back to Vs codes.

166
00:26:46.450 --> 00:26:48.890
Márk Tolmács: You will see that

167
00:26:49.650 --> 00:27:05.810
Márk Tolmács: in the file structure you will see that there is a package that Json in the root folder. So that's your project, metadata. Probably not that one that's that's for the rust bosom package in the root folder. You will find another package that Json.

168
00:27:06.070 --> 00:27:08.960
Márk Tolmács: probably at the bottom, I would guess.

169
00:27:09.920 --> 00:27:11.380
Márk Tolmács: Second one. Yes.

170
00:27:12.170 --> 00:27:20.100
Márk Tolmács: So that's your project, metadata. What you need to do is you need to add this

171
00:27:20.310 --> 00:27:24.909
Márk Tolmács: wasm package to your root project.

172
00:27:25.860 --> 00:27:36.270
Márk Tolmács: In order to import it in the main. Ts, so if you quit, just control, plus C on the right, console in the in the terminal.

173
00:27:37.390 --> 00:27:40.270
Márk Tolmács: just quit the quit the divide

174
00:27:40.730 --> 00:27:48.270
Márk Tolmács: runner. Yes, if you say yarn space, add Link.

175
00:27:51.190 --> 00:27:59.619
Márk Tolmács: and you know, what can you search it in the article? It's probably easier that way, because I'm worried that we're gonna misstype.

176
00:28:02.340 --> 00:28:07.550
Gabor Szabo: That's why I write articles, because I forget I can look it up. I mean.

177
00:28:07.760 --> 00:28:13.179
Márk Tolmács: Yeah. So while you look it up, I'm going to explain why exactly. We are using this special link.

178
00:28:16.280 --> 00:28:23.419
Márk Tolmács: just search for Link, and you will. Oh, yeah, that's it. So the reason why we're using it this way is that

179
00:28:23.540 --> 00:28:45.380
Márk Tolmács: by default, yarn copies your packages, even if they are local packages into the Dependency Library or the Dependency Directory. Sorry, which is not very useful if we keep updating our codes because it will not Update the dependencies. But if you use this linking it will SIM link

180
00:28:45.490 --> 00:28:55.810
Márk Tolmács: the package directory. So every time we update our wasm it will automatically be picked up by our main project. So copy and paste this, and add it to your.

181
00:28:55.810 --> 00:29:02.429
Gabor Szabo: Second just a second before we go. Do that. I wanted to check the git status

182
00:29:02.730 --> 00:29:08.210
Gabor Szabo: and see what should what should we add to get everything. What is generated here? The node module.

183
00:29:08.210 --> 00:29:09.239
Márk Tolmács: No, not.

184
00:29:09.240 --> 00:29:09.850
Gabor Szabo: The region.

185
00:29:09.850 --> 00:29:17.179
Márk Tolmács: Node, not Node modules or the Pkg Directory, but I see that the Pkg Directory is not added.

186
00:29:17.650 --> 00:29:20.760
Gabor Szabo: Okay, so this one node modules.

187
00:29:20.950 --> 00:29:23.310
Márk Tolmács: I think that's.

188
00:29:24.610 --> 00:29:26.219
Gabor Szabo: Enough for us.

189
00:29:27.040 --> 00:29:30.470
Gabor Szabo: So test source. Read me a note.

190
00:29:30.470 --> 00:29:35.818
Gabor Szabo: I would add everything else, everything else. Look at this. Okay,

191
00:29:36.350 --> 00:29:39.710
Márk Tolmács: Yeah, it's an old template, but it works reliably. So.

192
00:29:39.710 --> 00:29:41.920
Gabor Szabo: Okay, so.

193
00:29:41.920 --> 00:29:44.044
Márk Tolmács: Just drop it if you don't want to.

194
00:29:44.310 --> 00:29:55.509
Gabor Szabo: No, it's okay. We have some history there. There's an up here also. Okay, so we I don't know. Step one. I have no idea what to put here. Comment.

195
00:29:55.510 --> 00:29:56.260
Márk Tolmács: Perfect.

196
00:29:56.260 --> 00:30:01.129
Gabor Szabo: It's it's fine. I just wanted to have some some record of of what we are doing.

197
00:30:02.030 --> 00:30:08.699
Márk Tolmács: Of course. So so I go back to the to this one, and you say, to copy, paste this command.

198
00:30:08.700 --> 00:30:09.450
Márk Tolmács: yes.

199
00:30:09.450 --> 00:30:09.870
Gabor Szabo: And right.

200
00:30:09.870 --> 00:30:12.980
Márk Tolmács: And it has to be in the in the Root Directory. Yes.

201
00:30:13.170 --> 00:30:15.319
Gabor Szabo: Oh, okay.

202
00:30:15.730 --> 00:30:19.450
Márk Tolmács: Yeah. So this is standard. It's always named Pkg.

203
00:30:19.640 --> 00:30:24.699
Márk Tolmács: it's done. And hopefully it will update the package. Jason, as well.

204
00:30:25.130 --> 00:30:33.190
Gabor Szabo: If not, they would go out here. It's sort of so yeah git diff. We can see

205
00:30:34.410 --> 00:30:40.849
Gabor Szabo: this was the change in the package added this dependency, I see.

206
00:30:41.400 --> 00:30:42.100
Márk Tolmács: Yes.

207
00:30:43.110 --> 00:30:46.280
Gabor Szabo: And so log file. It also added itself.

208
00:30:46.840 --> 00:30:47.610
Márk Tolmács: Yes.

209
00:30:48.110 --> 00:30:48.990
Gabor Szabo: Okay.

210
00:30:49.850 --> 00:30:56.310
Márk Tolmács: So now that we link together our wasm and the main typescript project

211
00:30:58.309 --> 00:31:06.939
Márk Tolmács: we can actually just call into the wasm code and see what happens. So in the main, ts.

212
00:31:07.995 --> 00:31:15.149
Márk Tolmács: file I don't know if you're familiar with Javascript at all.

213
00:31:15.150 --> 00:31:15.670
Gabor Szabo: Yeah.

214
00:31:15.670 --> 00:31:21.999
Márk Tolmács: I somewhat, yeah, okay, so I'm not gonna like, spell things out for you. Just import import.

215
00:31:22.000 --> 00:31:24.689
Gabor Szabo: Oh, that. No, that's familiar. Yeah. Okay.

216
00:31:24.690 --> 00:31:25.870
Márk Tolmács: You see the example.

217
00:31:25.870 --> 00:31:27.849
Gabor Szabo: It's import here. Yeah, okay.

218
00:31:28.440 --> 00:31:54.950
Márk Tolmács: So I'll help you, because we have the shared session if you need to, but simply just import greets like setup counter, very similar with the brackets import. Greet in the in the brackets. Because it's yeah. See? That's already actually picks it up from the type definitions. So we already imported greets, and at the bottom of the the file maybe we should just call greets.

219
00:31:55.150 --> 00:31:57.419
Márk Tolmács: I don't think it has any parameters.

220
00:31:57.560 --> 00:32:00.420
Márk Tolmács: so just call it like a normal Javascript function.

221
00:32:00.420 --> 00:32:02.120
Gabor Szabo: Just here. I'm recording.

222
00:32:03.400 --> 00:32:07.179
Márk Tolmács: I think it's just great, and it doesn't have any.

223
00:32:07.540 --> 00:32:08.150
Gabor Szabo: Hmm.

224
00:32:08.150 --> 00:32:08.700
Márk Tolmács: I see.

225
00:32:08.700 --> 00:32:10.650
Gabor Szabo: No no semicolon here. What?

226
00:32:11.070 --> 00:32:14.240
Márk Tolmács: Yeah, you you don't have to in Javascript. You can.

227
00:32:14.240 --> 00:32:14.630
Gabor Szabo: I know.

228
00:32:14.630 --> 00:32:15.270
Márk Tolmács: See who's see.

229
00:32:15.270 --> 00:32:16.980
Gabor Szabo: Afterwards. It's like, Okay.

230
00:32:17.741 --> 00:32:27.040
Márk Tolmács: I mean, whatever you prefer, it works for us. So again, that would be it. We just need to do yarn dev and the console.

231
00:32:27.180 --> 00:32:33.359
Márk Tolmács: and if we all is good, all is good definitely.

232
00:32:33.680 --> 00:32:44.619
Márk Tolmács: all is definitely not good. So this is our 1st milestone where we need to modify things. Problem is white by default doesn't know what a wasm is.

233
00:32:44.780 --> 00:33:01.270
Márk Tolmács: We need to add a couple of plugins to vite in order to pick up and properly hoist the generated wasm and Javascript glue files into our typescript project. So if you go back to the article a little bit.

234
00:33:01.390 --> 00:33:03.219
Márk Tolmács: you will find

235
00:33:05.280 --> 00:33:06.630
Gabor Szabo: This one.

236
00:33:06.890 --> 00:33:14.738
Márk Tolmács: Yes, you need to add these plugins. One of them is mine. So shameless self plug.

237
00:33:15.530 --> 00:33:21.339
Márk Tolmács: The wasn't pack watcher is a plugin I created just for this occasion.

238
00:33:21.976 --> 00:33:42.970
Márk Tolmács: Which will help you iteratively develop the rust and the typescript code together because it will watch the the rust code base along with the typescript code base. So if you make any modifications and save the rust files, it will compile reloads hopefully. It will work for us. So we added this.

239
00:33:42.970 --> 00:33:45.530
Gabor Szabo: There is. There are some some questions I see.

240
00:33:45.530 --> 00:33:46.370
Márk Tolmács: Okay.

241
00:33:47.130 --> 00:33:56.649
Gabor Szabo: Should I read it? Or, yeah, I'll read it out, and then you can respond. So it wasn't the Bing gen is nature specifically for web and node.

242
00:33:56.790 --> 00:34:02.549
Gabor Szabo: May I ask what what the wasn't plugin and top level await, which is one word.

243
00:34:02.550 --> 00:34:02.920
Márk Tolmács: Okay.

244
00:34:02.920 --> 00:34:10.010
Gabor Szabo: Are doing precisely invite some good example, and then is top level.

245
00:34:10.699 --> 00:34:11.449
Márk Tolmács: Yes. So

246
00:34:12.030 --> 00:34:33.601
Márk Tolmács: so top level wait. The the plugin being just installed, is only needed for older white projects. But I added it. Because who knows? If people want to actually integrate this workflow into their older white project? What it did is basically allowed

247
00:34:34.350 --> 00:34:47.339
Márk Tolmács: calling essing functions without any shenanigans just in the the root of a Javascript file or config file sort of

248
00:34:47.739 --> 00:35:05.409
Márk Tolmács: yeah. So basically, if you're familiar with how Javascript works, you can't just call sync functions and await them on the root context, on the global context, you have to do some hoops basically embedding them in another function.

249
00:35:05.957 --> 00:35:26.530
Márk Tolmács: Which is Async, declared Async. And then you can evade your functions inside that Async function which you can call separately. That actually is a problem for the the next plugin, which is there wasn't back no. Sorry. It's just a wasn't plugin.

250
00:35:27.520 --> 00:35:38.299
Márk Tolmács: Yes, basically. Yes, answering the the comment. The other one is the key, one white plugin wasn't. That's the key. That's the one which does the hoisting for the white parts.

251
00:35:38.430 --> 00:35:43.029
Márk Tolmács: The 3rd one, the white Plugin wasn't pack watcher. You don't need it.

252
00:35:43.150 --> 00:35:59.320
Márk Tolmács: We will use it because it will help us quickly. Iterate. The key is the 1st one white plugin bosom. That's that's what we're gonna use. And let's continue, just so we can. We don't run out of time. Can you create the white config. Dev ts, file

253
00:36:00.023 --> 00:36:04.860
Márk Tolmács: you're standing at at it right now via the contents.

254
00:36:05.480 --> 00:36:06.330
Márk Tolmács: But the finance.

255
00:36:06.330 --> 00:36:09.599
Gabor Szabo: This one wide config dev dot tl.

256
00:36:09.600 --> 00:36:09.920
Márk Tolmács: Yes.

257
00:36:09.920 --> 00:36:11.209
Gabor Szabo: Wow! That's a long name.

258
00:36:11.210 --> 00:36:11.570
Márk Tolmács: Yes.

259
00:36:11.570 --> 00:36:13.399
Gabor Szabo: And I copy this one right?

260
00:36:13.680 --> 00:36:14.260
Márk Tolmács: Yes.

261
00:36:14.260 --> 00:36:14.799
Gabor Szabo: Buy it for.

262
00:36:14.800 --> 00:36:15.399
Márk Tolmács: So we got a trick.

263
00:36:15.400 --> 00:36:16.170
Gabor Szabo: Yes.

264
00:36:16.520 --> 00:36:28.270
Márk Tolmács: Yes, we're gonna create 2 vite config files, even though usually there is only one. The reason being is that I want us to use the plugin. The watcher Plugin, I built

265
00:36:29.543 --> 00:36:31.389
Márk Tolmács: for development.

266
00:36:31.390 --> 00:36:34.789
Gabor Szabo: Sorry I have to move it out, so I can see the size of the.

267
00:36:34.950 --> 00:36:35.410
Márk Tolmács: Sure.

268
00:36:35.870 --> 00:36:38.489
Gabor Szabo: Which which folder do? Should I put it.

269
00:36:38.490 --> 00:36:41.019
Márk Tolmács: It's it must be in the roots, folder.

270
00:36:41.400 --> 00:36:42.290
Gabor Szabo: Okay.

271
00:36:44.360 --> 00:36:51.110
Gabor Szabo: I have to had to move it to my other screen because I couldn't see so. And I can't enlarge those those letters. There.

272
00:36:52.720 --> 00:36:55.850
Márk Tolmács: Okay? So yes, and then create the other flow.

273
00:36:55.850 --> 00:36:59.170
Gabor Szabo: Just a second. I put it here in the route.

274
00:36:59.580 --> 00:37:00.430
Márk Tolmács: Perfect.

275
00:37:00.430 --> 00:37:01.080
Gabor Szabo: Okay.

276
00:37:01.330 --> 00:37:09.060
Márk Tolmács: Create the other file without the dot dev part white config dot ts, that's the production build configuration.

277
00:37:09.280 --> 00:37:12.560
Gabor Szabo: White config. Dot yes, so.

278
00:37:12.560 --> 00:37:19.840
Márk Tolmács: Yes, usually, that's the that's the standard byte config file which we use for production builds.

279
00:37:20.040 --> 00:37:31.320
Márk Tolmács: It does not contain my watcher, Plugin, because if you have my watcher, Plugin, your build will never end. It's just an unfortunate artifact of things, but it doesn't matter.

280
00:37:33.110 --> 00:37:34.901
Gabor Szabo: Right? Sorry for this.

281
00:37:35.510 --> 00:37:37.530
Gabor Szabo: I had to change. Yeah.

282
00:37:38.130 --> 00:37:38.970
Gabor Szabo: Okay, excellent.

283
00:37:38.970 --> 00:37:41.540
Gabor Szabo: Have the both of them here hopefully.

284
00:37:41.940 --> 00:37:43.349
Márk Tolmács: Okay, so files.

285
00:37:45.650 --> 00:37:58.209
Márk Tolmács: Then hopefully, we can actually start working with our code just to speed things up. If you take a look at my article just immediately below what you copied

286
00:37:58.600 --> 00:38:16.500
Márk Tolmács: you will see a section called yarn at Npm. Run at all. Yes, let's run this just quickly. Let's just copy paste. I'm going to explain what's going to happen. This is just a helper, Node Node, Js helper, to help us run commands

287
00:38:16.570 --> 00:38:35.579
Márk Tolmács: in a sequential parallel fashion, and it works platform agnostically. So if you're on windows, it will work for you, too. I just wanted to make sure that I respect people who actually work on different non-unix systems. And then, if you go back to my article, you will see a section scripts.

288
00:38:35.660 --> 00:38:47.209
Márk Tolmács: Just copy the inside and put it in package. Json, I wrote it for you guys. You can just copy paste. Yes, just replace the old script section in the package. Json in the root.

289
00:38:48.950 --> 00:38:49.460
Gabor Szabo: Yes.

290
00:38:50.270 --> 00:38:50.930
Gabor Szabo: Yes.

291
00:38:50.930 --> 00:38:51.520
Gabor Szabo: Have.

292
00:38:51.840 --> 00:38:52.670
Gabor Szabo: This was.

293
00:38:52.670 --> 00:38:57.679
Márk Tolmács: And just replace that. Yeah, I created this step by step.

294
00:38:58.458 --> 00:39:13.600
Márk Tolmács: Script sections so it will work on all platforms. It's easy to follow what's going on, and it will help us make things easier. So I think I believe we are ready just to your in dev. And if we

295
00:39:13.950 --> 00:39:16.504
Márk Tolmács: actually did everything all right. Then.

296
00:39:17.360 --> 00:39:17.770
Gabor Szabo: Wow!

297
00:39:17.770 --> 00:39:34.379
Márk Tolmács: Oh, perfect. Okay. So you can see. You can see that it compiled the rust code into into wasm and then started yarn. And you already see the the alert from the rust wasm code. So this is our 1st milestone. We are called into the wasm codes on web.

298
00:39:34.680 --> 00:39:45.770
Márk Tolmács: It displayed an alert, and returned to Javascript, and the Javascript just went on running, going forward.

299
00:39:45.770 --> 00:39:46.800
Gabor Szabo: With this, please open up

300
00:39:46.800 --> 00:39:54.069
Gabor Szabo: just just one second. So I have this greet function. That is, the pop up and

301
00:39:55.230 --> 00:40:01.869
Márk Tolmács: No, that is a a function you call from the rust code. If you open the lib. Dot. Rs.

302
00:40:05.970 --> 00:40:09.970
Márk Tolmács: you will see that that is our our exported function.

303
00:40:09.970 --> 00:40:15.419
Gabor Szabo: Right? Yeah, yeah, yeah. Okay. So so the greed is is coming from rust.

304
00:40:16.420 --> 00:40:18.420
Gabor Szabo: and then it then it calls the alert.

305
00:40:18.860 --> 00:40:19.640
Márk Tolmács: Yes.

306
00:40:19.640 --> 00:40:29.780
Gabor Szabo: Right? So and okay, this whole, the website only shows after I. After all, everything is executed here right.

307
00:40:29.780 --> 00:40:39.459
Márk Tolmács: Yes, because it runs on. Yes, because it runs an alert which stops the rendering of the code. It's not because you code into rust. It's just a quirk of.

308
00:40:39.460 --> 00:40:39.870
Gabor Szabo: I do?

309
00:40:39.870 --> 00:40:41.950
Márk Tolmács: How render rendering works.

310
00:40:42.340 --> 00:40:49.380
Gabor Szabo: Right? Right? There's a so and there wasn't plugin what? Sorry. Okay, so top level, a very.

311
00:40:49.380 --> 00:40:56.110
Márk Tolmács: No, no, that that was a that was a follow up question when I explained that the wasn't Plugin is the key of all the.

312
00:40:56.110 --> 00:40:56.730
Gabor Szabo: Oh!

313
00:40:56.730 --> 00:41:09.439
Márk Tolmács: Hoisting part so hopefully. That is answered. If not, then just ask again, and we're gonna come back and and talk about it. So right now we are at the milestone, where you probably would like to have a git commit.

314
00:41:09.580 --> 00:41:11.080
Márk Tolmács: because it's working.

315
00:41:11.340 --> 00:41:15.140
Gabor Szabo: I I wait. I do way more with git commits.

316
00:41:15.650 --> 00:41:32.689
Márk Tolmács: Cool. We have everything set up for development, and the good news is, if you change anything in the rust code or the typescript code. It will reload in your browser for those of you coming from the Javascript side.

317
00:41:32.850 --> 00:41:45.570
Márk Tolmács: This is not hot reloads, and I know why it supports hot reload, but you can't do it with rust. If you modify the rust code, it has to reload the whole application. That's just the way it is. It's

318
00:41:45.570 --> 00:42:04.250
Márk Tolmács: actually, you don't have to use the watcher, Plugin, but I think it makes easier iterating on the code when you're working in this dual setup where you have wasm and typescript in the same project up to you. That's what we're going to do, because it's easier for us for this presentation. You do you? I guess

319
00:42:04.330 --> 00:42:04.940
Márk Tolmács: so.

320
00:42:04.940 --> 00:42:14.920
Gabor Szabo: By the way, right now, the the repository is just local to me. I'm going to create a something. I would put it on Github. So you will have this.

321
00:42:15.090 --> 00:42:19.550
Gabor Szabo: and we'll put the link below the video or somewhere for for the repository.

322
00:42:20.470 --> 00:42:27.900
Márk Tolmács: So let's talk about how exactly can a rust implementation of a function

323
00:42:28.160 --> 00:42:34.979
Márk Tolmács: can call a into the browser function alert if you go back to Lib. Rs.

324
00:42:35.430 --> 00:42:36.990
Márk Tolmács: Who see the

325
00:42:37.160 --> 00:42:48.130
Márk Tolmács: the glue code part, which is the external C. Block. I don't know how many of you used ffe ffi sorry with rust.

326
00:42:48.160 --> 00:43:14.600
Márk Tolmács: These extend C blocks, basically for those of you who didn't before these extent C blocks say that there are external functions which you will call in your rust code. Don't worry about it. Rust, compiler. Everything's all right. I know what I'm doing, and even in even your ide shows that it is unsafe, even though you no longer need to mark that unsafe.

327
00:43:14.630 --> 00:43:19.960
Márk Tolmács: it is unsafe. You have to guarantee that the signature of the functions you put here

328
00:43:20.040 --> 00:43:23.270
Márk Tolmács: are actually correct, otherwise it will panic.

329
00:43:23.580 --> 00:43:41.520
Márk Tolmács: So again, you see the heritage of Webassembly, the C type of function, external function definitions. It follows the C conventions for function calling. It's the exact same as you would call into a library built

330
00:43:41.630 --> 00:43:58.990
Márk Tolmács: by A. C. Linker. That's why it's external. It's an external block with C calling conventions right now. Alert is super simple. If you see the Javascript part, it's basically alert, single string parameter. It shows you the alert box.

331
00:43:59.170 --> 00:44:06.820
Márk Tolmács: same here in the extend part. It's an alert. And it actually receives a string, slice

332
00:44:06.960 --> 00:44:09.250
Márk Tolmács: a view into an own string.

333
00:44:09.480 --> 00:44:12.199
Márk Tolmács: And basically, it's just

334
00:44:12.710 --> 00:44:24.450
Márk Tolmács: there wasn't Bing and Macro added on top of it, just takes care of everything, wires it up for you, and does the calling into the Javascript processor engine. And.

335
00:44:24.630 --> 00:44:28.679
Márk Tolmács: as you can see. And you saw previously, it just works.

336
00:44:29.610 --> 00:44:46.089
Márk Tolmács: So this is actually a the easy example. I don't know how much time we have. The next section would be if we could do some more complicated type of callings like calling from.

337
00:44:47.703 --> 00:44:50.920
Gabor Szabo: Just finish a sentence. Sorry.

338
00:44:51.280 --> 00:45:02.280
Márk Tolmács: Yeah. So the next section is basically going to show us how to call our functions Javascript functions from rust

339
00:45:03.240 --> 00:45:04.530
Márk Tolmács: shall we proceed.

340
00:45:04.890 --> 00:45:15.020
Gabor Szabo: I I what I think maybe we should have like 5 min break. I stopped the the recording, for now and and then we'll put it in a probably in a separate video. What do you think.

341
00:45:15.920 --> 00:45:16.660
Márk Tolmács: Sure.

342
00:45:17.200 --> 00:45:24.569
Gabor Szabo: And do you think it's good people, I mean for you as well? I mean, you have been talking for almost an hour now.

343
00:45:25.660 --> 00:45:31.400
Márk Tolmács: I'm happy to continue the best parts. The the next part is the best part, I think.

344
00:45:31.820 --> 00:45:37.779
Gabor Szabo: I'm sure. But okay, so we we don't. We don't have to have a break. Okay, that's fine. Okay, that's good. Also.

345
00:45:37.780 --> 00:45:48.080
Márk Tolmács: All right. So let's let's let's proceed, then. So let's create a function in main Ds in a very specific way.

346
00:45:48.310 --> 00:45:52.329
Márk Tolmács: If you go back to main dot ts.

347
00:45:54.180 --> 00:45:56.909
Gabor Szabo: Just a second. I'll close all the other things.

348
00:45:57.790 --> 00:45:59.600
Márk Tolmács: Sure that people don't need.

349
00:46:00.050 --> 00:46:06.160
Márk Tolmács: Yeah, we won't need them. You can actually do a split screen if you want. With the leave dot rs and main ts, and then

350
00:46:09.178 --> 00:46:12.160
Gabor Szabo: Okay, just tell me how, because I never do that.

351
00:46:12.160 --> 00:46:20.360
Márk Tolmács: Okay. So you have to like, hold grab the that works, too. But yeah, I usually just grab the tab and

352
00:46:21.380 --> 00:46:23.030
Márk Tolmács: works too perfect.

353
00:46:23.030 --> 00:46:26.209
Gabor Szabo: How do you do that? You take it and and grab it.

354
00:46:26.210 --> 00:46:27.630
Márk Tolmács: Grab it to the side.

355
00:46:28.830 --> 00:46:33.930
Márk Tolmács: to add all the way to the side, and just to the middle side in the middle.

356
00:46:35.730 --> 00:46:36.450
Gabor Szabo: Oh!

357
00:46:37.130 --> 00:46:38.429
Márk Tolmács: Almost almost.

358
00:46:38.980 --> 00:46:41.970
Gabor Szabo: No. Didn't like it. No. Okay.

359
00:46:42.220 --> 00:46:42.630
Márk Tolmács: Hey!

360
00:46:42.913 --> 00:46:44.900
Gabor Szabo: Learning how to use the editor. Okay.

361
00:46:45.170 --> 00:46:45.440
Márk Tolmács: Cool.

362
00:46:46.640 --> 00:47:04.379
Márk Tolmács: Alright. So we can actually see both of them. And it's probably gonna be easier. So in the Maints file, if you take a look at my article, and just to scroll a little bit down, you will see that there is a part of right after the Alert just a little bit down.

363
00:47:05.710 --> 00:47:10.939
Márk Tolmács: so I can explain it to you. Yes, me a little bit more and a little bit more.

364
00:47:11.770 --> 00:47:23.853
Márk Tolmács: You see that. Yeah, this is the part. This is the part you have to put. You have to put your Javascript function into the Global Space Namespace in Javascript in order for

365
00:47:24.270 --> 00:47:44.249
Márk Tolmács: Pick it up in a in a simple way. There are ways to encapsulate your functions and then group them. We're just focusing on a simple single use case right now. So we're going to put it in the the window. Global namespace. Just copy and paste. Let's put it in the Maints.

366
00:47:46.270 --> 00:47:47.820
Márk Tolmács: Simple. Yes.

367
00:47:48.780 --> 00:47:50.109
Gabor Szabo: So I could put it at the end right.

368
00:47:50.110 --> 00:48:00.910
Márk Tolmács: Yes, you can get rid of greet. Because yes, it reloaded. That's why you see. The chrome just get rid of greet for now, because we don't need it.

369
00:48:01.950 --> 00:48:12.670
Márk Tolmács: because it's gonna keep opening the alerts, which is really annoying. So we have our Gs function. It says, Hello from Gs, let's call it from rust.

370
00:48:14.560 --> 00:48:25.030
Márk Tolmács: you just simply add the Gs function to the external C block. If you need a little bit of help, it's in the article next segment, but it just yeah.

371
00:48:28.250 --> 00:48:29.529
Gabor Szabo: Just Fn.

372
00:48:30.807 --> 00:48:44.089
Márk Tolmács: Capital F function. Yep, and it already loaded, and it recognized that that is a function.

373
00:48:44.090 --> 00:48:51.370
Gabor Szabo: I wonder if it's if it's visual studio code doing it, or the co-pilot that I have that it just okay.

374
00:48:52.950 --> 00:48:53.680
Márk Tolmács: Wasn't.

375
00:48:53.680 --> 00:48:56.300
Gabor Szabo: Already read your article, and now it's copying from there.

376
00:48:56.300 --> 00:49:18.189
Márk Tolmács: Probably. Probably. So it already set up type check actually works. You can see that it already sees that there is a Javascript function present which you try to call into, and then all you need to do is just comment out the alert in in greet, and just call Gs function.

377
00:49:23.290 --> 00:49:24.160
Márk Tolmács: Yes.

378
00:49:27.520 --> 00:49:28.120
Márk Tolmács: Hmm.

379
00:49:28.450 --> 00:49:29.300
Gabor Szabo: Okay.

380
00:49:29.680 --> 00:49:37.249
Márk Tolmács: So all you need to do just call the grid function after your function definition in the main Ds file.

381
00:49:37.460 --> 00:49:41.520
Márk Tolmács: Just go greet you. You're gonna call into.

382
00:49:41.520 --> 00:49:42.040
Gabor Szabo: New York.

383
00:49:42.040 --> 00:49:48.379
Márk Tolmács: You're gonna call into rust. Yes, great, because we already imported it.

384
00:49:49.460 --> 00:49:50.649
Márk Tolmács: Yep, that's it.

385
00:49:51.440 --> 00:50:04.069
Márk Tolmács: Hello! From Gs, so what happened here is that we called the greed function in Webassembly, which called into Javascript with the Gs. Function which called alert, showing the little message.

386
00:50:04.070 --> 00:50:04.769
Gabor Szabo: Okay. I didn't.

387
00:50:04.770 --> 00:50:05.170
Márk Tolmács: Set up.

388
00:50:05.170 --> 00:50:05.820
Gabor Szabo: Okay.

389
00:50:06.020 --> 00:50:16.980
Márk Tolmács: So now we can see how you can define functions in Javascript. Call them from rust, how you can define functions in rust, call them from Javascript.

390
00:50:17.390 --> 00:50:17.840
Gabor Szabo: Hmm.

391
00:50:17.840 --> 00:50:24.020
Márk Tolmács: Could be another. Get. Commit moment if if you want to right the history

392
00:50:24.020 --> 00:50:29.630
Márk Tolmács: right? I'm trying still trying to understand what what just happened. But yeah, think about it then.

393
00:50:33.430 --> 00:50:44.321
Márk Tolmács: So basically, what happens here is wasn't been magic. This is the binding generator for you, working in action or doing the all the action here.

394
00:50:46.830 --> 00:51:13.170
Márk Tolmács: there are a lot of nuanced things going on how you map things, especially things like strings between Javascript and Webassembly. It is all papered and covered for you by Wasm Bingen. And these wasn't Bingen Macros, you see, added to the external block the function, and probably not today, but you can actually add it to practically anything in rust.

395
00:51:13.330 --> 00:51:32.990
Márk Tolmács: and it will most likely have a good default and expose it to Javascript for you. So it's quite flexible. As somebody mentioned. It's quite mature, Gregory. Thank you. It's quite mature. I used it in production projects, and it works really great, as you can see

396
00:51:33.440 --> 00:51:38.140
Márk Tolmács: relatively easy and fast to develop develop with.

397
00:51:38.800 --> 00:51:49.600
Márk Tolmács: Okay? Any questions. Let's just have a moment and see if anybody has any questions, because this is the very basics of how you can integrate these 2 technologies.

398
00:51:49.600 --> 00:51:49.920
Gabor Szabo: 30.

399
00:51:49.920 --> 00:51:50.790
Márk Tolmács: Together.

400
00:51:50.790 --> 00:51:57.239
Gabor Szabo: Let me try to go over what I there is actually something I see or or did you read this already? Maybe it's coming to.

401
00:51:57.240 --> 00:51:57.940
Márk Tolmács: Sorry.

402
00:51:58.120 --> 00:52:00.039
Gabor Szabo: I think this is the future of web.

403
00:52:00.780 --> 00:52:08.019
Gabor Szabo: I should. I have built an SDK on rust wasn't 32 with wasn't by being Gen. For blockchain.

404
00:52:08.470 --> 00:52:11.259
Gabor Szabo: I'm currently writing a white widget loader

405
00:52:11.480 --> 00:52:23.520
Gabor Szabo: loaded in angular loading wasn't exactly like your example. If you want to see it live, let me know. I share my screen, and maybe later, okay.

406
00:52:24.694 --> 00:52:25.309
Gabor Szabo: yes.

407
00:52:25.310 --> 00:52:25.930
Márk Tolmács: Sure.

408
00:52:26.540 --> 00:52:29.090
Gabor Szabo: I'll try to go over what? What we just wrote.

409
00:52:29.240 --> 00:52:32.660
Gabor Szabo: Okay, so because I am, I'm not.

410
00:52:32.950 --> 00:52:43.869
Gabor Szabo: So this, basically, this is what you you. This is how you define a function called Gs function and put it in the in the global namespace right of of the window on the window.

411
00:52:45.900 --> 00:52:49.989
Gabor Szabo: Why do you need this whole thing? Why not just call it Js function.

412
00:52:50.710 --> 00:52:51.210
Márk Tolmács: You know.

413
00:52:53.240 --> 00:53:10.290
Márk Tolmács: Yeah. So you don't have to do this, you can actually package it up. Later we will see that it it can be packaged. You can even call into class instances. Whatever basically, whatever you can imagine can be done wasn't been, can take care of it

414
00:53:10.800 --> 00:53:25.610
Márk Tolmács: right now, this example tries to be tries to be simple. It's an easy, easy way to show you how the basic global namespace function can be called from rust.

415
00:53:25.930 --> 00:53:27.770
Márk Tolmács: Unfortunately, how.

416
00:53:28.410 --> 00:53:29.189
Gabor Szabo: Go ahead.

417
00:53:30.000 --> 00:53:36.190
Gabor Szabo: No, I'm trying to. Still. So here, this is how I define the function, this one is needed because.

418
00:53:37.060 --> 00:53:37.790
Gabor Szabo: by the way.

419
00:53:37.790 --> 00:54:02.649
Márk Tolmács: Yeah. So yeah, yes, that's that's what I tried to. Yeah, that that's where I was at the moment. Yeah. So the reason why you have to put it explicitly in window. Is that how the Esm like the what's it called? So basically how the packager packages your code into Javascript modules. They are never global namespace.

420
00:54:03.020 --> 00:54:25.080
Márk Tolmács: It's just a quirk of how the Esm module bundling and module handling in browsers and javascript engines work because we are using typescript, and it is configured the way it is configured. Whatever function you define here in this, in this file, it will not be a global function, you will not be able to access it

421
00:54:25.150 --> 00:54:44.360
Márk Tolmács: from your browser. Console. You can try it. You can define a function here, and you will never be able to call it from your console unless you know what you're doing. But let's not go there. It's not not global. So what I do here is that I will say explicitly that it has to go on the window.

422
00:54:44.460 --> 00:54:46.179
Márk Tolmács: Global object.

423
00:54:46.340 --> 00:54:51.540
Márk Tolmács: That's the only way rust can access this access. This

424
00:54:51.960 --> 00:54:59.200
Márk Tolmács: function reference. So you can call into otherwise it doesn't know how to call your function. Does that make sense.

425
00:55:00.000 --> 00:55:02.129
Gabor Szabo: Right? Yeah, I think so.

426
00:55:03.150 --> 00:55:12.670
Márk Tolmács: So it's just a nasty. It's just a nasty but simple example. How simple function calls, can work between the 2 platforms.

427
00:55:14.690 --> 00:55:16.780
Márk Tolmács: Do you had other other questions regarding.

428
00:55:16.780 --> 00:55:21.799
Gabor Szabo: No, I think it's okay. So so in the other side, I just have to put in the same signature that we have here.

429
00:55:21.800 --> 00:55:22.170
Márk Tolmács: Yes.

430
00:55:22.170 --> 00:55:27.810
Gabor Szabo: And then, and then I can call it from here. Now I have to pass.

431
00:55:28.270 --> 00:55:31.509
Gabor Szabo: and earlier we passed in a string.

432
00:55:31.890 --> 00:55:34.049
Gabor Szabo: so we know how to do that already.

433
00:55:34.710 --> 00:55:42.080
Márk Tolmács: Yeah, you can actually add a string to greet

434
00:55:42.210 --> 00:55:52.040
Márk Tolmács: in rust if you want to string slice specifically. But string will work as well an own string and just provide a provide

435
00:55:52.370 --> 00:55:57.410
Márk Tolmács: the parameter in typescript, and it will get it in rust. And you can print that out.

436
00:55:57.600 --> 00:55:59.240
Márk Tolmács: Why don't we do it right now.

437
00:55:59.930 --> 00:56:01.800
Márk Tolmács: just simply add add the string size.

438
00:56:01.800 --> 00:56:08.759
Gabor Szabo: Okay. So so we we would like, now, the the greed function will. Basically, I would like to get here.

439
00:56:09.440 --> 00:56:11.000
Gabor Szabo: U bar right.

440
00:56:11.000 --> 00:56:11.930
Márk Tolmács: Sure.

441
00:56:12.330 --> 00:56:13.240
Márk Tolmács: Sure.

442
00:56:13.600 --> 00:56:18.110
Márk Tolmács: Yeah, it it keeps alerting. But let's disable it.

443
00:56:18.510 --> 00:56:19.690
Márk Tolmács: Yeah, okay.

444
00:56:19.690 --> 00:56:20.930
Gabor Szabo: For now it's all.

445
00:56:21.070 --> 00:56:22.970
Márk Tolmács: Yeah, so gonna change that.

446
00:56:23.640 --> 00:56:25.920
Gabor Szabo: For that I need to.

447
00:56:29.240 --> 00:56:34.800
Gabor Szabo: So the greet is is a is just a function here.

448
00:56:35.260 --> 00:56:37.000
Gabor Szabo: So I invest.

449
00:56:37.000 --> 00:56:41.979
Gabor Szabo: I don't need to. I don't think I mean I import just a name here, so I don't need to put anything else.

450
00:56:41.980 --> 00:56:42.580
Márk Tolmács: Yes, yes.

451
00:56:42.580 --> 00:56:48.366
Gabor Szabo: And here I need to tell you that it accepts. It's it's it's a

452
00:56:50.370 --> 00:56:51.825
Gabor Szabo: It's just a

453
00:56:52.310 --> 00:56:53.090
Márk Tolmács: String, slice.

454
00:56:53.090 --> 00:57:04.820
Gabor Szabo: Ross view right? And then and then this would let's say, call alert with the S.

455
00:57:05.940 --> 00:57:06.890
Márk Tolmács: Why not?

456
00:57:07.550 --> 00:57:10.949
Gabor Szabo: Right. And now, if I enable this.

457
00:57:11.160 --> 00:57:16.229
Gabor Szabo: then it will say, and we don't see Bell, but it says Fubar, right.

458
00:57:16.480 --> 00:57:19.370
Márk Tolmács: Sure. Yeah, that's that easy.

459
00:57:21.210 --> 00:57:28.899
Gabor Szabo: Nice, so you can write if anything like a mess or something, and any stuff, and.

460
00:57:28.900 --> 00:57:41.999
Márk Tolmács: Yeah, but you have to. You have to limit yourself in this regard. You have to limit yourself to simple Javascript types, with some exceptions, but we will see how we can add complex.

461
00:57:42.000 --> 00:57:59.030
Márk Tolmács: how we can pass complex parameters between Rust and Javascript in a moment. But before we do that I don't know about you guys, but I'm quite annoyed by these alert pop ups. Why don't we use

462
00:57:59.270 --> 00:58:03.959
Márk Tolmács: a basically console log to log into the console from rust.

463
00:58:04.150 --> 00:58:08.596
Márk Tolmács: So if you go back to my little article,

464
00:58:09.720 --> 00:58:16.830
Márk Tolmács: you will see a section called using existing Javascript Apis, and functions should go a little bit further down.

465
00:58:18.770 --> 00:58:29.920
Márk Tolmács: Yes, this is it. So let's add this dependency, and we need to add it manually, because I just want I mentioned this before. This is the Web Sys package I just added to cargo tomorrow

466
00:58:30.090 --> 00:58:33.299
Márk Tolmács: the reason why we doing it.

467
00:58:33.530 --> 00:58:38.849
Márk Tolmács: I mean, we could add it with cargo, add and specify the features. It's just.

468
00:58:38.850 --> 00:58:41.240
Gabor Szabo: It will be added. Oh, just as it is.

469
00:58:41.540 --> 00:58:52.010
Márk Tolmács: It's just a dependency in cargo, Tomo. The reason why it's it's like this is to show you guys that it actually adds a specific feature called window.

470
00:58:52.840 --> 00:59:06.870
Márk Tolmács: which we will need, and the other one which we will need, which which is not presenting the article. And that's problem. Hold on. You need to add console, I believe, to the feature list, because window is not enough.

471
00:59:09.210 --> 00:59:12.470
Gabor Szabo: Like, How do you do this? Yeah, I think so.

472
00:59:12.470 --> 00:59:21.980
Márk Tolmács: Yes, I believe, but it may be. Hold on, let me check. I think it has to be capital letter. Console.

473
00:59:22.520 --> 00:59:25.880
Márk Tolmács: No, no, you're right? No, no, all all the

474
00:59:26.080 --> 00:59:30.189
Márk Tolmács: oh, good, just, simple. Console, not capital letter console.

475
00:59:30.600 --> 00:59:33.569
Márk Tolmács: Okay. So once we have this

476
00:59:34.570 --> 00:59:41.710
Márk Tolmács: and we install install the packages hopefully, or it will automatically get downloaded. Once we

477
00:59:42.453 --> 00:59:44.920
Márk Tolmács: build the rust code again.

478
00:59:45.210 --> 00:59:48.049
Márk Tolmács: we will have access to the web.

479
00:59:48.050 --> 00:59:52.020
Gabor Szabo: Basically, if I just just make some changes, it will do it right.

480
00:59:52.160 --> 00:59:59.439
Márk Tolmács: Hopefully. Yes, it should. You will see in the console the cargo cargo build command.

481
00:59:59.570 --> 01:00:06.770
Márk Tolmács: But why? Probably sorry, Terminal, I meant terminal sorry, Terminal.

482
01:00:07.300 --> 01:00:09.829
Gabor Szabo: In in terminal, in this one.

483
01:00:09.830 --> 01:00:17.269
Márk Tolmács: Vs code. Yes, and if you built it you will see it just white actually scrolled it out in your terminal.

484
01:00:18.870 --> 01:00:23.599
Márk Tolmács: Whether or not it, added the package. That's what basically was. What what I'm looking for.

485
01:00:24.440 --> 01:00:26.110
Gabor Szabo: A little bit more.

486
01:00:37.520 --> 01:00:39.360
Márk Tolmács: Oh, I don't know.

487
01:00:41.920 --> 01:00:48.550
Márk Tolmács: Anyway, we will see it very soon. If it doesn't work. Bs code and rest. Analyzer will let us know.

488
01:00:48.730 --> 01:00:54.180
Márk Tolmács: So then then we will fix it, I promise.

489
01:00:54.740 --> 01:00:57.469
Márk Tolmács: So what what happens now is that.

490
01:00:57.470 --> 01:00:58.420
Gabor Szabo: That's really.

491
01:00:59.689 --> 01:01:10.789
Márk Tolmács: It will work out, I promise. So if you go back to my article, you'll see that. How exactly you will be able to call web systems.

492
01:01:11.130 --> 01:01:12.789
Márk Tolmács: So it's basically console log.

493
01:01:13.170 --> 01:01:19.479
Márk Tolmács: No, no, you don't have to copy it. It's just the one line web Sys console. It's the web, Sys package it. It contains 9.

494
01:01:20.700 --> 01:01:21.649
Gabor Szabo: In the FM.

495
01:01:22.670 --> 01:01:30.540
Márk Tolmács: Yes, yeah, basically, the only thing, the only thing we need. And let's just quickly review it.

496
01:01:30.830 --> 01:01:35.410
Márk Tolmács: Only thing that needs to be probably imported

497
01:01:38.720 --> 01:01:41.699
Márk Tolmács: if the Web Sys package is available.

498
01:01:41.910 --> 01:01:46.279
Márk Tolmács: Yes, now it's it is compiling websis. So

499
01:01:48.080 --> 01:01:54.030
Márk Tolmács: yay, okay, we have it. So it's easy to failed.

500
01:01:55.550 --> 01:01:59.890
Márk Tolmács: Yeah, because we have an error. It's not.

501
01:01:59.890 --> 01:02:01.860
Gabor Szabo: Person and not s, right? Okay.

502
01:02:01.860 --> 01:02:06.220
Márk Tolmács: Yes, yes, yes, there you go.

503
01:02:07.630 --> 01:02:18.490
Márk Tolmács: It worked perfect. So it's super easy to call console log in Javascript. And you just dump multiple parameters.

504
01:02:21.010 --> 01:02:27.700
Márk Tolmács: multiple parameters into the the calling signature. That's not so easy with compiled languages like rust.

505
01:02:27.880 --> 01:02:46.190
Márk Tolmács: Not compiled. Sorry. So low, level, lower level languages like rust. So basically, what it does is it has log, one log, 2, representing how many parameters you would like to add to the console log. So we use log one, because we always log a single parameter, which is a string.

506
01:02:46.460 --> 01:02:58.930
Márk Tolmács: You can actually have called console log, which receives an array of parameters, and it logs that just pointing it out that there are variations and that

507
01:03:00.127 --> 01:03:07.079
Márk Tolmács: complicated, it's easy to figure out. But right now we're gonna use this particular single parameter, console or call.

508
01:03:07.230 --> 01:03:21.670
Márk Tolmács: as you can see. If we rebuild and open chrome, you will not see the alert anymore, which is super annoying, but you will see it in your con web. Console the message we sent.

509
01:03:21.820 --> 01:03:25.040
Gabor Szabo: So let's add buzz as well, and then.

510
01:03:25.040 --> 01:03:25.530
Márk Tolmács: What?

511
01:03:25.530 --> 01:03:30.360
Gabor Szabo: You should supposed to here. Yeah.

512
01:03:31.950 --> 01:03:40.289
Márk Tolmács: All right. You can also use console error, console, warn, just like you would from Javascript the same way.

513
01:03:41.470 --> 01:03:44.932
Márk Tolmács: Okay? So why don't we pass

514
01:03:45.720 --> 01:03:50.659
Márk Tolmács: more complex parameters around if you're ready, and if you don't have any questions.

515
01:03:51.400 --> 01:03:52.949
Gabor Szabo: Yeah, I just go.

516
01:04:23.720 --> 01:04:24.550
Gabor Szabo: okay.

517
01:04:28.700 --> 01:04:39.069
Márk Tolmács: So going forward, I would recommend copy pasting from the article just because it's a lot of typing otherwise. And we have been sitting here until the morning.

518
01:04:39.070 --> 01:04:41.559
Gabor Szabo: Notice the the discussion in the chat. By the way.

519
01:04:42.188 --> 01:04:44.280
Márk Tolmács: No, I'm checking it right now.

520
01:04:50.480 --> 01:05:09.799
Márk Tolmács: Yes, Gregory, it's correct. You can use, by the way, awesome coding web workers. But by default it runs on the main thread. But you can use web workers with awesome today. We're not going to cover it. It's not that trivial, but it works great. So.

521
01:05:14.770 --> 01:05:25.980
Márk Tolmács: yes, that's this is this is the type of scoping with the Gs namespace parameter to the was on Bingan Macro. What I was talking about. That's how you can group stuff

522
01:05:26.090 --> 01:05:37.030
Márk Tolmács: and still access it from rust easily, just like with the console namespace which you can think of as an object in Javascript. And

523
01:05:37.280 --> 01:05:43.129
Márk Tolmács: yeah, basically, you can prefix it and use it simpler, just like Gregory shows.

524
01:05:44.070 --> 01:05:53.250
Márk Tolmács: Okay, so why don't we pass a class reference class instance to rust.

525
01:05:53.360 --> 01:05:59.339
Márk Tolmács: I wrote a little class in typescript. The Tsdef just named it Dsf.

526
01:05:59.870 --> 01:06:03.029
Márk Tolmács: just put it in the main dot Ds file.

527
01:06:07.220 --> 01:06:08.030
Márk Tolmács: Yeah.

528
01:06:08.150 --> 01:06:16.601
Márk Tolmács: So that's a simple class as a constructor with a string parameter and string attribute rather

529
01:06:17.570 --> 01:06:21.570
Márk Tolmács: and it has a message. Does a console log prints the id

530
01:06:22.240 --> 01:06:27.330
Márk Tolmács: absolutely the simplest thing you can do. Why don't we?

531
01:06:29.470 --> 01:06:34.659
Márk Tolmács: basically map it to the rust code and see how exactly

532
01:06:35.280 --> 01:06:46.040
Márk Tolmács: it works. Just copy and paste this section into the rust code. So we can talk about what's going on, because it's a more complicated

533
01:06:46.330 --> 01:06:48.519
Márk Tolmács: set up than the previous ones.

534
01:06:49.050 --> 01:07:02.720
Márk Tolmács: So we still have a function. Remote instance, param. I named it remote instance param the same way as greet, but it receives something else. If you scroll a little bit down in lib. Dot, rs.

535
01:07:04.770 --> 01:07:30.589
Márk Tolmács: so it receives the pointer to a Tsdef struct. So Javascript classes become mapped to structs in rust, and then you can call the functions access the parameters just like you would in Javascript. How exactly that mapping works. If you scroll a little bit up, we will see how a struct is mapped. Basically.

536
01:07:31.043 --> 01:07:35.370
Márk Tolmács: sorry. A class is mapped basically to a struct in rust

537
01:07:35.890 --> 01:07:38.870
Márk Tolmács: a little bit further up, please.

538
01:07:41.300 --> 01:07:51.510
Márk Tolmács: it's still an external block. Again, there are more idiomatic ways to do this. This is a simple example to show us how exactly it works. It's an external block

539
01:07:51.670 --> 01:08:02.550
Márk Tolmács: with a bind wasn't bind. And macro, you have to define a type which actually name matches with the Javascript side or typescript site.

540
01:08:02.770 --> 01:08:22.219
Márk Tolmács: And then you have to define basically all the functions which are like. You can see the example at the bottom. The run methods, not a function. Sorry, a method on your class in Javascript, which receives that this parameter? It's not the same as self.

541
01:08:22.350 --> 01:08:27.779
Márk Tolmács: because there is no South. This is not a sort of traditional struct in this sense.

542
01:08:30.270 --> 01:08:41.250
Márk Tolmács: and then you can see some magic with how you can access the public attribute on the class instance which is called id in our case.

543
01:08:42.272 --> 01:09:08.889
Márk Tolmács: The get what it. What it does basically is that it takes advantage of the Javascript object, prototype method getters and setters. And it does that by defining this specific with this specific nomenclature. So the the attribute is called id on the Javascript side. So we call it id in this externity definition.

544
01:09:09.523 --> 01:09:28.579
Márk Tolmács: Basically, mapping it with the wasn't Bindgen. Well, bind Gen. Sorry. Macro, you have to provide these parameters. So it knows it actually needs to generate the getter mapping for you, and almost similar. The setter part, which is called the

545
01:09:28.580 --> 01:09:43.219
Márk Tolmács: object prototype chain setter for the parameter. Again, naming is important, and you add the wasn't Bindgen Macro with the setter attribute to create the mapping. Yes, again.

546
01:09:43.229 --> 01:09:46.539
Gabor Szabo: This is how we define the getter to be id.

547
01:09:46.540 --> 01:09:47.290
Márk Tolmács: Yes.

548
01:09:47.290 --> 01:09:50.710
Gabor Szabo: And etc. But you say that the name is also important, that.

549
01:09:50.710 --> 01:09:57.460
Márk Tolmács: It is really important, unless you parameterize the bosom bind Gen. Macro to use.

550
01:09:57.570 --> 01:10:15.720
Márk Tolmács: you know, a menu on mapping. So it's just to not confuse everybody. I chose to follow the naming convention, and just a simple wasn't Bindgen Macro core. But you can configure pretty much everything with was on bind. Gen, you can add your own mapping. Js.

551
01:10:15.720 --> 01:10:18.059
Márk Tolmács: name. There's something naming right?

552
01:10:18.060 --> 01:10:23.630
Márk Tolmács: Js, name, it's always Gs underscore something with wasn't by engine.

553
01:10:24.240 --> 01:10:25.930
Márk Tolmács: Yeah. So you can add it.

554
01:10:26.140 --> 01:10:30.740
Márk Tolmács: you can add your own let's just focus on the simple example.

555
01:10:30.740 --> 01:10:32.200
Gabor Szabo: Yeah, well, and

556
01:10:33.160 --> 01:10:58.039
Márk Tolmács: People as an exercise can try to do these additional configurations. And what happens here is that the function which we're going to call from Javascript. The remote instance program receives that type definition which is auto-generated by Wasm Benjam for us. And look, it just simply calls the Id function just like you would in Javascript.

557
01:10:58.040 --> 01:10:58.450
Gabor Szabo: Here.

558
01:10:58.450 --> 01:11:03.930
Márk Tolmács: And awesome bindge, and takes care of everything for you. So

559
01:11:04.500 --> 01:11:11.910
Márk Tolmács: why don't we try to call this remote instance param on the Javascript side and see what's gonna happen.

560
01:11:22.080 --> 01:11:22.890
Gabor Szabo: I don't know.

561
01:11:23.060 --> 01:11:24.110
Gabor Szabo: I tried to copy.

562
01:11:25.100 --> 01:11:27.150
Márk Tolmács: Yeah. Copy. Paste it. Fine.

563
01:11:27.150 --> 01:11:29.729
Gabor Szabo: And that's what I tried, but it didn't like me.

564
01:11:29.730 --> 01:11:32.209
Márk Tolmács: You already have it. Just enter. Yep.

565
01:11:32.640 --> 01:11:33.340
Gabor Szabo: Okay.

566
01:11:33.540 --> 01:11:34.600
Márk Tolmács: Why not?

567
01:11:35.020 --> 01:11:40.490
Márk Tolmács: There you go, and then yes, and then just press, okay.

568
01:11:41.520 --> 01:11:44.519
Gabor Szabo: Yeah. So it's already the previous one.

569
01:11:44.520 --> 01:11:54.039
Márk Tolmács: Yeah, yeah, it's probably not use useful. But the alert is alert is in the new function. So that's why you're getting these.

570
01:11:54.040 --> 01:11:56.329
Gabor Szabo: I can comment out also that alert.

571
01:11:57.290 --> 01:12:01.109
Márk Tolmács: You can actually use the console. Now that you know how to use it.

572
01:12:01.700 --> 01:12:07.250
Márk Tolmács: it's easy to modify it. Just copy and paste. If you want to avoid the alert.

573
01:12:07.480 --> 01:12:11.429
Gabor Szabo: Wait a second. I can copy this. Console right?

574
01:12:11.430 --> 01:12:14.569
Márk Tolmács: Yeah, probably easier to follow than with the alerts.

575
01:12:16.450 --> 01:12:17.679
Gabor Szabo: So I can.

576
01:12:23.960 --> 01:12:27.410
Gabor Szabo: They don't need the stringification, you know.

577
01:12:37.890 --> 01:12:45.680
Gabor Szabo: right. And now it says here that the ZYXW.

578
01:12:45.680 --> 01:12:53.370
Márk Tolmács: Because it was overridden in in rust. So it sets, sets the id Zyx pam.

579
01:12:53.630 --> 01:13:01.029
Márk Tolmács: CYXW. Question is, why don't we call into the console login rust.

580
01:13:01.550 --> 01:13:05.490
Márk Tolmács: Maybe you need to reload. Maybe your console.

581
01:13:05.490 --> 01:13:05.939
Gabor Szabo: No, no, no.

582
01:13:05.940 --> 01:13:06.670
Márk Tolmács: I don't know.

583
01:13:06.670 --> 01:13:09.340
Gabor Szabo: Is. It isn't the thing that that this one

584
01:13:09.940 --> 01:13:13.790
Gabor Szabo: that's a pointer. And and then the browser replaces it.

585
01:13:13.790 --> 01:13:22.450
Márk Tolmács: Probably could be. It works for me. Maybe it overrides it sooner than sooner than it actually displays.

586
01:13:28.770 --> 01:13:30.740
Gabor Szabo: So, yeah, you see. Now.

587
01:13:30.740 --> 01:13:31.070
Márk Tolmács: Yes.

588
01:13:31.070 --> 01:13:36.760
Gabor Szabo: Id. 1, 2, 3, and when I when I let the alarm go through, then it replaces it.

589
01:13:38.360 --> 01:13:39.140
Márk Tolmács: Yep.

590
01:13:39.140 --> 01:13:44.299
Gabor Szabo: This is some, some I mean, I yeah, because it's a pointer. There.

591
01:13:44.300 --> 01:13:51.640
Márk Tolmács: Can you? Can you scroll up in your console? Maybe you don't need the alert. We just haven't seen the the message, because, see.

592
01:13:51.640 --> 01:13:53.300
Gabor Szabo: Do you think so?

593
01:13:53.680 --> 01:13:56.810
Márk Tolmács: Maybe you maybe you don't need the alert to wait for

594
01:13:57.306 --> 01:14:05.010
Márk Tolmács: the process because I don't think you would need to, because it runs on the main thread. There are no parallelization.

595
01:14:05.590 --> 01:14:06.670
Gabor Szabo: I clean this up.

596
01:14:06.670 --> 01:14:07.230
Márk Tolmács: But.

597
01:14:07.490 --> 01:14:09.109
Gabor Szabo: I remove the alert.

598
01:14:10.390 --> 01:14:11.679
Gabor Szabo: And yeah, we were right.

599
01:14:11.680 --> 01:14:15.270
Márk Tolmács: There you go. I would have been surprised, honestly.

600
01:14:15.410 --> 01:14:16.510
Márk Tolmács: But okay.

601
01:14:16.620 --> 01:14:17.180
Gabor Szabo: But I.

602
01:14:17.180 --> 01:14:22.040
Gabor Szabo: So this, maybe it's it's these objects that it was replacing the data inside.

603
01:14:22.620 --> 01:14:34.350
Márk Tolmács: Cool. Yeah. Usually a reasonable guess. In this case. This runs in a, you know, on a single thread, I would say a main thread. So it should be sequential.

604
01:14:34.700 --> 01:14:35.710
Márk Tolmács: Okay?

605
01:14:35.840 --> 01:14:36.880
Márk Tolmács: So

606
01:14:37.200 --> 01:14:50.900
Márk Tolmács: again, let's get back to our Vs code and let's stop for a moment. Do you have any questions? Do. Anybody has any questions what's going on? It's a little bit more complicated of an example. Now.

607
01:14:51.120 --> 01:14:54.410
Gabor Szabo: Anyone people questions okay, people.

608
01:14:54.520 --> 01:14:55.700
Márk Tolmács: Ask questions.

609
01:14:59.300 --> 01:15:03.590
Márk Tolmács: Yeah, you can. You can see how basically passing

610
01:15:03.830 --> 01:15:10.590
Márk Tolmács: complex types and complex data via pointers and mapping them on in rust

611
01:15:10.930 --> 01:15:20.430
Márk Tolmács: would work. It works the same way. When you do want to share a rust structure with Javascript.

612
01:15:20.670 --> 01:15:26.859
Márk Tolmács: I believe in the article you'll find another example little down the road.

613
01:15:27.100 --> 01:15:31.980
Márk Tolmács: a little bit down the road. I actually create a personal script.

614
01:15:32.540 --> 01:15:57.240
Márk Tolmács: I don't know if you want to spend time on doing this because there is a little bit of a gem I wanted to share with you, and it's also in the article, so maybe we don't have time to do this, but you can check it out. It's very similar. It defines struct it implements the binding. We are both on. Bindgen just sets the markers, and then, if you scroll a little bit further down, you will see that

615
01:15:57.590 --> 01:16:03.809
Márk Tolmács: you can just simply import the person struct as a class from your wasm package.

616
01:16:04.220 --> 01:16:08.510
Márk Tolmács: and you just instantiate it, and you can make changes.

617
01:16:11.920 --> 01:16:12.380
Márk Tolmács: Yes.

618
01:16:12.380 --> 01:16:19.990
Gabor Szabo: In in, in rust, and you would like to provide it to to be accessible from the Javascript.

619
01:16:20.270 --> 01:16:20.910
Márk Tolmács: Yes.

620
01:16:21.260 --> 01:16:21.720
Gabor Szabo: Nice.

621
01:16:22.020 --> 01:16:26.490
Márk Tolmács: One thing I would like to point out, since we talked about pointers.

622
01:16:26.650 --> 01:16:53.690
Márk Tolmács: You don't get the luxury of just leaving your instantiated structs from Webassembly around in Javascript. You have to explicitly call free on them, which is a method added by wasn't been because rust doesn't have. Oh, sorry Webassembly doesn't have garbage collection. You have to be responsible for cleaning it up. There is no drop semantics, either, because

623
01:16:53.810 --> 01:17:04.694
Márk Tolmács: it's hold. The the reference is held by the Javascript engine. So rest never knows when exactly it is released. This is how you get

624
01:17:05.330 --> 01:17:23.430
Márk Tolmács: Well, this is how you let the rust code know that you no longer need it. In Javascript it can be freed, and then drop is called. Everything's taken care of. So that's the only quirk you need to be aware of when you actually instantiate structs and store data

625
01:17:23.620 --> 01:17:40.699
Márk Tolmács: in rust memory, or was a memory and use it in Javascript. So this is the gem. Finally, I wanted to show you guys because you can actually do debugging just like you would with C or rust in chrome.

626
01:17:40.890 --> 01:17:47.740
Márk Tolmács: This is a chrome only feature you have to install this extension. I link here

627
01:17:47.880 --> 01:17:53.399
Márk Tolmács: because of how? Yes, because.

628
01:17:53.400 --> 01:17:56.509
Gabor Szabo: You already sent me right, and then I installed it. Yeah.

629
01:17:56.510 --> 01:17:58.410
Márk Tolmács: Hopefully you already installed it. Yes.

630
01:17:58.780 --> 01:18:01.090
Márk Tolmács: so let's just quickly set it.

631
01:18:01.730 --> 01:18:03.430
Márk Tolmács: Yes, yes, let's do it.

632
01:18:04.570 --> 01:18:05.870
Gabor Szabo: Okay. Whatever. Bro.

633
01:18:07.240 --> 01:18:15.210
Márk Tolmács: Top right in the in the puzzle piece you you should see your extensions puzzle the puzzle piece, icon

634
01:18:15.430 --> 01:18:17.660
Márk Tolmács: top. Right there you go.

635
01:18:18.240 --> 01:18:25.750
Gabor Szabo: This one it's can you see it? I don't see the this one. Yeah. It says it's in in there.

636
01:18:26.180 --> 01:18:27.630
Gabor Szabo: Okay, interesting.

637
01:18:28.410 --> 01:18:29.040
Márk Tolmács: Yep.

638
01:18:29.620 --> 01:18:31.620
Gabor Szabo: I mean for me. It's so small.

639
01:18:33.490 --> 01:18:37.449
Márk Tolmács: I don't know how you can zoom in. Unfortunately.

640
01:18:37.450 --> 01:18:44.589
Gabor Szabo: No, my, this is my internal screen, and the resolution is, maybe I should have changed the resolution, and but I don't want to touch it now.

641
01:18:44.860 --> 01:19:04.180
Márk Tolmács: Live and learn. Anyway, you need to do one other modification because it's a security risk. And you need to make the change manually in your preferences in the dev tools, preferences in the cook icon bottom left ish? No, it's it's in the devtools where you, where your console

642
01:19:04.330 --> 01:19:10.350
Márk Tolmács: process is to the right, to the right all all the way to the right all the way.

643
01:19:10.520 --> 01:19:19.210
Márk Tolmács: No, no, the cog, icon. And then it's already in the preferences, and you have to scroll a little bit down in the in this pane.

644
01:19:19.370 --> 01:19:24.030
Márk Tolmács: and you will see a section called Allow Devtools to load resources.

645
01:19:24.340 --> 01:19:25.910
Márk Tolmács: It should be there.

646
01:19:27.170 --> 01:19:38.200
Márk Tolmács: there you go. It's on the bottom. Yes, you have to click that, because otherwise it will not load your source files properly. When you actually

647
01:19:38.380 --> 01:19:42.029
Márk Tolmács: call it the breakpoint. So that's an extra step you need to do.

648
01:19:42.420 --> 01:19:49.660
Márk Tolmács: I don't know. Sometimes they change this with chrome releases. Right now, this is the magic switch which solves the problem.

649
01:19:49.830 --> 01:19:54.720
Márk Tolmács: The other thing which we would need is add the rest source code.

650
01:19:55.000 --> 01:20:05.869
Márk Tolmács: I don't know if you already added your rust source code to your system. You can find you can find the command in in my articles. Rust up component, add rust

651
01:20:06.310 --> 01:20:07.300
Márk Tolmács: source.

652
01:20:08.580 --> 01:20:10.510
Márk Tolmács: So let's get back to your yeah.

653
01:20:10.510 --> 01:20:11.100
Gabor Szabo: This one.

654
01:20:11.100 --> 01:20:18.392
Márk Tolmács: Go. Yep, add, let's add the source component. If it's not there yet, because if you step into

655
01:20:19.270 --> 01:20:28.579
Márk Tolmács: in a stack, trace you step into the Rust library. You won't have Source View, which is unfortunate. Oh, there you go! You already had it.

656
01:20:28.890 --> 01:20:34.490
Márk Tolmács: And then here's the other part of the debugging magic in the article you have to add this part.

657
01:20:34.630 --> 01:20:43.900
Márk Tolmács: this metadata change to your cargo, Tomo. What it does is it tells. Wasn't back our favorite builder

658
01:20:44.290 --> 01:20:56.580
Márk Tolmács: to do not do not throw out the dwarf debugging information because it does throw out the dwarf debugging information which we will need in the browser in order to resolve

659
01:20:56.710 --> 01:20:59.519
Márk Tolmács: function pointers and variable names.

660
01:21:02.010 --> 01:21:10.760
Márk Tolmács: And if everything went well and everything rebuilds, you should see in your console that the the library already picks up the dwarf.

661
01:21:11.105 --> 01:21:12.140
Gabor Szabo: The other console.

662
01:21:12.710 --> 01:21:15.630
Márk Tolmács: This one this one. Sorry I'm gonna call it terminal.

663
01:21:15.630 --> 01:21:17.670
Gabor Szabo: This was too many calls.

664
01:21:17.920 --> 01:21:22.139
Márk Tolmács: Yeah, this one. That's probably because I'm Hungarian, and

665
01:21:22.430 --> 01:21:31.440
Márk Tolmács: that's how you used it here. So if it rebuilds in the terminal, you should see in your console that the the

666
01:21:31.920 --> 01:21:42.359
Márk Tolmács: the extension, the chrome extension, picked up the debug information. Can you see if you reload in Chrome? Can you see that it picked up.

667
01:21:45.340 --> 01:21:46.420
Gabor Szabo: How could I see it?

668
01:21:47.310 --> 01:21:54.402
Márk Tolmács: It should be in your console, but I see it's not present. Yep, it should be there.

669
01:21:56.660 --> 01:22:04.239
Márk Tolmács: And can you please open up the little puzzle piece, and see if the extension is enabled, because it might not be enabled.

670
01:22:06.210 --> 01:22:08.070
Márk Tolmács: Yeah.

671
01:22:08.980 --> 01:22:12.610
Gabor Szabo: Let me move it to the other. Screen it

672
01:22:18.570 --> 01:22:19.430
Gabor Szabo: I

673
01:22:24.620 --> 01:22:26.639
Gabor Szabo: this I see.

674
01:22:26.640 --> 01:22:32.960
Márk Tolmács: It is enabled it should be enabled. Then, hmm!

675
01:22:33.470 --> 01:22:41.600
Márk Tolmács: What could be? Maybe maybe we should delete just exit the right from terminal. The process.

676
01:22:41.600 --> 01:22:44.979
Gabor Szabo: And and force force it to recompile everything.

677
01:22:44.980 --> 01:22:50.539
Márk Tolmács: Yes, because I suspect that it's still not putting in the dwarf information.

678
01:22:51.240 --> 01:22:57.139
Márk Tolmács: So just cancel. Cancel it in your terminal. Have removed the Pkg Directory.

679
01:22:57.140 --> 01:22:58.400
Gabor Szabo: This one.

680
01:22:58.400 --> 01:23:07.639
Márk Tolmács: Yep. Remove the Pkg Directory just to be absolutely 100%. Sure, it's not gonna reuse the old compilation.

681
01:23:07.990 --> 01:23:08.730
Gabor Szabo: That's.

682
01:23:09.050 --> 01:23:15.449
Márk Tolmács: Yep, and then just restart, and it has to recompile everything now.

683
01:23:17.290 --> 01:23:20.360
Márk Tolmács: and let's see if it loads piston.

684
01:23:20.360 --> 01:23:20.840
Gabor Szabo: Okay.

685
01:23:20.910 --> 01:23:31.399
Márk Tolmács: There you go. So now the debug information is read by the extension, and it is available in your console. So now, if we open up sources

686
01:23:31.720 --> 01:23:54.939
Márk Tolmács: that sources tab in the console the dev tools. Yes, and just give us a little bit of space, and on the left panel just extend the left panel a little bit so we can see there is something new. We didn't check it before, but there is something new you wouldn't see without the extension is the file section at the bottom. You can open it up.

687
01:23:55.120 --> 01:24:02.389
Márk Tolmács: Yes, it should contain your home directory already, and a couple of other things.

688
01:24:02.650 --> 01:24:06.099
Márk Tolmács: Just open up your home directory. It's home. Gabor.

689
01:24:07.570 --> 01:24:09.070
Gabor Szabo: Will I see all the.

690
01:24:09.800 --> 01:24:29.709
Márk Tolmács: And there you go. Work, wasm and web source. It's not all your files. It's just what the wasm compilation contains. Just open up work wasm and web source, and we will see that we have lib. Dot Rs in it. Just open it up and we see the source code

691
01:24:29.830 --> 01:24:38.040
Márk Tolmács: perfect. Maybe just leave a breakpoint somewhere, just to see if if it works

692
01:24:38.460 --> 01:24:41.049
Márk Tolmács: forever. It's reasonable in that file.

693
01:24:43.210 --> 01:24:45.720
Gabor Szabo: This is how I put a break. I never use this.

694
01:24:45.990 --> 01:24:49.730
Márk Tolmács: Oh, never mind, then, it's that's it perfect, and just reload.

695
01:24:50.510 --> 01:24:57.150
Márk Tolmács: Reload the browser page, and don't close this window, of course.

696
01:24:58.230 --> 01:25:06.559
Márk Tolmács: and there you go. We are at a break point. It actually stopped, as you can see, and on the right side you can see the parameters.

697
01:25:07.400 --> 01:25:16.789
Gabor Szabo: Go to the terminal if I could go to the terminal it's console now. Sorry. So it didn't print out those 2 messages yet.

698
01:25:17.360 --> 01:25:24.899
Márk Tolmács: No, it's actually holding on the breakpoint exactly. And you can see the parameters on the right side.

699
01:25:24.900 --> 01:25:26.430
Gabor Szabo: Oh, okay, nice.

700
01:25:26.990 --> 01:25:27.640
Gabor Szabo: I like it.

701
01:25:28.346 --> 01:25:30.489
Gabor Szabo: The steps here, so I can step.

702
01:25:31.530 --> 01:25:33.450
Gabor Szabo: That's step one.

703
01:25:33.780 --> 01:25:36.473
Gabor Szabo: No, they step in. So let's step out.

704
01:25:39.300 --> 01:25:40.829
Gabor Szabo: Oh, I guess that.

705
01:25:43.500 --> 01:25:49.540
Márk Tolmács: Yes, here's here's the here's the little caveat part. It's

706
01:25:49.670 --> 01:25:56.810
Márk Tolmács: it's great. If you don't have other tools to debug your complex interaction with the

707
01:25:56.870 --> 01:26:25.750
Márk Tolmács: typescript and javascript and and wasm, but it has its limitations. It usually leads you to memory addresses which are not very useful unless you do know what you're doing, and you can inspect the Javascript memory and gather what exactly Webassembly stores at the given memory address. But in general I found it easy to just have a quick step through in your code when you don't know what's going on, which was previously.

708
01:26:26.020 --> 01:26:33.829
Márk Tolmács: I mean, a couple of years ago. It was super hard. Now it's actually super easy, as you can see. You can set it up for yourself.

709
01:26:33.990 --> 01:26:35.629
Márk Tolmács: and then.

710
01:26:36.370 --> 01:26:47.840
Márk Tolmács: you know, step through the code. See what's going on, and you can actually do the hybrid step through. So whenever the Javascript part steps into the rust code, you can follow it along.

711
01:26:48.765 --> 01:26:55.020
Márk Tolmács: But understand that you you are using. Wasm Winchen.

712
01:26:55.330 --> 01:27:18.519
Márk Tolmács: It generates a lot of code in the background which you won't see unless you do debugging, in which case you will see every details and what exactly generated, in order to make sure that the interaction between Javascript and and Wasm rust works

713
01:27:18.610 --> 01:27:29.540
Márk Tolmács: seamlessly like we experienced today. So the reason I wanted you to add the rest source code and we haven't run into that yet is that

714
01:27:29.960 --> 01:27:58.320
Márk Tolmács: you can actually map it up to this devtools, console and have the chrome devtools open up your rust library sources if you want to. I don't know if you want to do it. It's not that complicated. What you need to do is you need to somehow copy. I don't know if you can copy the part in the file, in the left side of sources, panel under file, you will see something like

715
01:27:58.440 --> 01:28:08.400
Márk Tolmács: to the left. Yes, at the bottom you will see. Yep, something like oh, you already stepped into it! That's great to the bottom. Go to the bottom!

716
01:28:10.260 --> 01:28:19.579
Márk Tolmács: Yes, some of yes, so you will see that it says, rust. C slash, 9 0. B. 3, 5 a. It's a hash.

717
01:28:20.120 --> 01:28:20.650
Márk Tolmács: See?

718
01:28:20.650 --> 01:28:21.409
Gabor Szabo: Oh! This one!

719
01:28:21.410 --> 01:28:25.090
Márk Tolmács: Can you? Can you copy copy that directory name?

720
01:28:28.450 --> 01:28:29.640
Gabor Szabo: Okay.

721
01:28:33.320 --> 01:28:36.169
Gabor Szabo: I'll I'll take a look at in the other window.

722
01:28:40.290 --> 01:28:42.499
Gabor Szabo: No, it doesn't. It doesn't have a copy there.

723
01:28:43.130 --> 01:28:55.050
Márk Tolmács: Don't worry. I hope that we can actually do it live. But maybe I just quickly show it to you guys and then we wrap it up. Why, I wanted you to copy and paste it if you bring it back.

724
01:28:55.561 --> 01:29:02.240
Márk Tolmács: There is a neat trick with the chrome extension just open up the just huge.

725
01:29:02.605 --> 01:29:06.629
Gabor Szabo: Do. I don't have this in the in the other terminal.

726
01:29:06.870 --> 01:29:13.329
Márk Tolmács: So. Unfortunately, this is an artifact in in dwarf. It uses this masked

727
01:29:14.214 --> 01:29:38.079
Márk Tolmács: Directory or Virtual Directory for the fire references. But let me tell you what, and it's actually detailed in in my article. Let me tell you what I would do with it, so you can actually map it to your rest source codes. If we were able to copy and paste that name. If you finish up the the debug session because it's gonna cause some problems for us. Just let it run and remove the.

728
01:29:38.080 --> 01:29:38.690
Gabor Szabo: This one.

729
01:29:38.690 --> 01:29:42.750
Márk Tolmács: The yep, let it run and remove the breakpoints.

730
01:29:43.826 --> 01:29:54.263
Márk Tolmács: Yes, just go for it. Okay? So if you open up your puzzle, piece your extensions. You will see that there is a 3 dot menu

731
01:29:54.650 --> 01:29:56.080
Gabor Szabo: Does it go through this one.

732
01:29:56.390 --> 01:30:01.240
Márk Tolmács: No, no top, right top, right top, right bezel piece. The extension manager.

733
01:30:01.830 --> 01:30:02.300
Gabor Szabo: Okay.

734
01:30:02.300 --> 01:30:07.810
Márk Tolmács: Yes, the 3 dots options.

735
01:30:11.240 --> 01:30:14.259
Márk Tolmács: There you go. Can you zoom in a little bit so we can see.

736
01:30:14.260 --> 01:30:16.180
Gabor Szabo: This sort of you can actually.

737
01:30:16.400 --> 01:30:40.280
Márk Tolmács: You can actually add path substitutions. So these virtual directories, like the rust C slash, the hash code, can be replaced with your absolute path to your rust source, and what the little extension will do is going to access your local file system and load the rust source code files, too. So actually, you would be able to have

738
01:30:40.280 --> 01:30:51.430
Márk Tolmács: full source code all the way down to the bottom of the stack. It's just a neat trick you would enjoy. If you actually want to step through the code.

739
01:30:51.470 --> 01:30:55.760
Márk Tolmács: the other probably more useful effect

740
01:30:56.120 --> 01:31:22.389
Márk Tolmács: adding, these rust source codes could be is that if you, if you were to generate a an exception or a panic, it would actually on your console. It would actually display the stack with the proper findings and paths and line numbers, which is very useful. Debugging and collecting information on on crashes, because

741
01:31:22.630 --> 01:31:24.690
Márk Tolmács: that happens even with rust.

742
01:31:25.900 --> 01:31:26.764
Márk Tolmács: Okay,

743
01:31:27.980 --> 01:31:28.930
Márk Tolmács: So.

744
01:31:28.930 --> 01:31:30.479
Gabor Szabo: Just a panic in there.

745
01:31:31.160 --> 01:31:37.460
Márk Tolmács: We can try. I don't know if it actually will resolve everything now that you don't have the source code mapping in there.

746
01:31:38.470 --> 01:31:39.599
Márk Tolmács: Okay. And there is another.

747
01:31:40.790 --> 01:31:41.400
Gabor Szabo: Yeah.

748
01:31:41.570 --> 01:31:42.260
Gabor Szabo: Good luck.

749
01:31:42.260 --> 01:31:51.510
Márk Tolmács: Let's put in a panic. Let's see if it let's see if it logs it properly. At least it should show the lead. Dot Rs file the 1st line in the stack.

750
01:31:51.770 --> 01:31:52.630
Márk Tolmács: Okay.

751
01:32:01.470 --> 01:32:02.460
Gabor Szabo: Okay.

752
01:32:02.880 --> 01:32:03.870
Márk Tolmács: Let's see.

753
01:32:04.870 --> 01:32:09.100
Gabor Szabo: And I don't know if it should do something here.

754
01:32:10.940 --> 01:32:15.720
Márk Tolmács: You would see it in your console, if anywhere.

755
01:32:16.510 --> 01:32:18.580
Gabor Szabo: This console? Yeah, yes.

756
01:32:19.140 --> 01:32:25.200
Márk Tolmács: So this is the stack trace from your panic, and, as you can see.

757
01:32:26.110 --> 01:32:32.759
Márk Tolmács: why is it upside down anyway? The last one is the remote instance, parent function.

758
01:32:33.648 --> 01:32:36.400
Márk Tolmács: from the Javascript hosting

759
01:32:36.600 --> 01:32:44.209
Márk Tolmács: previous to that is, the hoisting from the the rush side, and then the 3rd to last

760
01:32:44.400 --> 01:32:49.940
Márk Tolmács: is our function in the Lib. Dot, Rs

761
01:32:50.870 --> 01:32:55.290
Márk Tolmács: dip, dot Rs file in the 38th line, which is exactly

762
01:32:55.400 --> 01:32:58.149
Márk Tolmács: where the panic is. I'm guessing.

763
01:32:59.390 --> 01:33:02.610
Márk Tolmács: and you can see some of the other stack trees.

764
01:33:03.440 --> 01:33:03.980
Gabor Szabo: Th-this.

765
01:33:03.980 --> 01:33:06.275
Márk Tolmács: Never mind, it's it's the 4th one. Sorry, yes.

766
01:33:07.970 --> 01:33:11.039
Márk Tolmács: Kind of mixed, mixed up a little bit. Yes.

767
01:33:11.310 --> 01:33:11.890
Gabor Szabo: Yeah, okay.

768
01:33:12.480 --> 01:33:20.749
Márk Tolmács: So that's why it's useful to have this set up. And that's why it's useful to have this little extension. It would make your panics.

769
01:33:22.480 --> 01:33:26.399
Márk Tolmács: Clear, basically what happened where exactly you need to look.

770
01:33:26.790 --> 01:33:32.350
Márk Tolmács: Yes, it contains some of the artifacts from wasn't been gem. But I think

771
01:33:32.550 --> 01:33:37.170
Márk Tolmács: it's arguably better that you have this information at your fingertips.

772
01:33:37.650 --> 01:33:38.800
Gabor Szabo: Yeah, indeed.

773
01:33:39.530 --> 01:33:40.350
Márk Tolmács: So

774
01:33:42.260 --> 01:33:54.759
Márk Tolmács: as a final step, and we won't do it in ourselves in in code. Just want to point it out in the article, because we actually spent a lot of time with this.

775
01:33:54.870 --> 01:34:03.363
Márk Tolmács: Is that indeed there is a way to run it outside of the browser, and in order to call

776
01:34:04.837 --> 01:34:28.040
Márk Tolmács: Sys system calls just like you would with C or rust. Directly. There is an Api called Wazi or Webassembly system interface, which tries to re-implement all the usual stuff you would get on. Say, node Gs or rust. So file handlers, network access

777
01:34:28.100 --> 01:34:43.407
Márk Tolmács: anything else, basically that, we would need to have a full fledged platform and run it on. Say, the server somebody mentioned maybe it was Gregory. I'm not sure that people starting to pick it up. And

778
01:34:43.820 --> 01:35:08.570
Márk Tolmács: it's starting to gain traction in web 3 crypto spaces because it's ability or capability to run in a platform independent manner. But I also enjoy seeing it picked up by sort of like the new new generation of Javascript bundlers like Swc. Built by versa

779
01:35:08.630 --> 01:35:17.335
Márk Tolmács: as a as a Plugin interface. So basically, anybody can create plugins in any language compared to wasm

780
01:35:18.240 --> 01:35:30.070
Márk Tolmács: with Wasi. And these bundlers can load it, no matter which platform they run in, and I'm sure there are tons of other use. Cases and people are starting to experiment with it.

781
01:35:30.300 --> 01:35:40.379
Márk Tolmács: In the end of the article I provided a somewhat older repository which uses an old version of Swc just to transpile

782
01:35:40.590 --> 01:35:47.569
Márk Tolmács: a sample Javascript file from the standard. Input it's basically that's what it says.

783
01:35:47.740 --> 01:35:59.040
Márk Tolmács: The specialty is that it can actually open standard input and standard output and use it. Which wouldn't be possible without the wasi interface.

784
01:36:00.640 --> 01:36:13.020
Márk Tolmács: Okay, I think that's it for my end. If you have any final questions on any of the things we talked about, and I'm happy to answer them.

785
01:36:14.210 --> 01:36:16.034
Gabor Szabo: I'm exhausted. Thank you.

786
01:36:16.900 --> 01:36:18.129
Márk Tolmács: I can imagine.

787
01:36:19.990 --> 01:36:35.329
Gabor Szabo: Anyone any questions before we stop the video. Yes, there is. Wasn't Banjan returning back? An unreachable panic is very common in the console. Really hard to understand. So this debug way could be really useful. Thanks for.

788
01:36:35.330 --> 01:36:36.190
Márk Tolmács: You're welcome.

789
01:36:36.190 --> 01:36:37.969
Gabor Szabo: I think you were evoking.

790
01:36:40.620 --> 01:36:43.419
Gabor Szabo: X. Isn't that? That is a frame.

791
01:36:43.420 --> 01:36:43.900
Márk Tolmács: Yes.

792
01:36:44.150 --> 01:36:45.040
Gabor Szabo: Mosey.

793
01:36:45.180 --> 01:37:04.040
Márk Tolmács: That there are. There are a lot of frameworks which make it easy. I try to create a basic, a very basic example, just for a tutorial presentation and article, hopefully and self contained one. Hopefully, that would make it easy for you guys to get started.

794
01:37:04.370 --> 01:37:09.490
Márk Tolmács: Try it out, and then you can branch out and learn more, and

795
01:37:09.600 --> 01:37:21.430
Márk Tolmács: that's actually a great segue to the last paragraph, and I added to the article as well where to go from here, and these are 2 open source available books.

796
01:37:21.560 --> 01:37:28.799
Márk Tolmács: The Rust Webassembly Book, which is a great starter. This presentation is heavily relied on that book.

797
01:37:29.070 --> 01:37:48.340
Márk Tolmács: and also the Wasm Bingon Guide which gives you an insight into how wasn't Bing Gen works and gives you lots of examples and techniques to do practically anything. It wasn't bingon. If you actually process and broke these 2 books.

798
01:37:48.500 --> 01:38:05.213
Márk Tolmács: you would be well versed to do practically anything with Webassembly on the web in the browser was somewhat different. There are various materials again. It's very early still.

799
01:38:06.850 --> 01:38:13.699
Márk Tolmács: it works. But it's most of the time it's unstable. I actually mentioned that a node is already

800
01:38:13.880 --> 01:38:25.590
Márk Tolmács: sort of. Was it compatible? Although it's in an experimental mode. Node 23 can now load was the preview one binaries and and run them

801
01:38:26.530 --> 01:38:30.830
Márk Tolmács: sort of a good spirit, with sort of a good success rate, I would say.

802
01:38:33.030 --> 01:38:44.220
Gabor Szabo: There is another comment. There seem to be so many moving parts, any of which can cause a problem. What is the simplest, smallest combination of tools to enable rust to augment

803
01:38:44.370 --> 01:38:45.970
Gabor Szabo: HTML. 5.

804
01:38:46.800 --> 01:38:49.470
Márk Tolmács: I would say this post

805
01:38:49.720 --> 01:39:00.570
Márk Tolmács: wanted to do, and this presentation wanted to do exactly that. I would say the 1st half when we hooked up vite the plugins

806
01:39:00.920 --> 01:39:08.950
Márk Tolmács: and wasn't been. Gen. Is the simplest case of all today. If you want to use this technology stack

807
01:39:09.676 --> 01:39:13.359
Márk Tolmács: again, I understand that it's overwhelming. It's

808
01:39:13.610 --> 01:39:23.860
Márk Tolmács: not a simplest topic on itself. But if you, I guarantee, if you get started tomorrow and just follow the article

809
01:39:24.327 --> 01:39:33.439
Márk Tolmács: or the post it would be. It would make more sense, especially that now that we have a recording, it's easy to just go back, stop it.

810
01:39:33.960 --> 01:39:50.690
Márk Tolmács: and maybe get some more context from my article, or just open up the rust Webassembly book, or there was some bind Gen. Book and just figuring out what's going on. But once you once it clicks just like with the borrow checker, I would say, once it clicks, it just makes sense.

811
01:39:52.760 --> 01:39:58.430
Gabor Szabo: Yeah. Well, yes, it just takes a long time to click. Yeah, I guess.

812
01:39:58.430 --> 01:40:00.539
Márk Tolmács: This one. I hope it's easier.

813
01:40:00.870 --> 01:40:05.910
Gabor Szabo: Yeah, no, it it seemed. I mean now that it's working together.

814
01:40:06.380 --> 01:40:15.800
Gabor Szabo: it seems easy to copy, paste something and then tweak a little bit and then start adding more functions.

815
01:40:16.120 --> 01:40:24.490
Gabor Szabo: So this this is what I actually, I'm missing from a lot of pieces of code, and

816
01:40:24.850 --> 01:40:29.879
Gabor Szabo: even in rost. But but in many places that in I mean.

817
01:40:32.190 --> 01:40:41.500
Gabor Szabo: we don't read that many document, that much of the documentation usually only when everything else fails we usually copy, paste and then try, start to tweak and

818
01:40:41.640 --> 01:40:43.470
Gabor Szabo: and want something to work.

819
01:40:44.560 --> 01:40:54.947
Gabor Szabo: I mean, at least I but I think I see many other people do that. So the fact that we have something working

820
01:40:55.980 --> 01:40:59.689
Gabor Szabo: the whole whole cycle here.

821
01:40:59.790 --> 01:41:07.449
Gabor Szabo: That's I think it's great. So this is how from from from this point. It's it's very easier to

822
01:41:07.910 --> 01:41:09.440
Gabor Szabo: to do all kind of other things.

823
01:41:09.440 --> 01:41:12.349
Márk Tolmács: So I hope so. That's the goal.

824
01:41:12.620 --> 01:41:20.520
Gabor Szabo: Yeah, so thank you very much. And if yeah.

825
01:41:20.720 --> 01:41:31.560
Gabor Szabo: the presentation was awesome and huge work of preparation. I'm using this technology for 2 years, and I believe this is the future for me. It's production

826
01:41:31.700 --> 01:41:37.949
Gabor Szabo: improved every day. Indeed, the 1st thing to really understand the power of this

827
01:41:38.060 --> 01:41:42.670
Gabor Szabo: duo is to log and play between wasn't.

828
01:41:43.040 --> 01:41:51.360
Gabor Szabo: And then with structs, you can put 90% of the code in Rust and Ts is just using the library

829
01:41:51.980 --> 01:41:56.517
Gabor Szabo: indeed interesting, and thank you very much, Mark. It was absolutely

830
01:41:57.930 --> 01:42:07.330
Gabor Szabo: I don't know that I would like. I would like to use some very high burden English. But I don't know anymore anything. So I just want to thank you

831
01:42:08.120 --> 01:42:09.190
Gabor Szabo: and thank you.

832
01:42:09.190 --> 01:42:10.240
Márk Tolmács: Having me.

833
01:42:10.430 --> 01:42:28.560
Gabor Szabo: Yeah, and take care of everyone who was watching to this point. And please like the video and follow the channel. And I will put under the the video some links. So you can. I mean, you have the article. You'll have the article and how to find Mark. So

834
01:42:28.790 --> 01:42:30.229
Gabor Szabo: that's it right?

835
01:42:31.440 --> 01:42:32.070
Márk Tolmács: I think so.

836
01:42:32.070 --> 01:42:47.039
Márk Tolmács: So. Yeah, it was a great time having, you guys thanks for the questions. Thanks for coming with us on this Preston Webassembly Journey, and I hope to see many, many great Webassembly solutions.

837
01:42:49.000 --> 01:42:50.039
Márk Tolmács: Let us know.

838
01:42:50.590 --> 01:42:52.690
Gabor Szabo: Okay, great. Thank you very much. And

839
01:42:52.990 --> 01:42:53.500
Márk Tolmács: Thank you.

840
01:42:53.500 --> 01:42:55.250
Gabor Szabo: We'll see you soon. Bye, bye.

841
01:42:55.250 --> 01:42:56.839
Márk Tolmács: See you. Bye-bye.

