---
title: "No Python was harmed: teaching a tiny MCU to learn as it goes"
timestamp: 2026-10-27T07:30:01
author:
published: true
description:
tags:
---

## Description

An embedded session showing how to deploy a useful hand-written neural network made with Rust supporting on-chip training on a tiny MCU.

## Bio

[Leon Vak](https://www.linkedin.com/in/leon-vak-24445922/)

As CTO of Embedded at abra R&D Solutions, I lead the technical architecture and engineering for highly customized, non-standard client projects. My daily work spans hands-on board bring-ups, custom silicon (ARM, RISC-V, ARC), Embedded Linux BSPs (specializing in Yocto project), and RTOS platforms. I work across the full software stack: from complex bare-metal layers and kernel drivers up to higher-level applications, everything primarily in C, C++, and Rust. Alongside running R&D POCs and solving low-level integration issues, I prepare and conduct technical trainings and meetups, while mentoring our development teams and representing the company at presales meetings.


## Links

* [microbit_learns](https://github.com/walruscraft/microbit_learns)


## Length

60 min


{% youtube id="9e2Q59SO7JI" file="2026-10-26-no-python-was-harmed-teaching-a-tiny-mcu-to-learn-as-it-goes-by-leon-vak.mp4" %}


## Transcript

1
00:00:01.990 --> 00:00:15.960
Gabor Szabo: Okay, so welcome and… hello, hello and welcome to this meeting and this presentation. Leon, thank you very much for agreeing to give this presentation, and welcome.

2
00:00:15.960 --> 00:00:35.700
Gabor Szabo: Welcome everyone who joined us here. It's really nice to have you, and hello everyone who is watching this video. If you are watching this, then please also like the video and follow the channel, and below the video, we will find links to information about Leon and about the presentation.

3
00:00:35.830 --> 00:00:38.529
Gabor Szabo: And, a link to the future events.

4
00:00:40.870 --> 00:00:49.220
Gabor Szabo: People who are here, you have the privilege of asking questions, so next time, those who are watching now, you might want to join us, so you can ask questions.

5
00:00:49.220 --> 00:01:03.089
Gabor Szabo: during the presentation, and hopefully no one will be able to answer them either during the presentation or after. And I also wanted to remind you that even though we had half an hour to introduce ourselves before and have some chat.

6
00:01:03.290 --> 00:01:22.389
Gabor Szabo: even after the presentation and the video is over, the video recording is over, we will have time to stay around and keep chatting and talking about everything in this topic. So that's it for me. Thank you very much for joining us, and the stage is yours.

7
00:01:23.310 --> 00:01:28.640
Leon: Alright, so let's start with enabling the sharing.

8
00:01:29.180 --> 00:01:31.530
Leon: This is my display.

9
00:01:32.960 --> 00:01:33.840
Leon: Totally.

10
00:01:35.990 --> 00:01:41.959
Leon: So, hi everyone, my name is Leon, and…

11
00:01:42.280 --> 00:01:50.910
Leon: I do embedded stuff, and we are talking about machine learning on NMC today.

12
00:01:51.830 --> 00:02:04.589
Leon: So… I suppose some of you know machine learning from, at least from studies, if not from work.

13
00:02:06.370 --> 00:02:13.159
Leon: And when I studied machine learning in the university.

14
00:02:13.730 --> 00:02:20.890
Leon: You know, all the stuff, gradient descent, backpropagation and stuff, Now… wait.

15
00:02:21.930 --> 00:02:24.479
Leon: Do we do this on an MCU?

16
00:02:25.540 --> 00:02:29.349
Leon: Have you ever seen this done on MCU?

17
00:02:30.860 --> 00:02:36.760
Leon: Well, I do see lots of customers,

18
00:02:36.950 --> 00:02:46.129
Leon: Every year, and I personally haven't seen anyone doing anything like this, and that's what gave me the idea for this talk.

19
00:02:47.270 --> 00:02:48.920
Leon: So, rock.

20
00:02:50.050 --> 00:02:56.349
Leon: machine learning on an MCU. Yes, the training Only MC.

21
00:02:57.230 --> 00:03:03.600
Leon: First of all, You know, the first clear question is, why would we do anything like that?

22
00:03:08.840 --> 00:03:16.480
Leon: Before we begin, what we're talking about. This is a demo of how

23
00:03:16.770 --> 00:03:19.489
Leon: Useful a miniature controller can be.

24
00:03:19.690 --> 00:03:31.050
Leon: And this is a proof that machine learning can be, and actually should be, performed on MCU for a bunch of useful reasons.

25
00:03:31.350 --> 00:03:43.690
Leon: And we're doing it in Rust, because we don't need any excuse for that, you know? We are not, like, 10 years ago, and it's alright today in Embedded to use Rust for

26
00:03:44.190 --> 00:03:45.229
Leon: Any project.

27
00:03:46.510 --> 00:03:51.380
Leon: What is… what we will not be doing today, we're not learning Rust.

28
00:03:51.500 --> 00:03:59.600
Leon: We are not talking about a very specific MCU as a promotion, because

29
00:04:00.180 --> 00:04:06.740
Leon: This line is here because I have used this specific MCU in

30
00:04:07.230 --> 00:04:15.599
Leon: several different meetups, and I do like it. It's a nice thingy, but we will delve to this soon.

31
00:04:15.670 --> 00:04:27.940
Leon: And we are not going to do a deep diving into code, because this is not really practical on such a screen and monitoring, you know, in a video.

32
00:04:28.190 --> 00:04:35.879
Leon: and whatever, but I've got a ripple for you, you'll tear it up, and break it, and play with it later.

33
00:04:36.550 --> 00:04:37.810
Leon: So, right.

34
00:04:38.060 --> 00:04:40.620
Leon: Why would we want to teach?

35
00:04:40.990 --> 00:04:42.070
Leon: something.

36
00:04:42.570 --> 00:04:43.620
Leon: On the controller.

37
00:04:44.250 --> 00:04:49.969
Leon: Well, here are a few use cases for you. Maybe I will convince you from the start.

38
00:04:51.390 --> 00:04:53.299
Leon: Consider assistive input.

39
00:04:54.430 --> 00:04:56.670
Leon: Like, disabled controls.

40
00:04:57.020 --> 00:05:05.820
Leon: For example, you know, you have seen, people, using disabled controls, like,

41
00:05:06.070 --> 00:05:12.790
Leon: Various, remote controls and stuff disabled people use daily.

42
00:05:13.190 --> 00:05:14.070
Leon: Emm…

43
00:05:15.180 --> 00:05:23.579
Leon: they are all, you know, like, people are all different, disabled people are also all different, and the disabilities are different.

44
00:05:23.800 --> 00:05:30.639
Leon: And if I need to, like, move a controller.

45
00:05:30.890 --> 00:05:37.530
Leon: Not every disabled person will be able to perform every action like another one.

46
00:05:38.130 --> 00:05:39.499
Leon: So why not?

47
00:05:39.730 --> 00:05:47.360
Leon: Train a controller, For this person, without making this a huge task Expensive task.

48
00:05:48.190 --> 00:05:57.080
Leon: Gaming precision, if there are any gamers here, imagine yourself, you're holding a controller, And…

49
00:05:57.550 --> 00:06:07.530
Leon: Exactly when you want to shoot at someone, Your action with the finger, Just slightly moves your controls.

50
00:06:07.700 --> 00:06:08.420
Leon: Right?

51
00:06:09.360 --> 00:06:15.460
Leon: You get used to it, you know, you get used to it, and then you're cooler, but still.

52
00:06:15.820 --> 00:06:19.730
Leon: You have this action that moves the controller slightly.

53
00:06:21.200 --> 00:06:25.160
Leon: Wouldn't you want to simply adapt for that?

54
00:06:25.630 --> 00:06:36.340
Leon: Like, increase your precision in gaming just by a very short procedure of several seconds to adapt for yourself, for your actions.

55
00:06:36.720 --> 00:06:37.730
Leon: Easily.

56
00:06:38.040 --> 00:06:49.850
Leon: Medical customization, it's about, you know, all the stuff that doctors use, same things as well. And this is all because not two people are alive.

57
00:06:50.780 --> 00:06:56.210
Leon: Another use case is the myoelectric prosthetics.

58
00:06:56.760 --> 00:07:03.400
Leon: I'm not sure that you are familiar with the thing where electrode placement

59
00:07:04.080 --> 00:07:07.359
Leon: Shifts a little bit every time the socket goes on.

60
00:07:08.160 --> 00:07:13.150
Leon: So every time, every morning when the person puts on the device.

61
00:07:14.400 --> 00:07:26.149
Leon: It has to be adjusted in the correct way for the, stuff to fire as needed to fill your,

62
00:07:26.260 --> 00:07:34.799
Leon: Your tiny electric currents in the nerves, and to move the way you want it to work.

63
00:07:34.940 --> 00:07:40.949
Leon: Because the prosthetics, they move because they feel current. They know nothing about you.

64
00:07:41.420 --> 00:07:45.330
Leon: What if you could really train it really easy?

65
00:07:46.270 --> 00:07:51.149
Leon: In a few seconds, instead of, you know, going crazy about

66
00:07:51.660 --> 00:07:54.419
Leon: Putting it in the right way.

67
00:07:56.150 --> 00:07:58.749
Leon: Another very different use case.

68
00:07:59.690 --> 00:08:03.730
Leon: Consider heavy machinery at factories and such.

69
00:08:04.280 --> 00:08:08.709
Leon: like… Is this shaking of the machine normal?

70
00:08:09.380 --> 00:08:14.119
Leon: or the press impact? Did it degrade over time?

71
00:08:14.490 --> 00:08:20.469
Leon: How do you know? Because, you know, the same machine installed in the different factories

72
00:08:21.260 --> 00:08:26.249
Leon: moves differently. The floors are different, the installation methods are different.

73
00:08:26.850 --> 00:08:37.169
Leon: And what if you could have an independent and redundant attachment to the machine that would monitor this specific installation?

74
00:08:37.330 --> 00:08:39.510
Leon: The way you expect to monitor it.

75
00:08:42.600 --> 00:08:48.350
Leon: Another case I thought would be interesting is agriculture sensor.

76
00:08:48.680 --> 00:08:53.069
Leon: So, like, the soil varies from place to place.

77
00:08:54.140 --> 00:08:58.510
Leon: And we want to monitor changes in the soil.

78
00:08:58.670 --> 00:09:00.670
Leon: So, what's the normal?

79
00:09:01.070 --> 00:09:09.820
Leon: Okay? Of course, you usually configure it and set it up in field, but what if you could do this very, very easily?

80
00:09:10.720 --> 00:09:14.600
Leon: What if you could simplify the deployment in field?

81
00:09:16.090 --> 00:09:25.010
Leon: And, actually, it's applicable to virtually any sensors you can think about, not just,

82
00:09:25.260 --> 00:09:29.639
Leon: Not just, some agricultural stuff. So…

83
00:09:30.380 --> 00:09:38.839
Leon: I'm not sure that everyone really understands what I'm talking about right now, but I promise you that by the end, you

84
00:09:41.690 --> 00:09:51.640
Leon: So, what's a common machine learning on an MCU? MCUs do run neural networks, and they do,

85
00:09:52.200 --> 00:09:53.930
Leon: Useful stuff for them.

86
00:09:54.230 --> 00:09:58.750
Leon: But usually, the progress is, like, you collect the samples.

87
00:09:58.960 --> 00:10:03.949
Leon: You know, you don't really collect the samples on the server, but you get the samples

88
00:10:04.060 --> 00:10:06.369
Leon: To the server, to train it.

89
00:10:06.710 --> 00:10:14.579
Leon: Train the neural network on the server, you quantize it, the result, to ints to save on space.

90
00:10:14.710 --> 00:10:18.520
Leon: Then you export it to a file, then you flash it to the MCU.

91
00:10:18.670 --> 00:10:23.429
Leon: The MCU does nothing with this neural network, just forward passes.

92
00:10:23.560 --> 00:10:32.190
Leon: All the time, and makes it, you know, makes it kind of useful, but clearly you cannot retrain it.

93
00:10:33.530 --> 00:10:38.300
Leon: In this demo, we are going to see a totally different picture.

94
00:10:38.440 --> 00:10:41.659
Leon: We're going to build the firmware on the server.

95
00:10:42.330 --> 00:10:54.150
Leon: And then the MCU will record the data, will train on it, and then we'll be in a working loop until I restart it, and it returns from the scourge.

96
00:10:57.200 --> 00:11:08.130
Leon: I will not get more into why this is useful. I hope that, minding the initial examples, we will be able to talk about it at the end.

97
00:11:10.370 --> 00:11:16.680
Leon: So, right, our goals are to showcase machine learning with training on an MCU,

98
00:11:16.960 --> 00:11:20.179
Leon: We will use my beloved microbeat.

99
00:11:21.110 --> 00:11:36.340
Leon: I've done so many things with it by now, and so many meetups, it's amazing. It's really a unique device, if you ask me. But it's still based on the standard Cortex-M4 CPU.

100
00:11:36.470 --> 00:11:41.690
Leon: at least the Microbit V2 that I'm using.

101
00:11:42.630 --> 00:11:53.730
Leon: It has half a meg of flash, 128K of RAM, and it's a 64MHz CPU.

102
00:11:55.750 --> 00:11:58.590
Leon: the code,

103
00:11:58.630 --> 00:12:17.060
Leon: that was prepared for this meetup is 29 kilobytes in size, eventually. Now, I'm speaking about a release version, not about a debug version, so it's the very strict one, of course.

104
00:12:17.210 --> 00:12:25.839
Leon: And the multi-layer perceptron, which is the… our neural network, is just 6 kilobytes.

105
00:12:25.970 --> 00:12:28.300
Leon: Of this, file.

106
00:12:29.140 --> 00:12:34.520
Leon: And in the run, we consume only 22 kilobytes of RAM.

107
00:12:35.650 --> 00:12:37.150
Leon: What does it mean?

108
00:12:37.410 --> 00:12:44.429
Leon: Well, you will decide, but if you ask me, it means that we're using something very minor.

109
00:12:44.720 --> 00:13:00.720
Leon: even based on the small microcontroller that we use, so we use a fraction of what it can offer us, so we can really improve this thing, but I didn't need much more to show you what we are going to see.

110
00:13:01.460 --> 00:13:09.040
Leon: So… you know, when I got the idea of training the,

111
00:13:09.480 --> 00:13:13.259
Leon: Training the neural network on the device.

112
00:13:13.510 --> 00:13:16.100
Leon: I thought what…

113
00:13:16.810 --> 00:13:34.019
Leon: what I can show you in usefulness of the device. And I've decided to connect it to one of my previous meetups about making this device, the microbit, control things on the PC.

114
00:13:35.570 --> 00:13:42.889
Leon: So, we are going to see training of this device through the accelerometer.

115
00:13:43.270 --> 00:13:45.779
Leon: that I have on the board.

116
00:13:46.830 --> 00:13:50.140
Leon: To four different, tasks.

117
00:13:50.410 --> 00:13:59.770
Leon: Reset, right, left, and shake, where right and left will become actual buttons, the, computer.

118
00:14:01.290 --> 00:14:06.550
Leon: And so… This is a classification task, let's use machine learning.

119
00:14:07.140 --> 00:14:13.480
Leon: By the way, I said that we're going to use right and left buttons, but… Why the shake, then?

120
00:14:13.970 --> 00:14:19.240
Leon: Well, I'm just not used to have three

121
00:14:20.100 --> 00:14:29.409
Leon: positions in anything, right? We work with power of twos, so it will recognize the shape as well, but the shape will not do anything useful for us.

122
00:14:29.540 --> 00:14:31.619
Leon: And I also like the graphs of it.

123
00:14:33.230 --> 00:14:39.889
Leon: So, what do we have on the MLP side, the multi-layer Perceptone?

124
00:14:40.210 --> 00:14:50.000
Leon: This is the very, very basic thing that you learn when you just start learning machine learning.

125
00:14:50.720 --> 00:15:00.930
Leon: This is the most basic thing you can think about. You have fully connected layers. In our case, we have 96 inputs.

126
00:15:01.110 --> 00:15:02.340
Leon: 24.

127
00:15:02.590 --> 00:15:10.000
Leon: real neurons in the hidden layer. And for outputs.

128
00:15:11.160 --> 00:15:18.269
Leon: The four outputs, well, you already understand what they are about. It's the, these are the shapes.

129
00:15:18.770 --> 00:15:27.740
Leon: Mmm… the shapes, it's, like, gestures, okay, let's call them gestures, yes, they are gestures.

130
00:15:28.090 --> 00:15:38.610
Leon: And the inputs, why are there 96? Because the decision was to use the 3-axis of the accelerometer.

131
00:15:39.490 --> 00:15:44.549
Leon: For 33, 32 consecutive samples.

132
00:15:44.980 --> 00:15:54.609
Leon: So we're sampling in time, like a sliding window, and we always use the 32 last samples for our inference.

133
00:15:56.360 --> 00:15:57.240
Leon: All right.

134
00:15:58.240 --> 00:16:10.520
Leon: So this is the… how the network is set up. 96 inputs, 24 hidden layer, neurons in a hidden layer, and 4 outputs.

135
00:16:13.120 --> 00:16:15.910
Leon: Alright, so now watch me do it.

136
00:16:16.730 --> 00:16:19.379
Leon: What can go wrong, really?

137
00:16:19.710 --> 00:16:24.590
Leon: First of all, I will just flash the unit.

138
00:16:25.580 --> 00:16:28.280
Leon: Okay, so this is the recall.

139
00:16:32.860 --> 00:16:33.880
Leon: Alright.

140
00:16:34.410 --> 00:16:41.970
Leon: Now, I will need Gabor, I will need your help, because I'm not sure…

141
00:16:42.130 --> 00:16:48.649
Leon: how exactly this is going to work. Can you see my camera?

142
00:16:48.780 --> 00:16:49.589
Leon: So it's…

143
00:16:49.590 --> 00:16:54.169
Gabor Szabo: pretty difficult, it's going to be a very small picture, so if you'd like to show that…

144
00:16:54.290 --> 00:16:56.769
Leon: So, yeah, so I will stop the sharing.

145
00:16:56.770 --> 00:16:57.450
Gabor Szabo: Yeah.

146
00:16:58.120 --> 00:16:58.800
Leon: Okay.

147
00:17:00.930 --> 00:17:04.999
Leon: Right. So, now, I hope you can see my…

148
00:17:05.000 --> 00:17:13.449
Gabor Szabo: And those people who are… sorry, and those people who are here in the session, you can change your setting to have speaker view, if you like.

149
00:17:13.599 --> 00:17:21.129
Gabor Szabo: I think, actually, yeah, and in the recording, it will be, I think, hopefully just the whole screen.

150
00:17:22.150 --> 00:17:23.540
Gabor Szabo: Thanks, Lori.

151
00:17:23.540 --> 00:17:28.639
Leon: Okay, so here is the microbit, you know, this is the device.

152
00:17:29.140 --> 00:17:35.789
Leon: And it has a lead matrix of 5x5 LEDs.

153
00:17:36.180 --> 00:17:42.489
Leon: And I have just flashed it, as you have seen, with this software. So what do we see right now?

154
00:17:42.600 --> 00:17:45.569
Leon: First of all, as you remember, is the training.

155
00:17:45.800 --> 00:17:52.480
Leon: So, the initial state… initial state is, rest.

156
00:17:52.840 --> 00:17:55.149
Leon: So, I want to train it to be…

157
00:17:55.290 --> 00:18:00.020
Leon: Rest. In rest. So now, I just press the A button.

158
00:18:00.190 --> 00:18:05.759
Gabor Szabo: We can hardly see the device now. Can you move a little bit to the top? Right, like this. Okay.

159
00:18:05.760 --> 00:18:13.310
Leon: Good, good, good. Thank you. So, I have pressed the A button, and it shows me one LED. I'll press it again.

160
00:18:14.850 --> 00:18:20.059
Leon: And then, again… Maybe I'll hold it a little bit different now, like this.

161
00:18:20.320 --> 00:18:22.770
Leon: And then… like this.

162
00:18:23.860 --> 00:18:27.760
Leon: But, as you can see, I am still holding it.

163
00:18:27.950 --> 00:18:31.359
Leon: very still, I am not moving it when I press.

164
00:18:31.620 --> 00:18:36.950
Leon: Because this is the position in which we're supposed to be in rest.

165
00:18:37.990 --> 00:18:41.079
Leon: This is, one more, more.

166
00:18:45.030 --> 00:18:46.110
Leon: One more.

167
00:18:46.440 --> 00:18:47.520
Leon: One more.

168
00:18:48.500 --> 00:18:52.350
Leon: One more, and the last one. 12 times.

169
00:18:52.740 --> 00:18:56.519
Leon: Alright, now even if I press more, it will stop learning.

170
00:18:57.000 --> 00:19:01.970
Leon: M… Now, I will, just for a second.

171
00:19:02.280 --> 00:19:09.920
Leon: switch the sharing on again, so you can see what I see on my display.

172
00:19:11.350 --> 00:19:17.639
Leon: Alright, notice this… And recording. This is what I see on the debug output.

173
00:19:18.750 --> 00:19:25.320
Leon: We have recorded 12 out of 12 of the rest. Now, if I press B,

174
00:19:26.040 --> 00:19:30.890
Leon: Now, it switches to record, And…

175
00:19:31.170 --> 00:19:34.509
Leon: The next section, which is the right.

176
00:19:35.030 --> 00:19:37.900
Leon: So I'll stop the sharing again.

177
00:19:38.420 --> 00:19:40.530
Leon: And you can see the arrow.

178
00:19:41.210 --> 00:19:49.810
Leon: shows me that we're pointing right. Now, what do I do with it? This will be tricky with this display over here. I will hold it like this.

179
00:19:50.570 --> 00:19:51.300
Leon: Okay?

180
00:19:51.820 --> 00:19:52.630
Leon: So…

181
00:19:53.090 --> 00:20:07.079
Leon: The idea is, as I mentioned, we're talking about a sliding window, so when I press the button, it records what happened for the last 33… 32 samples.

182
00:20:07.510 --> 00:20:13.170
Leon: Which is, time-wise, about 640 milliseconds.

183
00:20:13.550 --> 00:20:17.530
Leon: So, when I press the button, it will record what I've done.

184
00:20:18.750 --> 00:20:21.650
Leon: In the window between now and

185
00:20:21.800 --> 00:20:27.370
Leon: 640 milliseconds back. So what I do is, I just press…

186
00:20:27.660 --> 00:20:33.340
Leon: to do an action, like right, okay? So this is my right action, write, and press the button.

187
00:20:33.500 --> 00:20:35.290
Leon: Right? Press the button.

188
00:20:35.830 --> 00:20:36.810
Leon: Like this.

189
00:20:38.690 --> 00:20:43.240
Leon: I want the action to be Clear?

190
00:20:43.490 --> 00:20:46.509
Leon: And I press the button when I finish my action.

191
00:20:51.660 --> 00:20:57.639
Leon: 12 times. Now I press the B button, and now I learn the,

192
00:20:57.860 --> 00:21:03.010
Leon: the other direction, the left one. So, I do like this.

193
00:21:03.140 --> 00:21:04.189
Leon: Press the button.

194
00:21:15.690 --> 00:21:18.040
Leon: Alright, and the last one?

195
00:21:18.830 --> 00:21:20.050
Leon: This is the shake.

196
00:21:20.330 --> 00:21:28.929
Leon: So I'll try to shake and press the button at the same time, and it doesn't really work for me. I don't know, maybe the muscles don't do this right, but I will try.

197
00:21:40.330 --> 00:21:41.190
Leon: Alright.

198
00:21:41.670 --> 00:21:44.650
Leon: So, now I press the button.

199
00:21:45.630 --> 00:21:46.930
Leon: And watch this.

200
00:21:51.720 --> 00:21:53.519
Leon: Alright, we're done.

201
00:21:55.010 --> 00:21:57.659
Leon: What you've seen now was the training.

202
00:21:58.650 --> 00:22:01.820
Leon: Now, when it's trained, let's see if it catches me.

203
00:22:02.940 --> 00:22:04.200
Leon: All right.

204
00:22:07.030 --> 00:22:09.599
Leon: So, first of all, you see that it works.

205
00:22:11.260 --> 00:22:12.060
Leon: Right?

206
00:22:15.410 --> 00:22:16.490
Gabor Szabo: Very nice.

207
00:22:17.070 --> 00:22:23.570
Leon: Okay, now, The thing is, before we continue, please pay attention

208
00:22:23.680 --> 00:22:36.559
Leon: Just a second, okay, I see there was a hand raised. Before we continue, just notice something interesting. I'm holding my hand like this. When I do the right turn.

209
00:22:36.900 --> 00:22:42.020
Leon: You see that my hand just moves, just tilts just a little bit.

210
00:22:42.610 --> 00:22:48.719
Leon: But when I do the left turn, I do a full, like, at least 90 degrees tilt.

211
00:22:50.160 --> 00:22:55.379
Leon: Okay, so, this already means that there is no

212
00:22:55.790 --> 00:23:03.039
Leon: connection, no link between my right move action, And the left one.

213
00:23:03.990 --> 00:23:05.430
Leon: Right? Clear?

214
00:23:06.130 --> 00:23:11.330
Leon: All right, so, anybody have a question?

215
00:23:19.130 --> 00:23:20.010
Leon: I will…

216
00:23:24.960 --> 00:23:30.080
Leon: Okay, maybe we didn't have a question, so I will share my screen again.

217
00:23:30.080 --> 00:23:30.480
Gabor Szabo: No, no.

218
00:23:30.480 --> 00:23:31.190
Leon: Right, but…

219
00:23:31.190 --> 00:23:31.940
Gabor Szabo: progression.

220
00:23:32.540 --> 00:23:39.709
Leon: And by the way, you can already see that the debug output changed, so it detects the rest.

221
00:23:40.170 --> 00:23:42.910
Leon: Alright? And you see the probability.

222
00:23:43.530 --> 00:23:44.450
Leon: green?

223
00:23:50.680 --> 00:23:51.790
Leon: All right.

224
00:23:53.740 --> 00:23:56.309
Leon: So, we can continue.

225
00:23:59.100 --> 00:24:03.699
Leon: Let me… Show you myself talking again.

226
00:24:04.950 --> 00:24:07.630
Leon: So, nice, nothing went wrong.

227
00:24:07.780 --> 00:24:12.190
Leon: It's perfect. A live demo with nothing went wrong.

228
00:24:13.110 --> 00:24:16.559
Leon: Okay, so let's try to understand what we have seen now.

229
00:24:17.570 --> 00:24:22.730
Leon: we sample… at the… at 50Hz rate.

230
00:24:22.850 --> 00:24:25.809
Leon: So it's like 50 samples per second.

231
00:24:26.290 --> 00:24:28.899
Leon: This is a software decision.

232
00:24:29.840 --> 00:24:35.609
Leon: Okay, why are these 6… 96 neurons at the input?

233
00:24:35.780 --> 00:24:47.310
Leon: I will answer this immediately, we'll just finish talking about this slide. So, this… what you see here is actually a one right tilt.

234
00:24:48.140 --> 00:24:53.780
Leon: Made by me, alright? This is my… Signature tilt.

235
00:24:55.980 --> 00:25:01.030
Leon: Alright? It's… if you do the… if you do the same tilt, it will look different on the graph.

236
00:25:01.320 --> 00:25:04.910
Leon: You have the X, Y, and C axes.

237
00:25:05.760 --> 00:25:11.969
Leon: And we sample them at 50 Hz, which means that 32

238
00:25:13.090 --> 00:25:23.719
Leon: 32 samples are about 640 milliseconds. So this is one window of 32 samples.

239
00:25:24.750 --> 00:25:27.280
Leon: One more thing you can see on this graph.

240
00:25:27.410 --> 00:25:29.670
Leon: Pay attention to the gravity.

241
00:25:30.190 --> 00:25:35.019
Leon: You see the Z axis, which is the vertical one.

242
00:25:35.230 --> 00:25:40.650
Leon: In rest, at rest, it's… at exactly 1G.

243
00:25:41.390 --> 00:25:43.190
Leon: Negative. Down.

244
00:25:44.200 --> 00:26:00.990
Leon: All right? So, this just proves that the thing… the damn thing works, right? It's an accelerometer, it measures acceleration, and if you look on the Z axis, you always see 1G.

245
00:26:01.780 --> 00:26:04.630
Leon: vertically Okay.

246
00:26:04.830 --> 00:26:09.000
Leon: Now, why 96 inputs?

247
00:26:09.190 --> 00:26:14.369
Leon: Let's, knowing what we know now, let's go back to the…

248
00:26:14.370 --> 00:26:17.340
Gabor Szabo: Sorry, sorry, wait a second with that.

249
00:26:17.340 --> 00:26:17.740
Leon: Sorry.

250
00:26:17.740 --> 00:26:18.260
Gabor Szabo: out.

251
00:26:18.260 --> 00:26:18.940
Leon: Yes.

252
00:26:19.180 --> 00:26:23.770
Gabor Szabo: So, what does the X… that's these… the axes, right?

253
00:26:24.690 --> 00:26:29.010
Gabor Szabo: the X movement… Whatever movement is that.

254
00:26:29.400 --> 00:26:31.239
Leon: Yeah, it's a right tilt.

255
00:26:32.630 --> 00:26:34.400
Leon: So this is acceleration.

256
00:26:34.400 --> 00:26:37.279
Gabor Szabo: You moved your hand to the right, okay?

257
00:26:37.430 --> 00:26:40.139
Leon: I, I, I tilted my hand.

258
00:26:40.140 --> 00:26:40.570
Gabor Szabo: Yeah, dude.

259
00:26:40.570 --> 00:26:41.270
Leon: You're right.

260
00:26:41.680 --> 00:26:48.409
Gabor Szabo: And this means that you also moved a little bit to the right, right? That's what it says.

261
00:26:49.240 --> 00:26:51.399
Gabor Szabo: It measured, or, or…

262
00:26:52.020 --> 00:26:59.680
Leon: It measures acceleration, Right? So, you see the movement

263
00:27:00.110 --> 00:27:03.620
Leon: On the x-axis, here, on the right.

264
00:27:03.620 --> 00:27:08.739
Gabor Szabo: the whole device, or specifically the accelerometer, which we don't know where it is, right?

265
00:27:08.740 --> 00:27:09.130
Leon: That's funny.

266
00:27:09.130 --> 00:27:09.730
Gabor Szabo: It's true.

267
00:27:09.730 --> 00:27:10.740
Leon: If you're the left corner.

268
00:27:10.740 --> 00:27:11.520
Gabor Szabo: there.

269
00:27:11.680 --> 00:27:14.929
Gabor Szabo: Then it would move a lot more when you were tilting to.

270
00:27:14.930 --> 00:27:19.639
Leon: Exactly, and you can see the tilt mostly here.

271
00:27:20.640 --> 00:27:21.320
Gabor Szabo: Right.

272
00:27:21.650 --> 00:27:29.050
Leon: Okay, so if you combine these, you will see a tilt. On the Y-axis, almost nothing changed.

273
00:27:30.340 --> 00:27:32.940
Gabor Szabo: Because you didn't move forward and backward.

274
00:27:32.940 --> 00:27:34.200
Leon: Exactly!

275
00:27:34.200 --> 00:27:38.939
Gabor Szabo: You moved a little bit to the right and left, maybe your hand or the whole device is a little bit…

276
00:27:40.020 --> 00:27:40.610
Gabor Szabo: That's a good thing.

277
00:27:40.610 --> 00:27:42.569
Leon: This axis and the Z, yes.

278
00:27:42.960 --> 00:27:49.389
Gabor Szabo: So, because you tilted, the location of the accelerometer went up, and it'll be…

279
00:27:49.390 --> 00:27:49.820
Leon: Yes.

280
00:27:49.820 --> 00:27:50.759
Gabor Szabo: We're down, right?

281
00:27:52.210 --> 00:27:52.950
Gabor Szabo: Okay.

282
00:27:53.630 --> 00:28:04.090
Leon: Now, as you can see, this is a very nice recording, because it has almost the start of the movement, and it ends with a nice, quiet zone.

283
00:28:04.880 --> 00:28:11.960
Leon: Now, let me return to the question, why 96, Y96 inputs.

284
00:28:12.410 --> 00:28:21.649
Leon: As you can see, this graph is a single window, okay? It's a one window of 32 samples.

285
00:28:23.220 --> 00:28:24.050
Leon: Right?

286
00:28:24.330 --> 00:28:30.639
Leon: And this is exactly what we put as an input to the neural network.

287
00:28:31.640 --> 00:28:37.229
Leon: So, on the input, we have the three axes, X, Y, and Z, which is one sample.

288
00:28:37.700 --> 00:28:41.819
Leon: So if it's one sample, we have 3 values per sample.

289
00:28:42.570 --> 00:28:44.760
Leon: Multiplied by 32.

290
00:28:44.880 --> 00:28:47.740
Leon: Because there are 32 samples in every window.

291
00:28:48.690 --> 00:28:57.580
Leon: And these windows are… Inputted into the network every 200 milliseconds.

292
00:28:58.690 --> 00:29:01.900
Gabor Szabo: So the 96 is 3 times 32.

293
00:29:02.140 --> 00:29:02.960
Leon: Yes, exactly.

294
00:29:02.960 --> 00:29:05.880
Gabor Szabo: Why… why 32 samples?

295
00:29:05.880 --> 00:29:17.020
Leon: That's because it covers a good movement, and my movements are inside 9640 milliseconds. That's what I decided. I could decide something different.

296
00:29:17.020 --> 00:29:20.969
Gabor Szabo: So it's more or less arbitrary, but you feel it's a good number.

297
00:29:21.230 --> 00:29:24.250
Leon: Yeah, because, you know, when I'm doing a tilt.

298
00:29:24.590 --> 00:29:27.409
Leon: Okay, or a movement, it's quite quick.

299
00:29:28.990 --> 00:29:29.800
Gabor Szabo: Okay.

300
00:29:29.800 --> 00:29:32.800
Leon: So I don't need more than 600 milliseconds.

301
00:29:34.210 --> 00:29:37.720
Leon: And since I want 32 samples per window.

302
00:29:38.430 --> 00:29:42.360
Leon: and I have 3Xs, then, yeah, the input is 96.

303
00:29:42.650 --> 00:29:48.509
Gabor Szabo: The 600 milliseconds, like, 32 samples in 600 milliseconds, or every 6…

304
00:29:49.650 --> 00:29:59.879
Leon: Just, look here, we have 50 samples per second. Okay. And that means that for a window of 32 samples, which are less than 50,

305
00:30:00.330 --> 00:30:01.830
Leon: You're getting 400…

306
00:30:02.380 --> 00:30:02.900
Gabor Szabo: Oh, okay.

307
00:30:02.900 --> 00:30:04.230
Leon: 30 milliseconds.

308
00:30:04.230 --> 00:30:05.470
Gabor Szabo: I'm a bit slow, yeah.

309
00:30:05.970 --> 00:30:08.130
Leon: No, it's alright, it's alright, it's alright.

310
00:30:09.190 --> 00:30:10.030
Leon: It's good.

311
00:30:10.660 --> 00:30:20.290
Leon: Alright, so, yeah, so this is a single window, and this is what goes into our network.

312
00:30:20.400 --> 00:30:21.770
Leon: into our MLP.

313
00:30:22.300 --> 00:30:29.279
Leon: Okay, and this is a map of all the tilts I've done for a recording.

314
00:30:29.950 --> 00:30:34.409
Gabor Szabo: Tori, what was the 24? Question? Why 24?

315
00:30:34.410 --> 00:30:42.510
Leon: Oh, why 24? 24 is actually a decision of an old, wise man.

316
00:30:43.050 --> 00:30:48.740
Leon: Emm… I could have done something different. 24…

317
00:30:48.870 --> 00:30:58.339
Leon: Simply is a good number, because it separates… it allows nice separation for four outputs.

318
00:30:58.980 --> 00:31:16.179
Leon: And eventually, you know, it's a gut feeling, but after trying 24, I tried other numbers, and they didn't give anything better or worse, so 24 it is.

319
00:31:16.500 --> 00:31:23.790
Leon: But, you know, with neural networks, if you remember, if you've ever done studies, including

320
00:31:23.860 --> 00:31:40.980
Leon: you know, courses on machine learning, then you will know that the decisions about hidden layers is quite arbitrary. And also, you could ask, why do I have only one hidden layer? I could use two hidden layers, for example.

321
00:31:41.450 --> 00:31:44.829
Leon: But in a simple classification task.

322
00:31:44.980 --> 00:31:51.540
Leon: for neurons, there is absolutely no need in another layer, in a hidden layer.

323
00:31:52.590 --> 00:32:00.729
Leon: But, you know, it comes with experience, and with, you know, the things you learn on the way.

324
00:32:02.300 --> 00:32:03.050
Leon: Alright.

325
00:32:07.330 --> 00:32:12.249
Leon: Oh, yes. So, yeah, we have done,

326
00:32:12.670 --> 00:32:18.200
Leon: For this demo, I've done a lot of trials… oh, yeah, the question.

327
00:32:18.390 --> 00:32:23.770
Leon: The question is about, I would say it's,

328
00:32:24.190 --> 00:32:36.140
Leon: optimization techniques, right, we're eventually talking about, techniques to reduce number of routing, and so on. This is a fully connected network. The idea

329
00:32:36.340 --> 00:32:45.539
Leon: Of this dog, Is to show you the most basic learning that you can imagine, Doing useful work.

330
00:32:46.040 --> 00:32:48.589
Leon: From here, you can take it everywhere.

331
00:32:49.580 --> 00:32:57.059
Leon: Right. So, if you look at these recordings, you can clearly see

332
00:32:57.420 --> 00:33:04.770
Leon: the basics. The rest gave us different readings, because I was holding the board at different angles.

333
00:33:08.810 --> 00:33:21.310
Leon: All right? You can see this. And when I do the next action, it's similar, quite similar to the previous slide. You can see that the window catches

334
00:33:21.710 --> 00:33:28.429
Leon: Not… does not necessarily catch the exact spotlight. Here, it's perfect.

335
00:33:29.630 --> 00:33:36.549
Leon: But here, you know, we're catching it somewhere in the middle and the end, and here, this is really a bad sample.

336
00:33:38.000 --> 00:33:41.339
Leon: Alright? Didn't catch my movement, almost.

337
00:33:41.830 --> 00:33:46.490
Leon: And so on and so on. Here, this is a good sample, a good sample.

338
00:33:46.910 --> 00:33:51.469
Gabor Szabo: If I understand correctly, these samples are recorded when you press the button, right?

339
00:33:51.470 --> 00:33:52.350
Leon: Exactly.

340
00:33:52.350 --> 00:33:56.069
Gabor Szabo: And it takes the last 640 milliseconds.

341
00:33:56.070 --> 00:33:56.720
Leon: Yes.

342
00:33:56.720 --> 00:34:08.059
Gabor Szabo: So basically, if I understand correctly, when in the force example, when you almost see nothing, it basically means that you delayed pressing the button a little bit.

343
00:34:08.060 --> 00:34:09.100
Leon: Exactly.

344
00:34:09.100 --> 00:34:10.590
Gabor Szabo: Right? So, okay.

345
00:34:11.380 --> 00:34:17.650
Leon: Exactly. So, the, left recording, looks…

346
00:34:17.949 --> 00:34:20.390
Leon: Quite different, as you can see.

347
00:34:20.590 --> 00:34:26.729
Leon: And I think it recorded better, except for this one, maybe.

348
00:34:27.100 --> 00:34:30.680
Leon: And the shape is a shake, you know, you can see it's a shake.

349
00:34:30.929 --> 00:34:31.710
Leon: Right?

350
00:34:32.020 --> 00:34:41.110
Leon: And, what else? You can pay attention that the Y-axis almost never changes.

351
00:34:41.739 --> 00:34:45.850
Leon: Except maybe here, I had some spike on it, I don't know why.

352
00:34:46.230 --> 00:34:57.310
Leon: M… Well, that's about it. So, these are 48 windows. I recorded 48 windows, 12 for each action.

353
00:34:58.610 --> 00:35:00.900
Leon: Okay, by the way, can I now…

354
00:35:01.630 --> 00:35:04.329
Leon: Move the slide, with my…

355
00:35:06.500 --> 00:35:09.960
Leon: Oh, no, I cannot, I cannot, I cannot. Wait a second.

356
00:35:10.590 --> 00:35:12.730
Leon: Emm… Let me see.

357
00:35:13.480 --> 00:35:14.380
Leon: And…

358
00:35:17.820 --> 00:35:21.060
Leon: Well, I don't remember the comment.

359
00:35:21.220 --> 00:35:36.539
Gabor Szabo: There was also a question while you are doing this, is the model trained on MCU or on PC, and the MCU just used the weights? So, I think I can answer the question, that it was trained on the MCU, so we saw how Leon was,

360
00:35:36.630 --> 00:35:55.990
Gabor Szabo: recording the samples when he was moving his hand and clicking the buttons. That's when he recorded the sample, and I guess then behind the scene, I don't remember we seeing that you did any action to do that. Then it was trained, and then…

361
00:35:56.480 --> 00:36:08.090
Gabor Szabo: then… then he showed how his movements, then generate the arrows. So that was already after training it. Am I correct, Non?

362
00:36:08.090 --> 00:36:10.000
Leon: Yes, yes, yes, you are.

363
00:36:10.630 --> 00:36:13.150
Gabor Szabo: Okay, so maybe I learned something, that's good.

364
00:36:13.750 --> 00:36:28.069
Leon: All right, so, no, no, no, no, not this one, just a second. Where is the… oh, yeah. So this is the GitHub repo that you will be able to access later. I will copy it from here.

365
00:36:28.180 --> 00:36:40.530
Leon: the command… And, with the… But now… Oh.

366
00:36:41.800 --> 00:36:43.759
Leon: Just really big Stein.

367
00:36:44.380 --> 00:36:45.110
Leon: Oh.

368
00:36:48.730 --> 00:36:58.079
Leon: I don't know what's going on here now, but anyway, I will show you what we do when we want to attach the unit, so…

369
00:36:58.660 --> 00:37:05.379
Leon: This is the command, and we will see the explanation of it a bit later.

370
00:37:08.260 --> 00:37:09.460
Leon: Yeah, sudo.

371
00:37:11.360 --> 00:37:12.150
Leon: Alright.

372
00:37:14.050 --> 00:37:19.419
Leon: So, now, I will check if I can move stuff.

373
00:37:20.250 --> 00:37:22.610
Leon: with my micro 8.

374
00:37:22.750 --> 00:37:23.600
Leon: See?

375
00:37:23.980 --> 00:37:24.800
Leon: Oh.

376
00:37:25.130 --> 00:37:26.999
Leon: Alright, I'm doing left.

377
00:37:27.340 --> 00:37:29.460
Leon: It's moving left, moving right.

378
00:37:29.680 --> 00:37:33.109
Leon: Okay, maybe I will even, try to show it.

379
00:37:33.220 --> 00:37:34.190
Leon: like this.

380
00:37:34.190 --> 00:37:37.909
Gabor Szabo: We only see some text in the… in the screen.

381
00:37:38.070 --> 00:37:43.980
Leon: Yes, yes, but you see that when I move, It presses a button.

382
00:37:45.050 --> 00:37:52.660
Gabor Szabo: Oh, it moves the cursor. When you tilt the device, then it moves the cursor to the right.

383
00:37:53.000 --> 00:37:53.880
Leon: Exactly.

384
00:37:54.600 --> 00:37:59.389
Leon: Okay? So it imitates It imitates a keyboard.

385
00:38:00.250 --> 00:38:09.730
Leon: Okay, now, why is it useful? Because for this presentation, which dropped out for some reason, Let's…

386
00:38:10.130 --> 00:38:11.919
Leon: Try to switch it back on.

387
00:38:14.160 --> 00:38:17.769
Leon: All right, now I will try to go to the next slide.

388
00:38:19.660 --> 00:38:30.470
Leon: Very good. So, yeah. So this is just to prove that we're talking about something useful and not something boring.

389
00:38:30.870 --> 00:38:38.440
Leon: Okay, so our next slide was here, and this… we do two preprocessing steps.

390
00:38:38.810 --> 00:38:50.960
Leon: which are quite standard preprocessing steps for a generic sensor. Really nothing quite special for accelerometer or other stuff, but…

391
00:38:51.160 --> 00:39:00.079
Leon: this is really some simple action. So, the raw recording is on the left, We remove the X's.

392
00:39:00.540 --> 00:39:02.290
Leon: mean values.

393
00:39:02.650 --> 00:39:07.139
Leon: As you can see here, they all come more to the center.

394
00:39:08.330 --> 00:39:16.229
Leon: Why? Because we don't want to care about the initial position of the device and the Earth's gravity.

395
00:39:17.870 --> 00:39:18.730
Leon: All right.

396
00:39:18.880 --> 00:39:23.339
Leon: So we remove the mean from the axis, so it becomes like this.

397
00:39:23.650 --> 00:39:32.229
Leon: And the next one, you don't see much, because you need to look here at the axis. We simply divide by 1000,

398
00:39:32.390 --> 00:39:38.169
Leon: to, switch from, milli-G to G.

399
00:39:39.900 --> 00:39:46.900
Leon: All right? And why? Just because we, you know, neural networks like smaller numbers.

400
00:39:47.350 --> 00:39:51.650
Leon: No, nothing scary or special about it.

401
00:39:52.610 --> 00:40:05.439
Leon: And here is some code, so, you know, speaking of frost, this is some code here that does this exact thing, and you can already see it's quite simple.

402
00:40:05.650 --> 00:40:18.730
Leon: Nothing really special. We have the mean value, which becomes from the array of three X's.

403
00:40:18.890 --> 00:40:21.770
Leon: So this, runs 3 times.

404
00:40:22.100 --> 00:40:29.079
Leon: And we iterate over the samples in this window.

405
00:40:30.530 --> 00:40:33.259
Leon: And we,

406
00:40:33.600 --> 00:40:44.449
Leon: We're doing the sum of all the values in this window, and then we divide it by the,

407
00:40:44.630 --> 00:40:52.810
Leon: window size, and we get the result as F32, so we get the mean per axis.

408
00:40:53.390 --> 00:41:11.540
Leon: For the whole window, and immediately afterwards, we want to output the, what we've seen on the previous slide, so we run once again on all the three axes. But now, we're actually building the 96

409
00:41:11.650 --> 00:41:14.549
Leon: Inputs to the network.

410
00:41:15.610 --> 00:41:16.540
Leon: Okay?

411
00:41:16.690 --> 00:41:23.420
Leon: This is the way we do it. We run it with T times X's.

412
00:41:23.530 --> 00:41:26.709
Leon: So, per excess, we are in…

413
00:41:27.370 --> 00:41:35.139
Leon: going through. So, like, for, for the window, for,

414
00:41:35.330 --> 00:41:44.060
Leon: For the beginning, like, say, we have 96 inputs, so we begin with, and…

415
00:41:45.940 --> 00:42:00.060
Leon: with, say, 0, and then it switches 1, 2, and 3. So we're getting the 3Xs of this sample, and then we switch to the next sample. So next one will be 1,

416
00:42:00.410 --> 00:42:05.559
Leon: And, we are getting,

417
00:42:06.010 --> 00:42:10.630
Leon: 0, 1, and 2, so it will be blank.

418
00:42:10.770 --> 00:42:24.799
Leon: 1, and… sorry, it will be… it jumps 3, of course, because times X's. So, we'll jump 3, 4, and 5. This is the next sample. And then…

419
00:42:25.150 --> 00:42:29.450
Leon: We'll continue with jumps of 3.

420
00:42:29.740 --> 00:42:36.800
Leon: For each sample, and adding them up. So we're, like, taking the,

421
00:42:36.850 --> 00:42:46.020
Leon: the three samples per… sorry, the three X's per sample, and we stretch them.

422
00:42:46.020 --> 00:42:57.290
Leon: into a line of 96 inputs. Now, it's not… it doesn't really matter how we set them up, because this is just a neural network.

423
00:42:57.620 --> 00:43:00.790
Leon: We need… the only thing we need to do is to be consistent.

424
00:43:01.080 --> 00:43:05.349
Leon: Of course. So the same inputs will always be on the same places.

425
00:43:06.810 --> 00:43:23.800
Leon: And that's it, and then we provide the sample itself, the all three values of each sample, and minus the mean value of that sample, and divide by 1000.

426
00:43:24.080 --> 00:43:27.990
Leon: So, this is how we're getting Here.

427
00:43:28.400 --> 00:43:36.270
Leon: Okay, not only here, but to the whole window of 32 samples, 96 values.

428
00:43:36.890 --> 00:43:42.770
Leon: Alright, so this is how we receive our features. Features are the inputs to the neural net.

429
00:43:44.690 --> 00:43:53.769
Leon: Now, about the keyboard trick, we already mentioned it, so if you remember this, you're officially old.

430
00:43:55.920 --> 00:44:00.740
Leon: And I know that you didn't consider yourself old just because you remember it, but…

431
00:44:01.850 --> 00:44:04.289
Leon: This is how things work.

432
00:44:04.770 --> 00:44:08.300
Leon: Yes, yes, you're old. I'm sorry to say that.

433
00:44:09.180 --> 00:44:14.739
Leon: Mmm… Right, so… because my children have no idea what this is, yeah?

434
00:44:16.080 --> 00:44:17.720
Leon: And they are not little.

435
00:44:18.750 --> 00:44:29.800
Leon: Okay, so, the protocol can be found, on the web, at various old university sites.

436
00:44:29.800 --> 00:44:39.550
Leon: And the command on Linux is just input attached. This is a user space app that connects to a communication port.

437
00:44:39.580 --> 00:44:43.870
Leon: And then converts it to a clicks in user space.

438
00:44:43.990 --> 00:44:49.750
Leon: You must have sudo because you connect to the port, unless you've defined this port as accessible.

439
00:44:49.990 --> 00:44:52.540
Leon: And, this is the protocol.

440
00:44:53.910 --> 00:45:00.040
Leon: Mmm… On Windows, you may even…

441
00:45:00.560 --> 00:45:06.060
Leon: And… find a driver that will do this for you as well.

442
00:45:06.250 --> 00:45:11.979
Leon: But, you know, I… usually don't touch Windows at all, so… whatever.

443
00:45:13.090 --> 00:45:20.770
Leon: Alright, now, this is a recording.

444
00:45:20.970 --> 00:45:28.300
Leon: like the one I showed you on the screen previously, like the one… over here.

445
00:45:28.820 --> 00:45:43.529
Leon: Right? And this is how it looks like for, let's say, 12 seconds, with several actions I've performed. So, after this ride, I had a tiny hint of previews, of left.

446
00:45:44.500 --> 00:45:57.170
Leon: But it was a vague hint, and it was not a real action, and it was below the probability that I had decided

447
00:45:57.370 --> 00:46:01.500
Leon: That will, trigger the key prints.

448
00:46:01.770 --> 00:46:08.129
Leon: Alright? I have decided that it will be 0.75 probability.

449
00:46:08.840 --> 00:46:13.400
Leon: So everything above 0.75 will trigger the key.

450
00:46:15.680 --> 00:46:19.749
Leon: Right? I hope this graph is quite clear. There were several.

451
00:46:19.750 --> 00:46:22.110
Gabor Szabo: That's in the program, right?

452
00:46:23.390 --> 00:46:24.410
Leon: Yes, exactly.

453
00:46:24.770 --> 00:46:25.380
Gabor Szabo: Okay.

454
00:46:26.800 --> 00:46:32.500
Leon: All right. Now, note something interesting. We record a window

455
00:46:32.710 --> 00:46:39.040
Leon: every 200 milliseconds. This means that windows overlap.

456
00:46:40.840 --> 00:46:49.909
Leon: We record… I'll say it again… we record a 640 millisecond window every 200 milliseconds.

457
00:46:51.080 --> 00:46:55.689
Leon: And that's exactly why you see most of them are triple.

458
00:46:56.060 --> 00:47:00.640
Leon: Because there are, like, 3 recordings for my window.

459
00:47:02.370 --> 00:47:03.490
Leon: the trigger.

460
00:47:05.570 --> 00:47:07.300
Leon: Hope you understand the concept.

461
00:47:07.560 --> 00:47:24.980
Leon: Alright, so it's a sliding window, and I record them when they overlap, and that's exactly why, mostly, you see three recordings. Here, we caught another one. It was a longer one, and so on, but this is the normal operation.

462
00:47:25.510 --> 00:47:30.679
Gabor Szabo: Do I understand correctly that those two blue… bluish lines, the lower ones at the beginning.

463
00:47:30.680 --> 00:47:31.050
Leon: Yeah.

464
00:47:31.050 --> 00:47:36.150
Gabor Szabo: Those were when you actually returned your hand to the regular position?

465
00:47:36.750 --> 00:47:42.890
Leon: I suppose so, yes. I suppose so, and… because they are low confidence.

466
00:47:43.270 --> 00:47:44.300
Leon: Yeah, so…

467
00:47:44.300 --> 00:47:45.150
Gabor Szabo: over.

468
00:47:45.790 --> 00:48:03.950
Leon: Exactly, so they are low confidence, because this one, it really looks like it was a slow tilt, because these ones are a bit lower probability, and these ones got here somehow as well, and… yeah, that's… that's how it looks like in real life.

469
00:48:05.270 --> 00:48:06.310
Leon: Alright.

470
00:48:06.950 --> 00:48:21.009
Leon: Now, the program flow, as you can… as you have seen already, we record 12 times the last 640 milliseconds, 32 samples, windows. For each gesture, it's…

471
00:48:21.650 --> 00:48:25.629
Leon: For each guest chair, it is 48 windows total. This is wrong.

472
00:48:26.850 --> 00:48:33.669
Leon: Sorry, I suppose… I suppose something got mixed up here in the sentence, because

473
00:48:34.150 --> 00:48:39.530
Leon: We record, 12 times for each gesture.

474
00:48:40.100 --> 00:48:47.840
Leon: And this is, what, 48 windows in total. Okay, so this is a typo here or somewhere, it's a mistake.

475
00:48:48.210 --> 00:48:51.430
Leon: In total, we have 48 windows recorded.

476
00:48:51.950 --> 00:48:57.980
Leon: four gestures. Then we train… we train 200 passes off

477
00:48:58.410 --> 00:49:04.899
Leon: 48 windows each. So, we have 9,600 weight updates.

478
00:49:05.490 --> 00:49:06.460
Leon: Inputable.

479
00:49:06.950 --> 00:49:16.429
Leon: So, this means this is 9,600 backpropagation cycles. It's forward propagation and then backpropagation.

480
00:49:17.390 --> 00:49:18.240
Leon: Alright?

481
00:49:19.180 --> 00:49:26.520
Leon: And this is the simplest plain stochastic gradient descent that you know from every machine learning book.

482
00:49:27.180 --> 00:49:29.250
Leon: It's nothing more than the 3.

483
00:49:30.020 --> 00:49:34.229
Leon: And… and then we started working while we were working.

484
00:49:34.230 --> 00:49:37.610
Gabor Szabo: I guess there is a crate implementing the training, right?

485
00:49:37.830 --> 00:49:38.839
Leon: Not at all.

486
00:49:38.960 --> 00:49:48.659
Leon: It's all plain, simple code, because, you know, we're in really low-level embedded here. We run no STD.

487
00:49:48.790 --> 00:49:55.939
Leon: Which means, you know, when you run without operating systems, very little crates, I suppose, would help me here.

488
00:49:57.150 --> 00:49:57.890
Gabor Szabo: Okay.

489
00:49:58.780 --> 00:50:04.409
Leon: And then we start working, and working is exactly the previous slide. This is the work.

490
00:50:04.660 --> 00:50:19.900
Leon: And for every, like, 200 milliseconds, or actually it's 207, including reading the sensors, we estimate a gesture of the last 640 milliseconds, so windows overlap to make sure we catch it.

491
00:50:20.280 --> 00:50:24.959
Leon: And if we are confident enough, we generate a key press. That's the thing.

492
00:50:26.140 --> 00:50:28.509
Leon: Let's, see the data flow.

493
00:50:29.110 --> 00:50:37.570
Leon: So, the acceleration… accelerate… accelerometer sample, which is I16, And 3 of them.

494
00:50:38.460 --> 00:50:45.369
Leon: Alright, this is what we're getting, but pay attention that the accelerometer provides 12 bits of precision.

495
00:50:46.890 --> 00:50:55.399
Leon: Alright, so this is the 12-bit precision analog converter, and we use i16.

496
00:50:55.980 --> 00:51:00.220
Leon: Then we have the window, which is 32 of these.

497
00:51:00.510 --> 00:51:04.960
Leon: 32 samples. This is how the array looks like.

498
00:51:06.160 --> 00:51:10.300
Leon: And by the way, why did I say array and not vector?

499
00:51:10.810 --> 00:51:12.120
Leon: Any ideas?

500
00:51:14.980 --> 00:51:24.109
Leon: Well, it's just because we don't use any dynamic allocation here. It's all plain, stupid arrays.

501
00:51:24.470 --> 00:51:28.140
Leon: Okay, there is zero, a dynamic allocation in this program.

502
00:51:28.530 --> 00:51:30.470
Leon: And you'll see it soon.

503
00:51:31.450 --> 00:51:43.599
Leon: Then, this window, or, say, the data of this window is converted to the features data, just like we…

504
00:51:43.850 --> 00:51:47.129
Leon: Measured? Yes. Yes, exactly.

505
00:51:47.520 --> 00:52:00.789
Leon: So, just like we measured previously, and then we have our MLP, which is 96 in the input layer, then 24 in hidden layer, and 4 on the output.

506
00:52:00.940 --> 00:52:06.719
Leon: And this is what eventually provides us with a gesture with the probability.

507
00:52:08.280 --> 00:52:13.330
Leon: And then we may consider to running the PS2 key.

508
00:52:13.730 --> 00:52:23.160
Leon: And so, right, so a window is just an accumulation of 32 samples, and, the features arrive, but…

509
00:52:23.690 --> 00:52:30.760
Leon: Notice that between the window and the features, we converted to F32.

510
00:52:30.970 --> 00:52:39.309
Leon: We have seen it in the code snippet that we've seen before, that we begin with ints, but the features are already

511
00:52:39.470 --> 00:52:40.530
Leon: floats.

512
00:52:41.260 --> 00:52:42.850
Leon: Now, why?

513
00:52:43.910 --> 00:52:45.710
Leon: Why do we use floats?

514
00:52:48.010 --> 00:52:55.139
Leon: We use floats because, actually, we have an FPU, right? This chip has an FPU, so why not use floats?

515
00:52:56.210 --> 00:53:00.680
Leon: Okay, it will be quite fast, and we don't need to, you know.

516
00:53:01.190 --> 00:53:15.639
Leon: go crazy about it. Now, we could train on ints as well, but as you can imagine, training on ints will not give us as good as results that we receive.

517
00:53:16.040 --> 00:53:30.569
Leon: Because it's not precise. So, yeah, I suppose that for some use cases, you can go with ints, but you don't need it here. Now, also, I mention it because

518
00:53:31.000 --> 00:53:37.580
Leon: people find it a little bit hard to understand, because most of the models that you put on an MCU

519
00:53:38.130 --> 00:53:41.039
Leon: Are quantized to int 8.

520
00:53:42.010 --> 00:53:48.920
Leon: Why? Because we want to make it faster, the forward propagation faster, and use less memory.

521
00:53:50.070 --> 00:53:53.200
Leon: But here, first of all, you have seen that we have

522
00:53:53.840 --> 00:53:56.970
Leon: Lots of memory, at least for this scale of project.

523
00:53:57.510 --> 00:54:00.020
Leon: And,

524
00:54:00.990 --> 00:54:10.810
Leon: And we're quite fast. We're fast because we have the FP, so there is absolutely no need of quantizing it or using ints.

525
00:54:11.450 --> 00:54:20.970
Leon: Eventually, you know, you can quantize after you've trained your model in a floating point. So, this is not what's happening here.

526
00:54:22.140 --> 00:54:25.980
Leon: Alright, so this is how the MLP looks like.

527
00:54:28.530 --> 00:54:32.560
Leon: Alright, as you can see, this is all static.

528
00:54:32.900 --> 00:54:38.180
Leon: Very, very simple. We have weights and biases of the hidden layer.

529
00:54:38.310 --> 00:54:40.479
Leon: We have the weights and the biases.

530
00:54:40.930 --> 00:54:43.320
Leon: Of the, output layer.

531
00:54:43.500 --> 00:54:50.890
Leon: And the input one… Does not need weights and biases, because it's just the Alright?

532
00:54:51.500 --> 00:54:59.650
Leon: Mmm… The initialization of the network Damn…

533
00:55:00.050 --> 00:55:16.279
Leon: Yes. For the initialization, we need to… you know, you cannot train… this is from neural networks course, you know, you cannot train a neural network with zeros inside. You cannot do math on it.

534
00:55:16.600 --> 00:55:34.809
Leon: So, you need to fill it out with something. Usually, you use some randomish numbers. Here, you also use randomish numbers as an order ratio trick, or fill it up in randomish, but not really random numbers, because I didn't want to use

535
00:55:34.930 --> 00:55:38.399
Leon: the, any sort of RNG,

536
00:55:38.560 --> 00:55:43.580
Leon: Just because, you know, the quasi-random stuff is…

537
00:55:44.020 --> 00:55:54.209
Leon: not better than the golden ratio. Actually, the golden ratio is better. And I don't want to use the hardware RNG, because why should I even bother?

538
00:55:54.570 --> 00:55:58.150
Leon: And it's longer, and it will, you know, take more time, and so on.

539
00:55:58.960 --> 00:56:06.079
Leon: So, but I do… but I do initialize the network in runtime.

540
00:56:08.040 --> 00:56:15.629
Leon: Alright, the beginning, in the initialization of the code. So, in the code, the MLP is a singleton.

541
00:56:17.110 --> 00:56:34.609
Leon: Why did I use a singleton here? Not to make sure that nobody calls it from a different spot, but just because this, using Cortex-M singleton macro will place the… will make it static on the BSS

542
00:56:34.880 --> 00:56:39.079
Leon: Instead of, you know, initializing it as usual on the stack at runtime.

543
00:56:39.730 --> 00:56:56.639
Leon: Now, why did I do this? Just because it's fun. I never used the singleton macro from Cortex before. I could have it on a stack. This is really not a problem. I started with it on the stack, and then I tried to put in

544
00:56:56.980 --> 00:57:05.269
Leon: To put it there, and it's nice in the code, so pay attention when you rip down the repo afterwards.

545
00:57:07.320 --> 00:57:11.779
Leon: Alright, this is our main part of it.

546
00:57:11.880 --> 00:57:20.730
Leon: And, it just has some initializations, like the sensor itself, and the I.O, which is ours, this,

547
00:57:20.790 --> 00:57:33.260
Leon: this I.O. struct is what handles the, everything related to time, like time ticks and recording the windows, including the buttons.

548
00:57:33.730 --> 00:57:45.530
Leon: And we have the keyboard, which is the PS2 keyboard on the UART. Very simple. Here is the single DOM, this is the model itself.

549
00:57:46.020 --> 00:57:53.650
Leon: And then we have the three steps, the recording, Then training, and then… to work. That's it.

550
00:57:53.860 --> 00:57:54.990
Leon: That simple.

551
00:57:56.020 --> 00:57:57.589
Leon: Really nothing much in it.

552
00:57:58.270 --> 00:58:02.789
Leon: About the forward pass,

553
00:58:03.430 --> 00:58:11.920
Leon: It is quite simple, but I'm not really going to really dig into the code here. The code here is really, really simple.

554
00:58:12.030 --> 00:58:25.989
Leon: The logic here is the standard logic of neural networks, so it's like, you know, the sum of 96 weights times 96 inputs.

555
00:58:25.990 --> 00:58:33.170
Leon: you know, what happens in every neuron. It's all the weights times all the inputs, plus the bias.

556
00:58:33.580 --> 00:58:42.780
Leon: So, you know, nothing new was invented here. And it has the re-loop function, which is implemented just with Macs.

557
00:58:43.060 --> 00:58:49.719
Leon: And I've gave, I put here the reload function, GraphQL.

558
00:58:50.000 --> 00:58:51.790
Leon: Want to recall.

559
00:58:52.040 --> 00:58:55.240
Leon: And this is the hidden layer.

560
00:58:55.380 --> 00:59:04.609
Leon: This is all the forward paths of the hidden layer. It calculates these sums, and then runs the real function.

561
00:59:04.610 --> 00:59:16.489
Leon: But we do save the, the pre, the pre-activation, because we need it for the back propagation when we do the learn.

562
00:59:17.340 --> 00:59:26.249
Leon: Yeah, so, and this is the output, the output layer. It's exactly the same.

563
00:59:26.440 --> 00:59:34.640
Leon: As above. We also use the dot product, which is described over here below, we'll get there immediately.

564
00:59:34.780 --> 00:59:39.230
Leon: But it's exactly the same theme.

565
00:59:39.460 --> 00:59:50.770
Leon: And just for less neurons this time, it's just for neurons, that's why I used this array access instead of the for loop.

566
00:59:50.860 --> 01:00:08.570
Leon: And we have the softmax at the output, and for people who are not familiar, the outputs become probabilities, but we want the sum of all probabilities on the output to be 1.

567
01:00:08.810 --> 01:00:09.810
Leon: Always.

568
01:00:10.340 --> 01:00:14.320
Leon: So, the softbox does exactly that.

569
01:00:14.750 --> 01:00:17.030
Leon: And… that's about it.

570
01:00:17.540 --> 01:00:24.380
Leon: And the dot product function, there is one thing I can say about it. First of all, the dot…

571
01:00:24.530 --> 01:00:25.900
Leon: product?

572
01:00:26.080 --> 01:00:29.449
Leon: does… Exactly this operation.

573
01:00:29.580 --> 01:00:34.490
Leon: Okay, not the bias, but it sums multiplications.

574
01:00:35.580 --> 01:00:43.189
Leon: we could… You know, when you talk about embedded, you always want to optimize for speed.

575
01:00:43.620 --> 01:00:55.829
Leon: And there are operations that allow you to optimization on this kind of functions, like multiplication and sum.

576
01:00:56.700 --> 01:01:00.840
Leon: But… We are living in a float world.

577
01:01:00.960 --> 01:01:07.309
Leon: In floating point, we just have the FPU on this, at least on this MCU.

578
01:01:07.710 --> 01:01:27.269
Leon: And if you're working… if you're working with integers, then… even then, you will have hard time of making the compiler really optimize your stuff, and you probably will end up writing some assembly code here, if you want… if you really want it optimized for the CPU.

579
01:01:27.470 --> 01:01:37.890
Leon: But this is not about it right now, and this does the work perfectly and quickly on this CPU, on the floating point process.

580
01:01:39.660 --> 01:01:55.360
Leon: The backpropagation will be in two slides and without much of a code, because it's just more complicated than would fill the screen, and we don't want it. But I have decided to give you the feeling of what's going on to make it in examples.

581
01:01:55.690 --> 01:02:05.020
Leon: So, you may consider we have the example of output, when, running the,

582
01:02:05.280 --> 01:02:13.239
Leon: the teaching, and we have the photo right, yeah, it was the correct action that was right, and we do have

583
01:02:13.340 --> 01:02:15.099
Leon: The highest value on it.

584
01:02:15.240 --> 01:02:19.540
Leon: So we had… 0.1, 0.6, 0.2, and 0.1.

585
01:02:20.300 --> 01:02:27.359
Leon: Now, we run the derivative of, just of the right answer.

586
01:02:27.800 --> 01:02:30.440
Leon: And so, the result will be

587
01:02:30.610 --> 01:02:33.950
Leon: Minus 0.4 for the right one.

588
01:02:34.070 --> 01:02:36.289
Leon: There are others remain the same.

589
01:02:36.410 --> 01:02:40.710
Leon: Why? Because we know what action it was, so this is the supervised learning.

590
01:02:41.120 --> 01:02:42.610
Leon: Emm…

591
01:02:43.190 --> 01:02:54.239
Leon: this is how we achieve it. Very simple. Nothing, complex. And then, we have the, the forward.

592
01:02:54.240 --> 01:03:04.769
Leon: Which is computed, and we… well, we've done it before. We had a forward pass, which did this operation, as we mentioned.

593
01:03:05.710 --> 01:03:10.340
Leon: We had the logit, this logit, for example.

594
01:03:10.620 --> 01:03:13.119
Leon: We had it calculated in this way.

595
01:03:13.510 --> 01:03:16.499
Leon: Kind of this way, this is just for…

596
01:03:16.760 --> 01:03:31.360
Leon: Getting a feeling, the sum of multiplications. And the backwards propagation does exactly the reverse of it to find the delta for the hidden layer for each neuron.

597
01:03:31.450 --> 01:03:40.390
Leon: So for each neuron, we fire the sum of multiplications for the logins.

598
01:03:41.030 --> 01:03:42.939
Leon: And this is how we do it.

599
01:03:43.520 --> 01:03:45.290
Leon: Pretty simple, nothing…

600
01:03:45.440 --> 01:03:54.540
Leon: really is complicated here. And, you know, this is the back propagation, the classic, the most classic type of backpropagation.

601
01:03:55.580 --> 01:04:03.689
Leon: Then we have the last two stages. We choose only… by the way, this is only for the hidden layer.

602
01:04:03.810 --> 01:04:16.400
Leon: Because the input layer has nothing… has no weights at all, and we don't care. But the hidden layer, we need to ask ourselves.

603
01:04:16.610 --> 01:04:18.920
Leon: Should we then update the weights?

604
01:04:19.120 --> 01:04:26.110
Leon: for a given neuron. Because if the neuron did not affect the output.

605
01:04:26.760 --> 01:04:29.000
Leon: Then why would we teach it something?

606
01:04:29.570 --> 01:04:33.799
Leon: So, if the value of the neuron, the output value.

607
01:04:34.450 --> 01:04:45.859
Leon: the pre-calculated one was 0 or below. Well, you almost never get zero, right? If you get zero, this means that this neuron is out of business.

608
01:04:46.150 --> 01:04:51.670
Leon: It happens, but this is not the goal, and, you know, with floats, you almost never get zero.

609
01:04:52.340 --> 01:04:55.360
Leon: So if it's less than zero.

610
01:04:55.580 --> 01:05:00.190
Leon: We… we just continue. We don't… we do not update anything.

611
01:05:01.760 --> 01:05:09.519
Leon: But if this neuron did affect the output, we want to slightly push it towards the right solution.

612
01:05:09.680 --> 01:05:11.449
Leon: So, what do we do?

613
01:05:11.810 --> 01:05:22.090
Leon: We, for example, again, example, say the node H, that we've discussed here, and so on, it contributed

614
01:05:22.540 --> 01:05:25.269
Leon: 2.0. This is what it had on the output.

615
01:05:25.860 --> 01:05:30.630
Leon: The right error, we know already, is minus 0.4.

616
01:05:31.060 --> 01:05:44.619
Leon: And by the way, the more, as you can notice, the more the correct… the result is correct closer to 1, the error will be, of course, slower, right? Because we're doing…

617
01:05:46.100 --> 01:05:47.180
Leon: Minus 1.

618
01:05:47.930 --> 01:05:52.209
Leon: So, for the right neuron, we're… what we're getting here

619
01:05:52.470 --> 01:05:59.539
Leon: And we're getting some preset value that we'll just see in a second.

620
01:05:59.870 --> 01:06:06.470
Leon: And then times 0.4, which is the error, times 2, which is the original output.

621
01:06:07.160 --> 01:06:14.869
Leon: And this is the value, so we'll… we will get, eventually, 0.616.

622
01:06:16.610 --> 01:06:19.250
Leon: Okay. Now,

623
01:06:23.710 --> 01:06:25.070
Leon: Open 6…

624
01:06:26.160 --> 01:06:40.950
Leon: Yeah, yeah. So, it looks like I skipped an explanation here, because the original value of the weight is 0.6. I've skipped the…

625
01:06:41.040 --> 01:06:51.789
Leon: the number here in explanation. We are treating the weight itself as 0.6, so when we're doing this operation, minus equals.

626
01:06:51.960 --> 01:07:10.269
Leon: then it means that the original value, plus, because it's minus times minus, 0.016, this is the output of this, this multiplication, and we're getting 6.16. So, this neuron on the right will contribute more

627
01:07:11.390 --> 01:07:16.170
Leon: But if we are doing exactly the same, For the left one.

628
01:07:16.600 --> 01:07:26.549
Leon: Okay, which is… If you remember from the start, it was… the left was 0.2 and remained 0.2.

629
01:07:26.850 --> 01:07:31.250
Leon: Okay, didn't receive minus 1, then it will end up

630
01:07:31.600 --> 01:07:44.449
Leon: say it was 0.2, so it will become 0.192, so it becomes smaller. So its effect on the neuron output will become smaller.

631
01:07:44.740 --> 01:07:52.689
Leon: This is all very basic, it's very standard, so if you're confused, I suppose you can look it up.

632
01:07:54.170 --> 01:08:13.050
Leon: on the web, and this value, the 0.02 in my case, it's just the learning rate that I decided on. Once again, this is just a decision, and I choose how quickly I want the network to be teached.

633
01:08:15.210 --> 01:08:19.419
Leon: And it works quite nicely here, I didn't have to even play with it.

634
01:08:20.460 --> 01:08:30.190
Leon: So, this was the back propagation, and this is the interesting graph. You always see these when speaking of errors and learning.

635
01:08:33.920 --> 01:08:49.259
Leon: Oh, okay. So, you can see the loss drops, the error drops, okay? While we're teaching, we have the 200 epochs of 48 windows each.

636
01:08:49.920 --> 01:09:07.639
Leon: One funny thing to note about it, check a look, take a look. These are marks over here, and here, and here on the… on equal distances, and where the marks are, you can see some spikes, like this one, like here.

637
01:09:07.810 --> 01:09:13.499
Leon: And here, And these, and these, and these, very clearly.

638
01:09:13.899 --> 01:09:15.279
Leon: Why are they?

639
01:09:17.000 --> 01:09:23.089
Leon: Because… I'm doing one more thing that I didn't mention till now.

640
01:09:23.390 --> 01:09:30.789
Leon: Every window that goes into the training, like the 48 windows in each epoch.

641
01:09:31.229 --> 01:09:37.009
Leon: I take all the 48 windows and shift… sorry, rotate them left.

642
01:09:40.120 --> 01:09:42.060
Leon: Okay, you can see it here.

643
01:09:42.380 --> 01:09:46.769
Leon: This is a window. This is the original window.

644
01:09:47.810 --> 01:09:50.920
Leon: Okay, it's the tilt as it was recorded.

645
01:09:51.260 --> 01:10:02.710
Leon: Just one of the X's for simplicity. And this one is the same tilt shifted, sorry, rotated left 17 times.

646
01:10:04.170 --> 01:10:09.820
Leon: So I keep rotating it because I want the neural network to learn

647
01:10:10.300 --> 01:10:13.840
Leon: what the action I did is.

648
01:10:14.150 --> 01:10:16.129
Leon: And not what the numbers are.

649
01:10:16.750 --> 01:10:22.189
Leon: Okay, this is a standard learning trick for every neural network, and

650
01:10:22.370 --> 01:10:38.200
Leon: you can notice that when it's shifted, like, 70 times left, rotated left 70 times, it looks like it's a totally different action, right? It starts high, and then drops low, and then becomes high again.

651
01:10:38.680 --> 01:10:43.069
Leon: It's the opposite action, like, it's… looks like an opposite action.

652
01:10:43.490 --> 01:10:48.010
Leon: And that's exactly why you have the spikes here. It thinks that

653
01:10:48.220 --> 01:10:58.779
Leon: These are not really good samples, and it's correct, but eventually, it… in neural networks, it really helps the network learn better.

654
01:10:59.050 --> 01:11:04.989
Leon: If you skip this step, it will learn, you'll have slightly worse results.

655
01:11:06.330 --> 01:11:11.590
Leon: Okay, so this is also another useful technique in neural networks.

656
01:11:12.460 --> 01:11:16.599
Leon: So, we're at the end, and, was it useful?

657
01:11:16.990 --> 01:11:23.039
Leon: First of all, consider what we spoke about before, the customization in field.

658
01:11:23.160 --> 01:11:37.680
Leon: Like, you can take a microcontroller in field, and adjust it, like, for a gameplay, or for a disabled person, or for a sensor, or for machine shaking.

659
01:11:37.980 --> 01:11:42.370
Leon: You can do a very, very simple action very cheaply.

660
01:11:42.510 --> 01:11:51.669
Leon: Infield, and it will be a completely stand-alone device that will Do actual, useful work.

661
01:11:52.830 --> 01:12:08.459
Leon: Right? So, you can do a lot of useful things with this. As you have seen, this device here works very nicely for different actions that I have,

662
01:12:08.690 --> 01:12:20.290
Leon: Programmed it for, and it uses the simplest and the dumbest form of, of,

663
01:12:22.390 --> 01:12:25.270
Leon: I lost the word. Classification.

664
01:12:25.850 --> 01:12:32.459
Leon: Right? We just need to classify for four actions in a certain window, and it does perfectly.

665
01:12:32.550 --> 01:12:46.220
Leon: with a standard, very standard network, I could do preprocessing. If I knew that I'm working with actions, I could do some preprocessing, and I've done it, by the way, to improve the precision and so on.

666
01:12:46.580 --> 01:12:49.050
Leon: Or maybe useless memory and such.

667
01:12:49.190 --> 01:12:57.059
Leon: But I don't need to, because it already does this good job. So imagine what you can achieve on this tiny

668
01:12:57.310 --> 01:13:01.269
Leon: Very, very cheap microcontroller for your use cases.

669
01:13:02.400 --> 01:13:07.630
Leon: Alright? So… Yeah, I guess,

670
01:13:08.050 --> 01:13:14.220
Leon: If anyone has bigger questions, this would be a good time.

671
01:13:18.270 --> 01:13:30.440
Gabor Szabo: this was an excellent presentation. Can you paste the link to the chat as well? I will definitely add it to the… there is going to be a webpage for the

672
01:13:30.550 --> 01:13:36.529
Gabor Szabo: for the event, or there is already, I'm going to share it here in the chat.

673
01:13:38.070 --> 01:13:47.760
Leon: Okay, actually, I wanted to see people on cameras doing, like, this telephone thing, but, you know… Sorry?

674
01:13:47.760 --> 01:13:48.450
Gabor Szabo: 22?

675
01:13:48.450 --> 01:13:55.490
Leon: I wanted to see people, you know, shaking their phones, like, trying to catch the barcodes on the cameras.

676
01:13:55.720 --> 01:13:59.979
Leon: But yeah, I will, paste it right now.

677
01:14:03.810 --> 01:14:08.740
Gabor Szabo: Okay, so here is this. This is going to be the link where the content of the

678
01:14:08.970 --> 01:14:14.399
Gabor Szabo: We'll have the links there, and it's going to be… I'll advertise it here in the

679
01:14:14.660 --> 01:14:25.220
Gabor Szabo: you will find it under the video, so if you're watching the video only now, YouTubers, you will find this link below the video, and…

680
01:14:26.470 --> 01:14:31.419
Gabor Szabo: And I will also send it out to all the people who registered to this event.

681
01:14:31.960 --> 01:14:37.610
Gabor Szabo: M… Any questions, in the chat, that,

682
01:14:38.810 --> 01:14:44.170
Gabor Szabo: reinforce… someone is writing that I think a good approach could be also reinforcement learning.

683
01:14:45.890 --> 01:14:54.430
Leon: Well, it depends. I don't think we should really consider anything specific at this point. I…

684
01:14:54.670 --> 01:15:01.160
Leon: Just wanted to give you the feeling of what you can achieve very easily.

685
01:15:01.290 --> 01:15:07.050
Leon: On the, on a very tiny microcontroller, so, yeah.

686
01:15:07.320 --> 01:15:08.160
Gabor Szabo: Yeah, I think it…

687
01:15:08.160 --> 01:15:08.910
Leon: I mean?

688
01:15:10.540 --> 01:15:21.120
Gabor Szabo: you really showed that all kinds of things can be done on something, and I'm surprised that it's not, like, standard that people are doing this, but…

689
01:15:21.410 --> 01:15:23.270
Gabor Szabo: Okay.

690
01:15:23.950 --> 01:15:24.670
Gabor Szabo: Yes.

691
01:15:24.670 --> 01:15:35.780
Leon: So, why shift the data over the window? Yeah, so let me return to sharing, because we discussed it all

692
01:15:36.070 --> 01:15:39.230
Leon: This slide just, over here.

693
01:15:40.020 --> 01:15:49.459
Leon: This is quite a standard method. I did not invent it, I used it during studies a lot.

694
01:15:49.760 --> 01:16:04.540
Leon: it simply helps to recognize the pattern on the input better by the network, and I will give you an example from the real-world courses,

695
01:16:04.730 --> 01:16:09.909
Leon: You may, even if you did not,

696
01:16:10.080 --> 01:16:13.719
Leon: Learn anything about networks and such.

697
01:16:13.910 --> 01:16:26.449
Leon: You probably, heard that during, during the, the learning.

698
01:16:27.540 --> 01:16:40.120
Leon: People usually, like, for the kittens' images, like, neural networks that recognize kitten images, very common for a neural network and machine learning courses.

699
01:16:40.270 --> 01:16:46.669
Leon: They don't just take the picture of the kittens and throw it in as it is.

700
01:16:47.080 --> 01:16:49.390
Leon: what the,

701
01:16:50.070 --> 01:17:04.169
Leon: What they do is they do throw them in like they are, but then they also turn them the other way around, and they inverse the colors, and they do a lot of actions that actually quite distort the picture.

702
01:17:04.320 --> 01:17:12.560
Leon: Why? Because they still want the, the network to see this kitten just like you would.

703
01:17:12.680 --> 01:17:20.899
Leon: Because if you see a cat in the dark, from your balcony, two stories below.

704
01:17:21.300 --> 01:17:26.820
Leon: You can still recognize it's a cat, so you want the neural network to also

705
01:17:26.980 --> 01:17:34.649
Leon: See it in a distorted way to understand that this is the cat, or this is the action.

706
01:17:34.980 --> 01:17:46.839
Leon: And here, what I do is the simplest form of distorting the input, and still keeping it clear that this was this action.

707
01:17:47.060 --> 01:17:58.079
Leon: Right? So, this is kind of pre-training. You can look it up on the web, and you'll find a lot of explanations for it.

708
01:18:01.240 --> 01:18:02.580
Leon: Any more questions?

709
01:18:06.890 --> 01:18:07.960
Leon: All right.

710
01:18:09.120 --> 01:18:10.429
Gabor Szabo: Well, either my…

711
01:18:10.430 --> 01:18:16.210
Leon: Either my explanations were very good, or maybe most of them are asleep already.

712
01:18:17.250 --> 01:18:32.409
Gabor Szabo: No, I think, some parts were absolutely clear, especially after you answered the questions and so, and well, some part just went over my head, but

713
01:18:32.530 --> 01:18:47.160
Gabor Szabo: This was excellent, and we will share the… hopefully we can also share the slides, but definitely we'll share the links you already shared. I will also include them in the page.

714
01:18:47.360 --> 01:19:02.550
Gabor Szabo: And those people watching the YouTube, now let's finish this up. Those people watching the YouTube, please like the video and follow the channel, and below the video, you will find the links to

715
01:19:02.890 --> 01:19:11.840
Gabor Szabo: Future events and all these things that you mentioned in this event, that you shared in the chat.

716
01:19:11.910 --> 01:19:24.960
Gabor Szabo: And, thank you, everyone, for those who were present and asked questions and made comments, and especially thank you, Leon, for giving the presentation. It was very, really nice. Thanks, Devin. Thank you very much.

717
01:19:25.040 --> 01:19:38.100
Gabor Szabo: And those who are here in the meeting, you can stay around. We stop now the recording, and then we can keep the conversation on off the record. So, thank you, and bye-bye.




