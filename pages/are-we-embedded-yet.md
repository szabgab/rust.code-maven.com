---
title: Are We Embedded Yet? - Implementing tiny HTTP server on a microcontroller with Maor Malka
timestamp: 2025-01-02T20:30:01
author: szabgab
published: true
show_related: true
description:
tags: []
---

Given Rust's destiny to become the go-to replacement for C, a clear target for such a change would be the world of embedded microcontrollers. we sometimes tend to ignore these devices, but they cover so much of our basic appliances and they all are running C. The embedded world is notorious for its lack of capabilities of reuse and following proper safe code guidelines. this is especially true when most Embedded C developers use many tricks to get the most optimal result in regards to speed and size.
I wanted to try and leverage the power of Rust to show both its capabilities to create advanced projects easily, and do so in a "safe" manner.
I will show an example project, done on a custom STM32 microcontroller board with only 128KB Flash and 40KB running a HTTP server by becoming a USB Ethernet Adapter.
Link to Repo of stamrust

About [Maor Malka](https://www.linkedin.com/in/maor-malka-a46640134/):
Currently a Digital Design Engineer at ARBE robotics Been doing embedded, board design, FPGA and system design for the past 11 years. Now starting to add rust to my toolbox 😉

[repository](https://github.com/maor1993/stamrust/)

![Maor Malka](images/maor-malka.jpeg)


{% youtube id="NclcQcNcLI4" file="2025-01-30-are-we-embedded-yet-implementing-tiny-http-server-on-a-microcontroller.mp4" %}

## Transcript

WEBVTT

1
00:00:01.870 --> 00:00:22.670
Gabor Szabo: Okay, so hello and welcome. That's what I wanted to say. Codemaven meetings and the code Maven Youtube Channel. If you're watching in this in the video. If you're watching, please, like the video and follow the channel before I forget.

2
00:00:22.760 --> 00:00:34.290
Gabor Szabo: My name is Gabor Sabu. I myself teach rust and teach python and help companies introduce testing and Ci.

3
00:00:35.110 --> 00:00:59.180
Gabor Szabo: and I think sharing knowledge is is really important. I've been organizing various meetups for more than 20 years now, and mostly in person. But recently I started this this idea of of inviting people and sharing their their knowledge. And I'm really happy that it's still it's it's starting to take off.

4
00:00:59.440 --> 00:01:09.740
Gabor Szabo: So if you have any idea that we'd like to present to the audience, then you're welcome to connect contact me, and and we can discuss a presentation.

5
00:01:09.920 --> 00:01:15.200
Gabor Szabo: and this time we are going to talk about rust and embedded rust, and.

6
00:01:15.330 --> 00:01:18.489
Gabor Szabo: moreover, I'm going to give the the

7
00:01:18.800 --> 00:01:47.779
Gabor Szabo: place for for more, and he'll introduce himself much better than I do, and thank you for coming, and thank you for watching the video. So that's that's not your turn. And just before I finish this, so if you are here in the video, in the in the meeting, feel free to ask questions, that's 1 of the nice parts of participating in the in the event that you can ask questions in the chat and then stay around discussing things so.

8
00:01:48.180 --> 00:01:49.180
maor malka: More.

9
00:01:50.160 --> 00:01:55.669
maor malka: Thank you, Bubble. I'm going to share my screen. Let me just make sure everybody can see it.

10
00:01:56.803 --> 00:02:00.270
maor malka: Just a picture. Everybody can see the screen.

11
00:02:01.656 --> 00:02:02.750
Gabor Szabo: Yes, we.

12
00:02:03.420 --> 00:02:04.120
maor malka: See the presentation.

13
00:02:04.120 --> 00:02:06.779
Gabor Szabo: I can see the. Are we embedded yet.

14
00:02:07.140 --> 00:02:08.470
maor malka: Yeah, okay, perfect.

15
00:02:09.060 --> 00:02:22.380
Gabor Szabo: I wanted to say that we we saw this. We noticed this an earlier meeting, that some reason that the slides weren't followed well, by zoom. So please mention when, if you feel that it's off.

16
00:02:22.610 --> 00:02:26.168
maor malka: Okay. Okay. Hi, everyone

17
00:02:27.270 --> 00:02:37.099
maor malka: Welcome to the presentation called, are we embedded? Yet? This is the story of my little journey to implement, a tiny Http server on a microcontroller.

18
00:02:38.070 --> 00:02:48.470
maor malka: So who am I? Hi, I'm or I'm an electronics engineer and a maker. I currently work for arbay as a digital design engineer.

19
00:02:48.590 --> 00:03:00.349
maor malka: I've been doing embedded hardware design logic design for the past 11 years. And I got interested in rust. And I decided, I want to try and add it to my toolbox of knowledge.

20
00:03:01.900 --> 00:03:06.990
maor malka: So what are we gonna talk about today? I hope you're seeing the 1st slide right now with the agenda. Yeah.

21
00:03:06.990 --> 00:03:07.640
Gabor Szabo: Yep.

22
00:03:07.640 --> 00:03:37.330
maor malka: Okay, perfect. What are we gonna talk about today? We're gonna do a quick run to about what our microcontrollers we're gonna talk. What is the challenge with using rust in the embedded environment. We're gonna look at the use case. I was trying to create what's the process was to create the the drivers and the tool set to actually make it work. What is the build environment usually use? How do you debug code? And we're gonna look at a bit of demos.

23
00:03:37.830 --> 00:03:40.840
maor malka: And once we finalize all that we can talk about conclusions.

24
00:03:41.450 --> 00:04:00.910
maor malka: So what are microcontrollers? Microcontrollers are compact, self-contained computing devices? They consist of the processor memory peripherals and sometimes specialized hardware to do specific things like, for example, USB control cameras displays, and it's all integrated into one single chip.

25
00:04:01.480 --> 00:04:23.189
maor malka: Usually we use microcontrollers when we need something to work as real time operations. Most of the times you take a microcontroller and you run pure code on it without an operating system running some kind of just while loop or an interrupt driven loop. So it will do actions depending on. For example, if a button is pressed.

26
00:04:23.540 --> 00:04:40.240
maor malka: or in other cases that are also super common. You use something called a real time operating system, which lets you create somewhat something equivalent of threads, but they are very deterministic, and you can know exactly when each one will run.

27
00:04:41.010 --> 00:04:48.310
maor malka: They're usually also designed for low power consumption. So you most of the things that use batteries would use microcontrollers.

28
00:04:48.510 --> 00:04:58.550
maor malka: And yes, they're everywhere, in every, almost every electronic product that isn't using some form of an advanced processor will probably use a microcontroller.

29
00:05:00.050 --> 00:05:02.880
maor malka: So what types of microcontrollers are there?

30
00:05:03.100 --> 00:05:27.629
maor malka: There's a huge variety, and it's a constant battle between getting the most performance or the lowest power or the cheapest price. Even if we look, for example, at architectures, there are so many different architectures that are still being used today. If, for example, look at 8 bit architectures. We can talk about Arduino, for example, which uses its own architecture called Avr.

31
00:05:27.680 --> 00:05:38.170
maor malka: Those are the original ones, not the modern ones, we can think about the 80 51 which is super common to see in in more like industrial electronics.

32
00:05:38.260 --> 00:06:04.359
maor malka: In the sixteen-bit world, we have, for example, the Msp. 440, and of course, in the 40 two-bit world we have the arm which the M series which stands for Microcontroller. We have Risc 5, which is now starting to become much more popular in actual products. And we also have MIPS which you can see in some older generation of microcontrollers.

33
00:06:04.730 --> 00:06:18.760
maor malka: The amount of memory we usually have in these in these microcontrollers is very small. You can see examples that can be as low as 8 kB of flash memory, and 512 Byte of RAM, and sometimes even lower.

34
00:06:19.450 --> 00:06:28.259
maor malka: And even to this day you can still buy microcontrollers, which are one time programmable, and they can be as cheap as 3 cents.

35
00:06:29.570 --> 00:06:33.290
maor malka: Okay, that's great. And all, what about rust?

36
00:06:34.260 --> 00:06:46.810
maor malka: So let's talk about what is the main challenge with doing rust with microcontrollers. Most microcontrollers, if not all of them, implement a single memory space

37
00:06:46.920 --> 00:06:55.970
maor malka: for everything which means your code. All the peripherals RAM, everything is mapped into a single memory space.

38
00:06:56.770 --> 00:07:07.660
maor malka: That means, for example, if you want to blink an led, you need to write or read from a specific memory address in the memory space.

39
00:07:08.160 --> 00:07:16.270
maor malka: Why is that such a problem? Well, that essentially breaks a lot of Rust's memory safety concepts.

40
00:07:16.670 --> 00:07:24.900
maor malka: And we can see an example of that. If we look at a single example, just making sure everybody is seeing the table. That says Systic.

41
00:07:26.250 --> 00:07:27.495
maor malka: Yes.

42
00:07:28.740 --> 00:07:37.340
maor malka: okay, no glitches so far, that's good. So every arm Microcontroller has a small piece of hardware called the systick.

43
00:07:37.660 --> 00:07:45.539
maor malka: which is a counter that can count clock cycles. You can use it to calculate a small delay or

44
00:07:45.710 --> 00:07:48.210
maor malka: to count to count elapsed time.

45
00:07:48.860 --> 00:08:06.449
maor malka: And the way you do that is this block has 4 registers that you can access. Starting at a specific memory offset. And, as you can see in this example, 3 of these registers are read-write registers, and one of them is a read-only register.

46
00:08:08.060 --> 00:08:13.369
maor malka: So okay, let's let's try to implement it in rust. So

47
00:08:13.570 --> 00:08:18.550
maor malka: the 1st thing we do is say, okay, let's write this as a, it's not clicking

48
00:08:19.210 --> 00:08:24.309
maor malka: is the what's going on. Oh, sorry little glitch.

49
00:08:26.450 --> 00:08:31.429
maor malka: So say, okay, let's let's implement it in rust. So let's write a struct

50
00:08:31.560 --> 00:08:40.549
maor malka: called Systic, which has 4 4 fields, one for each one of these registers. The size is 32 bits.

51
00:08:40.919 --> 00:09:01.579
maor malka: and we'll use a special thing above the struct to tell rust to not try to break down this struct as in to keep it in the same style as C, which means that these 4 registers or 4 fields are essentially a continuous piece of memory.

52
00:09:02.430 --> 00:09:09.329
maor malka: Okay, now, we have the struct that represents this structure. How do we actually use it?

53
00:09:09.470 --> 00:09:14.710
maor malka: So this is, this is where, unfortunately, we need to start using unsafe code.

54
00:09:15.170 --> 00:09:36.560
maor malka: So to do this, we need to take the memory offset that we see here the 0 XE. 1,000 underscore e. 10, and cast it as a mutable reference to a systic struct, and then give a mutable reference to that, as the object that we use.

55
00:09:38.301 --> 00:09:44.298
maor malka: That's yeah. That's that's a mouthful. And to to make things even worse,

56
00:09:45.090 --> 00:09:48.489
maor malka: rust. And like C does this as well

57
00:09:48.730 --> 00:10:05.270
maor malka: because you don't want the compiler to optimize out these memory accesses. You have to do this, reads and writes in a volatile fashion, and doing these accesses in a volatile fashion is also unsafe code.

58
00:10:05.440 --> 00:10:07.519
maor malka: which means in just these

59
00:10:07.680 --> 00:10:16.299
maor malka: 6 lines we've had to use unsafe constantly in order to access something. That's a very basic part of our code.

60
00:10:17.370 --> 00:10:26.019
maor malka: Okay, so this is obviously very, very uncomfortable. And obviously, we don't just want to put unsafe on the entire code and just keep going. So what do we do?

61
00:10:26.280 --> 00:10:29.307
maor malka: One example that we can do is

62
00:10:30.100 --> 00:10:44.050
maor malka: create a struct which holds a static reference, a static mutable reference to the actual register block and hide all the unsafe operations behind functions that we present to the user.

63
00:10:44.380 --> 00:10:48.890
maor malka: When we do it like this, it means that, as you can see here under the

64
00:10:49.280 --> 00:11:07.080
maor malka: under a function called example usage. The user no longer has to write unsafe code, which means us as crape providers can handle and make sure that the unsafe side is perfect and the user doesn't have to actually run unsafe code on its own.

65
00:11:08.080 --> 00:11:11.149
maor malka: This is great and all. But what about the borrower checker?

66
00:11:12.110 --> 00:11:34.390
maor malka: So we can look at it at the embedded C idea and say, what we usually do in C is, say, let's just make this a global variable. Let's hold us a global struct with this thing we want to do. But the main thing is that when we write this in rust. We're essentially writing a static, mutable object

67
00:11:34.720 --> 00:11:40.950
maor malka: and to access static, mutable things. It is an unsafe operation. So we got nowhere.

68
00:11:41.100 --> 00:11:43.066
maor malka: So what do we do

69
00:11:43.830 --> 00:11:55.489
maor malka: The developers of the embedded rust word group have thought of an idea of representing each one of the peripherals as a single tone.

70
00:11:55.800 --> 00:12:00.770
maor malka: which means that you take the peripheral and you wrap it within an option.

71
00:12:01.330 --> 00:12:12.590
maor malka: and when you want to use that peripheral, you essentially take it, and by taking it the inner of the option becomes none. Which means the next time someone will try to take it.

72
00:12:13.256 --> 00:12:14.839
maor malka: You'll get a panic.

73
00:12:18.170 --> 00:12:32.840
maor malka: Okay, so now we have this basic tool set which allows us to actually access peripherals and use it in some form of a safe fashion and getting some of the safety capabilities of rust.

74
00:12:33.530 --> 00:12:46.199
maor malka: Obviously, we don't want to just go now and write every register block inside the chip. It can have hundreds of, if not thousands, of registers that we need to write. What do we do?

75
00:12:46.680 --> 00:12:59.279
maor malka: So the methodology that usually is done when it comes to the embedded world is to use something called a hardware abstraction library, and the way that the embedded rust implemented this

76
00:12:59.300 --> 00:13:18.490
maor malka: is by taking a special file called an Svd file, which is a file that usually is provided by most microcontroller vendors, which contains every peripheral, the addresses, and all the fields and bit width, and everything that each part of the chip has.

77
00:13:18.750 --> 00:13:37.980
maor malka: and they created a translator which takes this file and converts it into rust code, or, more specifically, to something called a peripheral access crate. This creates all the necessary templated rust code to access each and every one of the registers and fields in the chip.

78
00:13:38.980 --> 00:14:07.040
maor malka: And what we do above that is that we can create something called a Hal crate. So a Hal Crate allows us to. Instead of using, for example, to read the sample from the ABC. Where we'll have to write all these like, I don't know 1516 registers just to configure the ABC. We can create a function which allows us to initialize the ABC. And underline it uses the peripheral access crate

79
00:14:07.370 --> 00:14:08.930
maor malka: to do all the code.

80
00:14:09.490 --> 00:14:34.589
maor malka: One more above that is something called a board crate. So, for example, if we're using a specific PCB which has, let's say, an led connected to PIN 8 of the Microcontroller, you can describe that as an abstraction which says, this is the red led on the board, and when you control that you underline, it goes and controls PIN number 8 and blinks it.

81
00:14:35.329 --> 00:14:43.529
maor malka: Obviously, this is this conversion? Part of the Svd. Which saved a lot of time is obviously written in python.

82
00:14:44.910 --> 00:15:04.499
maor malka: So let's talk about the use case I was trying to make. So it seems like embedded rust is going to be very interesting. So I wanted to see if I can get the 3 core values implemented in an actual embedded project. So I wanted 1st of all to see that reuse.

83
00:15:04.860 --> 00:15:13.560
maor malka: which means that I can actually use cargo to save me from writing a lot of code blocks which, again, working in the embedded sea world.

84
00:15:13.830 --> 00:15:22.580
maor malka: you find yourself copy and pasting because there is no actual ideal method of sharing code one second.

85
00:15:24.260 --> 00:15:25.960
maor malka: The second thing is size.

86
00:15:26.270 --> 00:15:33.180
maor malka: Obviously, we're using a much higher level language than C. But it was important to me to see that

87
00:15:33.430 --> 00:15:38.520
maor malka: even in this case it can fit in small, in small microcontrollers.

88
00:15:38.760 --> 00:15:41.789
maor malka: And the 3rd thing is safety. I want to show that

89
00:15:41.900 --> 00:15:45.750
maor malka: the rust compiler helped me avoid any funny business.

90
00:15:46.980 --> 00:15:48.320
maor malka: So what's the task?

91
00:15:48.540 --> 00:15:52.449
maor malka: I'm trying to implement a web server on an Stm. 32.

92
00:15:52.980 --> 00:16:19.370
maor malka: Why do I want to do that? Usually, when people create these embedded projects, they provide with the project these archaic Guis, they wrote in a horrible language called C. Sharp, which uses this proprietary protocol to communicate with their product, and that's the only way to work with it. And if you have the wrong version, it breaks. And

93
00:16:19.580 --> 00:16:31.400
maor malka: and obviously, if you don't have the right.net drivers. It breaks so it seems like the most viable ideas. Why not provide the Gui with the product?

94
00:16:32.460 --> 00:16:33.550
maor malka: So

95
00:16:33.870 --> 00:16:43.639
maor malka: let's try to do that. The main issue is, though, that the Microcontroller, which you can see on the right on my finger has no networking hardware.

96
00:16:43.920 --> 00:16:50.649
maor malka: It has no file system support, and it only has 128 kB of flash and 40 kB of RAM

97
00:16:52.020 --> 00:16:57.170
maor malka: to top it all off, because this microcontroller is this small

98
00:16:57.654 --> 00:17:01.350
maor malka: what you usually do is you have to use something called no standard.

99
00:17:01.530 --> 00:17:07.260
maor malka: So rust provides a lot of a lot of utilities in its standard crates.

100
00:17:07.770 --> 00:17:20.219
maor malka: and because these are usually rely on some other underlying like requirements of the operating system, and sometimes can take a lot of space, we can't use that.

101
00:17:20.609 --> 00:17:32.229
maor malka: So this strips a lot of basic things, such as even using panic or using vec, we can't use it without actually providing some additional resources that we need to add to our code.

102
00:17:32.740 --> 00:17:40.489
maor malka: The biggest implication, though, is any crate that we want to use also has to support no standard mode.

103
00:17:42.000 --> 00:17:43.729
maor malka: So how are you going to do this?

104
00:17:45.420 --> 00:17:54.869
maor malka: we're going to use the USB interface of the Microcontroller to essentially tell the computer that this is a USB to Ethernet

105
00:17:55.000 --> 00:18:01.880
maor malka: dongle, using something in the USB standard, which is called a Cvc Ncm.

106
00:18:02.800 --> 00:18:18.940
maor malka: After the computer thinks that the connected device is an Ethernet device, we will add to the Microcontroller, Tcp IP stack, which will let us send basic protocol messages to the USB interface.

107
00:18:19.290 --> 00:18:39.260
maor malka: And finally, above that, we will create 2 sockets in the Microcontroller. One is an Http socket. So we can serve the website and handle, get and post requests from the client, and we'll also create a Dhcp socket which will allow the microcontroller to give the computer an IP address

108
00:18:41.590 --> 00:18:48.850
maor malka: kind of like this. So each one of these gray gray cubes is

109
00:18:49.280 --> 00:18:57.189
maor malka: is a crate that I ended up using. And the blue ones are things I had to write.

110
00:18:58.180 --> 00:19:07.180
maor malka: So, for example, you can see here things such as the hell as we talked about a moment ago, and.

111
00:19:07.470 --> 00:19:13.269
Gabor Szabo: Just sorry. Just a second. There was a question, where can you do? Can you do Hipaa location? Noah City.

112
00:19:13.830 --> 00:19:14.760
maor malka: One second

113
00:19:15.230 --> 00:19:21.510
maor malka: about Hua location. Yes, we actually have a slide about it. So once I get there, I'll talk about it.

114
00:19:22.260 --> 00:19:30.548
maor malka: And yes, okay? So as you can see, a lot of the

115
00:19:31.030 --> 00:19:48.580
maor malka: The the right side which talks about the actual peripherals of the Microcontroller were luckily standing on the shoulder of giants that have did the effort to make all these crates possible. And of course, the Tcp IP stack.

116
00:19:48.740 --> 00:19:53.489
maor malka: using something called small Tcp, which is a no standard. Tcp, IP stack.

117
00:19:54.150 --> 00:19:57.340
maor malka: So how does this actual interface looks like?

118
00:19:58.590 --> 00:20:02.960
maor malka: 1st off, we need to write the the Cvc Mcm. Driver.

119
00:20:03.080 --> 00:20:32.910
maor malka: So this is something you take from the USB spec about how you're supposed to provide the computer. The information of this is a Cdc Ncm device. What's its Mac address, what connections does it have? So that is the 1st phase. Afterwards. You will now start getting a data stream coming from the USB. Port. You need to write an Api handler to handle the traffic which we want to convert to Ethernet packets and vice versa.

120
00:20:33.590 --> 00:20:53.590
maor malka: and the packets come in small chunks, because this is a USB 1.1 device. It only provides 64 Byte each time. So we need to essentially buffer and slowly parse the messages. And this luckily was done, using a concurrent queue which let us essentially buffer a set of messages and only handle them later

121
00:20:54.250 --> 00:20:55.100
maor malka: and

122
00:20:55.290 --> 00:21:20.750
maor malka: on the Tcp. IP. Side. Small Tcp appsects weigh the idea of an interface to providing what it calls Tx and Rx tokens. So you need to provide an implementation for handling. When the small Tcp. Requests a Tx token, or wants to consume an Rx token, you need to provide that interface.

123
00:21:21.150 --> 00:21:34.750
maor malka: The way I did it is by creating another concurrent queue, which means that the the small Tcp can deposit or pop prepared messages for him to for it to parse

124
00:21:34.920 --> 00:21:36.940
maor malka: or send away, and

125
00:21:37.180 --> 00:21:44.649
maor malka: I can handle them in a store place, because again, the the USB side, I can only do it in small chunks of 64 Byte.

126
00:21:48.800 --> 00:21:57.360
maor malka: So how does this environment look like? What? What do we need to actually compile bare metal code for for using rust?

127
00:21:57.550 --> 00:22:00.340
maor malka: So 1st off, we need a cross compiler, tool chain

128
00:22:00.560 --> 00:22:13.080
maor malka: and rust. Does this amazingly. Using rustop, you just add a different target. This specifically for this case, it's a, it's an arm target called a fun. V, 6. I think.

129
00:22:14.530 --> 00:22:23.270
maor malka: or v, 7. The point is that it's a lot simpler than what you usually need to do in C to implement a cross compilation.

130
00:22:23.620 --> 00:22:27.749
maor malka: We need to provide runtime, which

131
00:22:28.340 --> 00:22:36.600
maor malka: if you didn't know when you, for example, define a variable in C or in rust which has some initial value. Let's say 7.

132
00:22:36.720 --> 00:22:49.580
maor malka: Some code has to run before your main code to actually prepare that variable, to actually be the value you're expecting it to be. That code is called the Runtime

133
00:22:49.960 --> 00:22:51.100
maor malka: and

134
00:22:51.380 --> 00:23:04.550
maor malka: the rust embedded group have also provided some form of a minimal runtime that you need for the Microcontroller to ensure that everything around your code that's running is prepared properly.

135
00:23:05.460 --> 00:23:21.509
maor malka: You also need a file called Memory X, which is Rust's way of creating a linker script. Using that file, you essentially define the address spaces where your flash and RAM live. You can also use that which I used

136
00:23:21.710 --> 00:23:25.079
maor malka: to define where you want to do heap allocations.

137
00:23:25.780 --> 00:23:28.799
maor malka: The next thing you need is a file called Buildrs.

138
00:23:29.240 --> 00:23:33.299
maor malka: which lets us force the rust compiler to use the Linker script we created.

139
00:23:35.060 --> 00:23:38.130
maor malka: And finally, after all that we have our main Rs.

140
00:23:38.420 --> 00:23:46.840
maor malka: Which contains which should contain at least 4 things. The 1st one is obviously our function main, which would be our entry point to the code

141
00:23:47.580 --> 00:23:49.120
maor malka: we would want to create

142
00:23:49.220 --> 00:23:58.429
maor malka: or or import a panic handler. So once any part of the code calls panic, it will jump to that specific piece of code.

143
00:23:59.230 --> 00:24:11.809
maor malka: We also want to define a global allocator. So anytime, we want to use something that uses the heap, for example, creating vex. Using most of the news that we want to use, we will need to provide a global allocator

144
00:24:12.150 --> 00:24:19.249
maor malka: which will allocate memory in the heap of a certain location that we define.

145
00:24:19.650 --> 00:24:31.080
maor malka: And the final thing we need to define is something called the hardware exception handlers. So when an interrupt occurs in the chip. For example, if you divide by 0,

146
00:24:31.380 --> 00:24:42.999
maor malka: you need to tell the Microcontroller a form of an emergency jump. It needs to do to specific location of code that you want to handle that situation.

147
00:24:43.230 --> 00:24:45.140
maor malka: So you need to also provide those.

148
00:24:46.110 --> 00:24:51.190
maor malka: And after we provided all that we can start working on our on our project.

149
00:24:51.820 --> 00:24:53.250
maor malka: So what about debugging?

150
00:24:53.500 --> 00:25:10.480
maor malka: So debugging is super easy. If you have the right tools. Rust luckily supports Gdb out of the box, which means we can use, for example, something called Openocd, which is a piece of software that lets us take any type of debugger

151
00:25:10.610 --> 00:25:17.020
maor malka: and use it to expose a Gdb server that we can connect you, for example, using Vs code.

152
00:25:19.190 --> 00:25:25.890
maor malka: That allows us to do step-by-step debugging on the Microcontroller while running rust and

153
00:25:26.300 --> 00:25:40.159
maor malka: on top of that in this specific case, because the Microcontroller is a modern arm chip. It also has something called Rtt. Which lets us to the Jtac port. Essentially send any arbitrary data that we want.

154
00:25:40.420 --> 00:25:51.749
maor malka: and a company called Fair System created a crate called Deformat, which lets you send out logs and panics and all this information throughout to the Jtac port

155
00:25:51.920 --> 00:25:54.640
maor malka: without requiring any extra space.

156
00:25:54.960 --> 00:25:56.200
maor malka: And

157
00:25:56.540 --> 00:26:19.809
maor malka: so in this case, because I was essentially enumerating USB, a lot connecting the Microcontroller to my windows. PC. Would make it crash quite quickly. So I set aside a raspberry pi which has the Microcontroller, which you can see down here connected to it, and Jtag, debugger by St. Connected to its debug pins.

158
00:26:20.190 --> 00:26:27.539
maor malka: And all this remote because this is Gdb. Gdb. Exposes a remote connection server, which means

159
00:26:27.770 --> 00:26:50.759
maor malka: the raspberry pi can be located somewhere further, and I can just easily, by clicking debug, actually debug remotely on the raspberry pi. The Microcontroller code. And on top of that we also have the deformat which lets me just see structured messages, including their location in code where they're printed from.

160
00:26:53.198 --> 00:26:55.839
maor malka: Okay, so what about safety?

161
00:26:57.710 --> 00:27:22.889
maor malka: so we started with talking about global variables, global variables are coming as sand in embedded projects. They're literally one of the most common things you see in almost every embedded project. And because we're using rust, and we want to change or read these variables across other pieces of our code, we're forced to use a mutex. We can't do it otherwise the rust compiler will stop you.

162
00:27:23.910 --> 00:27:42.910
maor malka: The second thing is concurrency. So in the same aspect. If we want to transfer data between pieces of code, we will have to make sure that all these shared data cannot be cannot be written overwritten while we're working on it.

163
00:27:42.990 --> 00:27:57.899
maor malka: So the only way to actually implement it is by using some form of a concurrency tool which, for example, could have been a mutex, but in this case, using the concurrent queue, helped also alleviate the fact that the data rate between

164
00:27:58.020 --> 00:28:00.950
maor malka: certain pieces of the code is not equal.

165
00:28:01.360 --> 00:28:13.339
maor malka: and the nice thing about it is because the concurrency is forced on you, even if you use an interrupt, driven system which you can think about as multiple threads that are unscheduled.

166
00:28:14.060 --> 00:28:17.960
maor malka: you can still use the same concurrent ideas, and it will still be safe.

167
00:28:19.200 --> 00:28:32.480
maor malka: And the 3rd thing is obviously compilation and panic gotchas. So obviously, when you write a parser to an Api, you won't necessarily get it right the 1st time, especially when it comes to packed data.

168
00:28:32.850 --> 00:28:51.100
maor malka: So the rust compiler caught me in a lot of times where I would either miscalculate things and cause an overflow without overflow for a certain field, or I would miscalculate the Api sizes which also caused panics or compilation errors.

169
00:28:52.210 --> 00:28:56.909
maor malka: Okay, I think I dig enough. Let's let's look at a demo.

170
00:28:59.610 --> 00:29:04.860
maor malka: So I'm gonna switch my obs to

171
00:29:05.080 --> 00:29:08.909
maor malka: this. Everybody can see. See me and the

172
00:29:09.220 --> 00:29:12.020
maor malka: and the little chrome tab that's running

173
00:29:15.310 --> 00:29:16.010
maor malka: double.

174
00:29:16.010 --> 00:29:18.598
Gabor Szabo: Yeah, I can. Yeah, yeah. Sorry.

175
00:29:19.730 --> 00:29:24.509
Gabor Szabo: you can switch to Speaker, view, everyone can switch to. In the view section of.

176
00:29:24.510 --> 00:29:28.660
maor malka: Okay, I'll hit that like this spotlight for everyone. Now, can everyone see me.

177
00:29:29.420 --> 00:29:33.019
Gabor Szabo: Yeah, no, yeah. That's what I should have done. Yeah.

178
00:29:33.020 --> 00:29:35.007
maor malka: Okay, perfect. So

179
00:29:36.220 --> 00:29:46.810
maor malka: what are we looking at right now? So on on our little chrome tab, we can see the website that was served by the Microcontroller.

180
00:29:46.920 --> 00:29:58.880
maor malka: It's right here on the bottom. You can see it right here. The Microcontroller served the website, and of course it oh, there we go. It's still alive, thank God!

181
00:29:59.860 --> 00:30:02.569
maor malka: And it it served the website.

182
00:30:02.750 --> 00:30:10.409
maor malka: And in the website there's also Javascript code that constantly pulls the Microcontroller with get requests.

183
00:30:10.530 --> 00:30:16.390
maor malka: using that it pulls out the amount of loops per second, the main.

184
00:30:16.700 --> 00:30:24.590
maor malka: while loop has done, and also the Adc. Is running and presenting us. The current temperature of the Microcontroller. So if I

185
00:30:24.870 --> 00:30:30.979
maor malka: if I touch and apply pressure on the chip a bit, it should let the temperature rise up slightly.

186
00:30:32.380 --> 00:30:44.059
maor malka: and we can also do it the other way, using post requests. So, for example, there's a little screen here which I'm realizing you're not seeing. But I'm now picking a different color. And hopefully.

187
00:30:44.600 --> 00:30:53.859
maor malka: we can see on the screen. Of course it doesn't work that I didn't. I didn't praise the demo gods, so we're gonna give it another shot.

188
00:31:02.010 --> 00:31:02.990
maor malka: And

189
00:31:06.110 --> 00:31:10.255
maor malka: while the devil gods do not like me today, well,

190
00:31:11.510 --> 00:31:22.560
maor malka: yeah. So essentially, this section of the of the RGB should have controlled the RGB. Led on the board. I'm guessing it might be because of chrome. Not like me today.

191
00:31:23.220 --> 00:31:25.770
maor malka: But trust me, it works

192
00:31:31.210 --> 00:31:33.625
maor malka: one second. Let's go back to the

193
00:31:47.480 --> 00:31:48.320
maor malka: Okay.

194
00:31:49.450 --> 00:31:54.830
maor malka: One important note. I wanted to say about the this. This project.

195
00:31:56.190 --> 00:32:02.879
maor malka: This ended up being 93 K. Of flash used to make this project

196
00:32:03.020 --> 00:32:10.080
maor malka: where 6 kB was the actual website. One nice trick that I did was by

197
00:32:11.850 --> 00:32:30.440
maor malka: by minifying and then gzipping the website. I don't know if you know, but you can. You can send on the wire a gzipped website like an HTML file, and the browser itself will unzip it and then display it for the client.

198
00:32:30.710 --> 00:32:38.299
maor malka: which means you can really make a really really small website quite compact and still provide it.

199
00:32:38.480 --> 00:32:53.410
maor malka: And I ended up using 16 kB of RAM for the heap to do all the message parsing and creating the HTML text that you need to serve well, the the get and post requests mostly handling them.

200
00:32:57.800 --> 00:33:21.090
maor malka: obviously having a project that is pure rust is great and all. When you make a little hobbyist project, or you're trying to make something from scratch. But life tends to be a bit more complicated than that, and a lot of times we would like to add just a little bit of rust to our C code.

201
00:33:21.290 --> 00:33:34.340
maor malka: But luckily Rust supported that from day one that was one of the original things they they did when they created the language. So rust inherently supports using a foreign function interface

202
00:33:34.530 --> 00:33:40.219
maor malka: to essentially call or be called by C code, for example.

203
00:33:41.390 --> 00:34:01.360
maor malka: So that means that we can use our code to call other code that was written in C, or we can have some C base and have only the safe and optimized part or the newer parts of our code that we want to use in this big project written in rust.

204
00:34:03.020 --> 00:34:05.749
maor malka: One perfect example for that is.

205
00:34:05.950 --> 00:34:11.380
maor malka: on the Esp. 32. We have a crate called the Esp. Idfsbc.

206
00:34:13.590 --> 00:34:17.650
maor malka: So esp provides something they call their idf.

207
00:34:18.010 --> 00:34:25.566
maor malka: which is a which is a framework which contains which contains

208
00:34:27.310 --> 00:34:45.389
maor malka: free rtos running with multiple drivers to control many of the actual parts of the chip, for example, controlling the Wi-fi, controlling the Bluetooth, controlling all the peripherals, handling, networking encryption, file operations.

209
00:34:45.610 --> 00:34:52.599
maor malka: and you write your code above it as one of the threads of the Free Rtos.

210
00:34:52.909 --> 00:35:06.209
maor malka: So what the people that wrote the Esp. Idf. Crate, as Esp. Idf. Svc. Crate did, was essentially treat your rust code as one of those threads which means you can treat. You can just write standard rust

211
00:35:06.820 --> 00:35:09.230
maor malka: and even Async rust if you want.

212
00:35:09.680 --> 00:35:21.880
maor malka: and it will be embedded into this project, and you have just created a safe part of the code running inside this massive framework that's already been written.

213
00:35:22.870 --> 00:35:41.699
maor malka: So this framework obviously contains all the Hal drivers that we talked about. It also contains higher level setup functions. So, for example, you can ask the Wi-fi to scan, which would require quite a lot of code to write. But for you it's just one function you call in rust.

214
00:35:42.890 --> 00:35:43.950
maor malka: And

215
00:35:44.190 --> 00:36:02.499
maor malka: to actually show that I created another demo which I call the Clickup task e-paper display. I like to use clickup, which is like Monday, and all these other to do to do apps to manage all my the personal tasks I need to do, and

216
00:36:02.990 --> 00:36:11.359
maor malka: what I what I wanted to do was have a little e-paper screen next to my desk, which shows me my current tasks.

217
00:36:11.650 --> 00:36:23.910
maor malka: So the flow of the project is that the Microcontroller connects to my home. Wi-fi uses https to perform a get request on the clickup, Api, and get all my tasks.

218
00:36:24.030 --> 00:36:27.910
maor malka: It then parses that information uses Saturday. Json.

219
00:36:28.030 --> 00:36:43.749
maor malka: because this is what comes back is Json. It validates which of the tasks are late, and it adds a small indicator about it. We then use a hashing function to check if any of the data changed from the last time we checked it.

220
00:36:44.030 --> 00:36:51.890
maor malka: and if it did, we create a new display layout on the epaper display, and we send that that frame buffer to the Epaper display.

221
00:36:52.850 --> 00:37:06.070
maor malka: This project is again is only maybe 6 or 7 tasks that we need to do, but in that we relied on so much code from the est Idf. Crates and from the rust side as well.

222
00:37:06.070 --> 00:37:29.039
maor malka: For example, we'd had to use the Wi-fi driver. We had to have the Esp Idf provide certificates so we can do https properly. We had to use networking to do all the protocols we had to use. Http. We had to use Smtp to get the actual real time right now a Dhcp client to actually provide an IP address for the Esp. 42

223
00:37:29.070 --> 00:37:30.430
maor malka: spi to

224
00:37:31.390 --> 00:37:38.649
maor malka: to control the screen and to hold the previous hash. We also needed to use the non-volatile storage.

225
00:37:38.980 --> 00:37:45.189
maor malka: And on the rust side, obviously, there's more crates as well. So we had to use vex which are part of the standard rust

226
00:37:46.940 --> 00:37:57.419
maor malka: and we also use survey. And we used some tools to create the graphics which is embedded graphics and a driver which controls the the Epaper display.

227
00:37:59.220 --> 00:38:08.540
maor malka: and I can, even if I'll do a quick one of these, and I'll do spotlight

228
00:38:08.740 --> 00:38:10.667
maor malka: so we can see.

229
00:38:11.750 --> 00:38:21.070
maor malka: all we needed is one microcontroller which is one here and obviously the the display adapter board. And it just works

230
00:38:26.810 --> 00:38:27.880
maor malka: one second.

231
00:38:31.910 --> 00:38:37.030
maor malka: Okay, we had fun. Let's talk about conclusions so

232
00:38:37.500 --> 00:38:52.170
maor malka: great. We have made projects which are safe. They fit in microcontrollers. They're as fast as we want, really just 0 cost abstractions. Everything is great. But the question we wanted to ask was, Are we embedded?

233
00:38:52.690 --> 00:38:53.550
maor malka: So

234
00:38:53.750 --> 00:39:00.480
maor malka: we need to talk about the small issues that we currently have in the industry, especially when it comes to embedded projects.

235
00:39:00.650 --> 00:39:19.899
maor malka: So the 1st one is that embedded engineers are a really tough knot. Usually they have so much experience and insanity writing really archaic C code, or even just like hand making assembly to handle all these extreme cases in their microcontrollers, that they really really

236
00:39:20.100 --> 00:39:22.410
maor malka: rarely will even think about trying

237
00:39:22.730 --> 00:39:25.169
maor malka: trying to use a different language than C

238
00:39:25.330 --> 00:39:28.210
maor malka: or C plus plus if they use embedded Linux.

239
00:39:29.350 --> 00:39:35.010
maor malka: We also have that rolls us to the second problem, which is the customer vendor problem

240
00:39:35.120 --> 00:39:41.310
maor malka: as long as the customers don't ask. I want to do this project on your microcontroller in rust.

241
00:39:41.710 --> 00:40:03.109
maor malka: All the support and tool chain will stay in the open source side, and I've been constantly checking a lot of vendors, for example, St. Which we used here, Mxp. Renaissance microchip, which all these companies are some of the biggest companies in the world. When it comes to microcontrollers that we can find everywhere. None of them support. Rust.

242
00:40:04.185 --> 00:40:05.080
maor malka: They

243
00:40:05.190 --> 00:40:09.490
maor malka: and some of them even tell you, why do you do this? You don't want to do this.

244
00:40:09.710 --> 00:40:26.549
maor malka: so there is quite a lot of Pushback. The only company right now that's supporting the community when it comes to rust on embedded project is expressive with the Esp. 32, which provides examples and support for the actual rust code.

245
00:40:27.790 --> 00:40:49.960
maor malka: The 3rd thing is that rust did not invent the word safe code. The idea of having a safe code has been around for quite a couple of decades and has been standardized in multiple technical standards for the automotive, medical, industrial, aerospace military world. Each one of them has their own standard.

246
00:40:50.090 --> 00:41:13.699
maor malka: I am happy to say that Ferosystem, the company that we talked about earlier have created their own version of the rust compiler called Ferrocene, and this compiler can actually comply with the standards. So if you write your rust code and use that specific compiler, you can get approval for advanced

247
00:41:14.050 --> 00:41:31.550
maor malka: safety standards that are provided in other industries. The 2 things that are currently unsupported by this compiler are the aerospace industry, so you can't send yet a missile written in rust, but maybe hopefully soon, and and military applications.

248
00:41:32.460 --> 00:41:50.490
maor malka: And the final thing is the Rtos thing. So I vaguely mentioned Rtos in the start. And when we were talking about Esp. But rtoses are quite common in the embedded world, and they're so common that they have a lot of age in them. So, for example, free Arthos, which is quite

249
00:41:50.600 --> 00:41:53.379
maor malka: used a lot, is 21 years old.

250
00:41:53.600 --> 00:42:12.220
maor malka: Fredx, which has been used quite a lot. It's been bought by Microsoft and now called Azure Rtos. It's been around for 27 years, and we also have something called Microcos, which has been used to land rockets on the moon, and it's also been around for 33 years.

251
00:42:12.500 --> 00:42:30.449
maor malka: but on on the Rtos, on the rust side we do have 2 projects which I know of at least that are pretty big and, like pretty, have a good amount of code, base and followers. The 1st one is called Embassy, which tries to implement Async rust

252
00:42:30.550 --> 00:42:50.930
maor malka: in microcontrollers with a single, again, with a single single core. Essentially it works pretty well, and it has all the supports, but obviously it. And the next one has the same issue. It's they're still very, very young. So it's kind of hard for people to trust

253
00:42:51.110 --> 00:42:53.220
maor malka: that these

254
00:42:53.420 --> 00:43:02.059
maor malka: artists are stable enough for actual hard applications. For example, we can take an extreme case and say a pacemaker.

255
00:43:02.330 --> 00:43:06.499
maor malka: which you have to make sure that everything works to the point.

256
00:43:07.010 --> 00:43:15.530
maor malka: and we also have Arctic, which is a really cool example where they use interrupts as threads.

257
00:43:15.840 --> 00:43:18.579
maor malka: and it is another concept of an Rtos.

258
00:43:18.790 --> 00:43:30.710
maor malka: But the end of the day, till we won't have enough years and experience of of using these artists, it will still be a challenge to treat them as good as the C equivalents.

259
00:43:32.070 --> 00:43:37.070
maor malka: That's it. Thank you so much, and if you have any questions, you're more than welcome to stay.

260
00:43:42.180 --> 00:43:45.740
Gabor Szabo: 1st of all, thank you. Let me just check it.

261
00:43:46.320 --> 00:43:48.720
Gabor Szabo: How did you turn on this?

262
00:43:52.470 --> 00:43:57.319
Gabor Szabo: So the spotlight? How did you turn on the spotlight.

263
00:43:57.900 --> 00:43:58.260
maor malka: Huh!

264
00:43:59.400 --> 00:44:06.039
maor malka: Right here, you right click your. I believe you right. Click your face and do spotlight for everyone.

265
00:44:08.330 --> 00:44:09.530
Gabor Szabo: And.

266
00:44:09.640 --> 00:44:10.410
maor malka: Excuse me.

267
00:44:13.910 --> 00:44:31.799
Gabor Szabo: Okay, I can't find it now. But okay, I'll find it. Okay, anyway. Thank you very much for this talk. If anyone has questions or would like to ask. Now, please go ahead now, still in the chat. Otherwise we are going to finish the video, and then you can stay around and have a chat without

268
00:44:31.940 --> 00:44:40.300
Gabor Szabo: without needing to type. But it seems that people had enough questions already, and they are saying, Thank you. So thank you too.

269
00:44:40.550 --> 00:44:46.439
Gabor Szabo: So, Mahour, thank you very much for this presentation, and hopefully the.

270
00:44:46.440 --> 00:44:47.890
maor malka: Thank you for having me.

271
00:44:48.280 --> 00:45:09.170
Gabor Szabo: Hopefully, the video will be I. It's a pleasure, really. We already had a couple of these presentations and others, and really, really like it, and I hope that you'll give us more, more presentations like this and even more detailed ones. And anyone actually watching and listening you would be welcome to offer your presentation.

272
00:45:09.340 --> 00:45:21.950
Gabor Szabo: Talk to me and those who, watching the video. Thank you and please like the video and follow the channel and see you next time. Bye-bye.

