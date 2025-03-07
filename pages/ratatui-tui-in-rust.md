---
title: Ratatui - Terminal User Interfaces in Rust with Orhun Parmaksız
timestamp: 2025-03-07T09:30:01
author: szabgab
published: true
show_related: true
description:
---

In this session we were live-coding (pair programming) a TUI using the Ratatui crate in Rust and answering any questions that you might have.

{% youtube id="OkmYsa25pIw" file="2025-03-06-ratatui-terminal-user-interfaces-in-rust.mp4" %}


[Orhun Parmaksız](https://www.linkedin.com/in/orhunp/), the author of [Ratatui](https://ratatui.rs/)


![Orhun Parmaksız](images/orhun-parmaksiz.jpeg)

To get started run:


```
cargo generate ratatui/templates
```

* The [ratatop GitHub repo](https://github.com/szabgab/ratatop) we worked on.

* [Ratatui on GitHub](https://github.com/ratatui/ratatui)

* Website of [Orhun Parmaksız](https://orhun.dev/)
* [YouTube channel of Orhun Parmaksız](https://www.youtube.com/@orhundev)

* [cargo generate](https://github.com/cargo-generate/cargo-generate)
* [Ratatui templates](https://github.com/ratatui/templates)

For "hot reload" the output gui, without having to quit and `cargo run` try:

```
cargo watch --no-process-group --clear -x run
```

Additional Widget: [tui-bar-graph](https://crates.io/crates/tui-bar-graph)


## Transcript

1
00:00:02.070 --> 00:00:10.320
Gabor Szabo: So Hello, and and welcome to this wonderful meeting. And if you're watching the video, then welcome to the code Me. Events. Youtube channel.

2
00:00:11.010 --> 00:00:21.650
Gabor Szabo: My name is Gabor. I usually help companies use rust teaching them or helping them other ways. Same with python.

3
00:00:21.850 --> 00:00:28.590
Gabor Szabo: And I also love to learn other things, and I'm really happy that this meeting on this meeting, or

4
00:00:29.530 --> 00:00:40.129
Gabor Szabo: agreed to show me, show us Ratatouille, because I already started to prepare actually a presentation about it when when we didn't have a presentation at one of our meetings.

5
00:00:40.460 --> 00:01:00.839
Gabor Szabo: But then I figured it would be much better if someone who actually knows what he's doing will tell us about it. So is this, just be, if you're watching the video, please, just like the video and follow the channel, because we have before I forget. And now it's your turn on. Please introduce yourself and take over whatever you'd like.

6
00:01:01.770 --> 00:01:15.090
Orhun Parmaksiz: Yep, welcome, everybody. And yeah, my my name is Orhan, as Gabor said, and I'd like to say, Thank you for the invite. 1st of all, it's a great opportunity to have this hands on

7
00:01:15.250 --> 00:01:30.865
Orhun Parmaksiz: payer programming session thing. And I think it's gonna be very fun. Because, yeah, we're going to just dive into recipe. Practically, I'm not going to just show you a presentation. And here's this thing. Here's that thing, and so on. But yeah,

8
00:01:31.580 --> 00:01:39.250
Orhun Parmaksiz: other than that, I am basically maintaining this projecti is a brust library for building terminal user interfaces.

9
00:01:39.480 --> 00:01:57.700
Orhun Parmaksiz: And I am just doing open source these days. Actually, not these days. But I should say these years, and trying to like, you know, make something out of it, creating some content on the Internet. You can also follow me on Youtube. It's Orhan Dev.

10
00:01:58.010 --> 00:02:02.240
Orhun Parmaksiz: and yeah, just enjoying rust and building stuff. So

11
00:02:02.470 --> 00:02:05.879
Orhun Parmaksiz: that's that's what we're gonna do today as well.

12
00:02:06.440 --> 00:02:20.739
Orhun Parmaksiz: So yeah, feel free to like, use the chat and ask questions. If me or Gobor catches a question, we can just answer it on the fly. I think that's a good, like interactive way of

13
00:02:21.680 --> 00:02:25.740
Orhun Parmaksiz: moving forward with this session. So just feel free.

14
00:02:26.150 --> 00:02:44.449
Orhun Parmaksiz: And I like to. 1st of all show you what Reta 3 is, and what we are planning to build today together. So I will try to share my screen. If my Linux setup Arch Linux, by the way allows me to do so, because, you know.

15
00:02:44.900 --> 00:02:50.670
Orhun Parmaksiz: anything can crash at this point because of zoom. But I'm guessing you can see my screen now right.

16
00:02:50.870 --> 00:02:53.250
Gabor Szabo: Yeah, we we can see some firefox.

17
00:02:53.860 --> 00:02:55.340
Orhun Parmaksiz: Yeah, okay, great.

18
00:02:55.870 --> 00:03:00.900
Orhun Parmaksiz: And you see, docs, rs, I hope right. Everything is fine.

19
00:03:01.390 --> 00:03:02.070
Gabor Szabo: Yes.

20
00:03:02.230 --> 00:03:15.829
Orhun Parmaksiz: Okay, great. So rota 3 is like, like, I said, it's a 3 library, 3 stands for terminal user interface. So we're basically building these interfaces in the terminal

21
00:03:16.410 --> 00:03:17.839
Orhun Parmaksiz: using this library.

22
00:03:18.944 --> 00:03:20.679
Orhun Parmaksiz: I don't want like.

23
00:03:20.870 --> 00:03:36.329
Orhun Parmaksiz: get into the concepts and everything right now, because actually, we're gonna just code something together. And you know, while we're doing that with Kabul, I'm just gonna like talk over it. But the idea is to let me actually run some Demos

24
00:03:36.440 --> 00:03:37.600
Orhun Parmaksiz: to show it to you.

25
00:03:40.890 --> 00:03:43.339
Orhun Parmaksiz: I got to go to examples.

26
00:03:44.970 --> 00:03:46.520
Orhun Parmaksiz: Apps. Demo.

27
00:03:47.440 --> 00:03:50.730
Orhun Parmaksiz: Let's run this one. Well, let me actually run this one.

28
00:03:52.200 --> 00:04:04.679
Orhun Parmaksiz: So yeah, as you can see, I'm in the terminal. And I'm just running one of the demos. And I'm gonna just talk about what we can build with Residi very shortly. And then I'm going to show it to you what we're gonna build today.

29
00:04:07.310 --> 00:04:10.420
Orhun Parmaksiz: So basically, we can have like these

30
00:04:10.700 --> 00:04:17.990
Orhun Parmaksiz: tabs widgets. And these charts, sparkline gauges, lists, chart. I don't know. Text colored

31
00:04:18.100 --> 00:04:37.900
Orhun Parmaksiz: stuff, a map, and so on. So you can just build these type of colored dashboard like interfaces using Reta 3. And also we have other stuff in here as well. So if you're interested, just go to route 3 like github.com Reta 3 slash router 3,

32
00:04:38.140 --> 00:04:50.649
Orhun Parmaksiz: and you will have some examples. The best way of getting started is to just go to ratity dot Rs. And we have a bunch of tutorials and documentation there, and sorry for the light theme. One second.

33
00:04:50.930 --> 00:04:59.282
Orhun Parmaksiz: I know that some people are complaining now, even though I don't hear them. Okay, there you go. So yeah, I just all right.

34
00:04:59.920 --> 00:05:03.170
Orhun Parmaksiz: yep. So today, I want to

35
00:05:03.320 --> 00:05:05.880
Orhun Parmaksiz: build something simple. We don't have

36
00:05:06.060 --> 00:05:19.070
Orhun Parmaksiz: much time. I don't think this session will be 6 h long. So let's aim for something reasonable. And I have this little program that I prepared for a workshop. Actually.

37
00:05:19.340 --> 00:05:26.590
Orhun Parmaksiz: so I'm gonna show it to you. Cargo run. Yeah, very simple. Some CPU

38
00:05:26.700 --> 00:05:47.390
Orhun Parmaksiz: stats, some disk stats memory network and some processes. So this is like a very simple H top like program built with, we're not using any like other 3rd party widgets here, just the stock widgets, and everything is just styled being colored

39
00:05:47.580 --> 00:05:51.279
Orhun Parmaksiz: and rendered using the tree.

40
00:05:51.440 --> 00:06:02.329
Orhun Parmaksiz: And this list is this table is interactive, and so on. So I think this is a good thing to just demonstrate how to use the widgets, how to structure your app.

41
00:06:02.550 --> 00:06:21.320
Orhun Parmaksiz: and how to handle events, and so on. Because when you start building applications, one of the most commonly asked questions is, how do you structure your rust application because we don't provide you a like a boilerplate, so to speak. We don't like

42
00:06:21.500 --> 00:06:24.212
Orhun Parmaksiz: Vertuse is like, kind of like a

43
00:06:24.980 --> 00:06:41.229
Orhun Parmaksiz: like, you can basically like structure application like you want. We know we don't like. Say that you should like, put your logic here. Your your like modules should be like this, and so on, and we don't like provide a end.

44
00:06:42.430 --> 00:06:46.309
Orhun Parmaksiz: for example, a stateful approach. So

45
00:06:48.400 --> 00:06:50.229
Orhun Parmaksiz: let me give you an example. Actually.

46
00:06:50.490 --> 00:06:59.329
Orhun Parmaksiz: So there are like 3rd party libraries that you can use for using. For example, elm, architecture like react like an architecture.

47
00:06:59.460 --> 00:07:15.190
Orhun Parmaksiz: but we don't do that in rotate. So there isn't a a something like, you know. You have the widgets. You have their states, you you kind of like, get some kind of callbacks from them, and so on. There isn't such thing. So you have to kind of

48
00:07:15.570 --> 00:07:31.979
Orhun Parmaksiz: find your architecture that suits your use case best and kind of stick to it. So I think this is a good way of starting out. So this is, let's just try to build this I'm gonna like we're gonna start from the 0 ground 0. So

49
00:07:32.575 --> 00:07:35.869
Orhun Parmaksiz: I'll try to like guide you through everything here.

50
00:07:36.100 --> 00:07:42.750
Orhun Parmaksiz: And I think it's going to be fun. So let's just do it. What do you say? Let's let's just build it together.

51
00:07:43.580 --> 00:07:48.580
Gabor Szabo: Yeah. And I was moving my computer. So my microphone got disconnected for a second more.

52
00:07:49.190 --> 00:07:50.800
Gabor Szabo: Hopefully. Now it's working.

53
00:07:51.560 --> 00:07:52.600
Orhun Parmaksiz: Yep, I can hear you.

54
00:07:52.600 --> 00:07:56.709
Gabor Szabo: Okay, they did. Somehow, it has some con

55
00:07:56.870 --> 00:08:05.330
Gabor Szabo: connection issues with the USB, anyway. Okay, so shall I share my screen now? And and okay.

56
00:08:05.500 --> 00:08:06.489
Orhun Parmaksiz: Let's do it.

57
00:08:15.990 --> 00:08:24.480
Gabor Szabo: Okay. So I am on Terminal. I I'm I'm a white white person, or whatever it's called.

58
00:08:26.960 --> 00:08:33.100
Gabor Szabo: Yeah, little less like, okay. So white. Background. Now, not background. Yeah.

59
00:08:33.679 --> 00:08:37.479
Orhun Parmaksiz: Okay, so let's create a repo. 1st of all.

60
00:08:37.730 --> 00:08:39.150
Gabor Szabo: Just to the empty people.

61
00:08:39.893 --> 00:08:45.909
Orhun Parmaksiz: Okay, let's let's actually use the templates that we provide. So do you have cargo generate, installed.

62
00:08:49.750 --> 00:08:50.800
Gabor Szabo: This one.

63
00:08:51.590 --> 00:09:02.919
Orhun Parmaksiz: Yeah. So for context cargo generate is a tool that allows you to initialize a repository with some

64
00:09:03.180 --> 00:09:08.890
Orhun Parmaksiz: predefined code, we, we can use this tool to pull some

65
00:09:08.990 --> 00:09:16.319
Orhun Parmaksiz: some source from A from a repo on on Github and just initialize a project really? Quickly.

66
00:09:16.938 --> 00:09:21.780
Orhun Parmaksiz: Wait. I'm gonna send a link to the chat in case people are not familiar with the tool.

67
00:09:22.880 --> 00:09:35.080
Orhun Parmaksiz: Let's do that. Cargo generates. There you go. So in order to initialize a Rosa. 3 app. You need to run cargo, generate route, 3 slash templates.

68
00:09:39.070 --> 00:09:39.740
Gabor Szabo: Rich.

69
00:09:40.650 --> 00:09:43.400
Orhun Parmaksiz: Yeah. Yeah. Slash templates. Yeah, there you go. I'm gonna send.

70
00:09:43.400 --> 00:09:47.729
Gabor Szabo: Oh, so I can't. I can't type that fast as you do. So. Okay.

71
00:09:47.890 --> 00:09:49.800
Orhun Parmaksiz: No, that's it.

72
00:09:50.150 --> 00:09:51.090
Orhun Parmaksiz: There you go. Yep.

73
00:09:53.220 --> 00:10:14.990
Orhun Parmaksiz: all right. So cargo generate asks you which one you want to use here, and let's go with simple. We don't need Async now, we don't need a component architecture. Hello, world is too simple for this. So let's go with simple yeah. Project name. I would go for Rat Top, because, like, you know, h top, but rather top kind of.

74
00:10:14.990 --> 00:10:17.220
Gabor Szabo: Like this or or lower case.

75
00:10:17.500 --> 00:10:22.709
Orhun Parmaksiz: That's fine. Yeah, just something random. We can use something else, too. Here. So there you go.

76
00:10:22.710 --> 00:10:23.579
Orhun Parmaksiz: Okay, alright.

77
00:10:23.580 --> 00:10:30.399
Orhun Parmaksiz: And so it's created a repository for us. If you go into the top and open up your editor.

78
00:10:30.890 --> 00:10:32.769
Orhun Parmaksiz: or like, kind of Yeah.

79
00:10:36.070 --> 00:10:36.670
Gabor Szabo: Yes.

80
00:10:36.670 --> 00:10:41.109
Orhun Parmaksiz: Be the file tree here. It's like a very simple

81
00:10:41.500 --> 00:10:45.729
Orhun Parmaksiz: application, just the source app and main files.

82
00:10:46.180 --> 00:10:47.850
Orhun Parmaksiz: We don't see our editor right now, but.

83
00:10:47.850 --> 00:10:50.210
Gabor Szabo: No, no. It opened on another window

84
00:10:50.510 --> 00:10:56.209
Gabor Szabo: another screen. So I waited till you were finishing token. Okay, yeah. So.

85
00:10:56.660 --> 00:10:58.870
Orhun Parmaksiz: So let's go over the files.

86
00:11:02.000 --> 00:11:05.210
Gabor Szabo: Okay, let's go over the files, you see.

87
00:11:05.210 --> 00:11:08.360
Orhun Parmaksiz: Yeah. Yeah. So in main. Rs.

88
00:11:08.750 --> 00:11:15.979
Orhun Parmaksiz: let's go to the main. Rs, in case you don't know this, like the entry point of your app. And here we are initializing

89
00:11:16.130 --> 00:11:25.209
Orhun Parmaksiz: your terminal for you so that you can just start coding your application. What we are doing here is actually let me just go over the lines

90
00:11:25.310 --> 00:11:51.719
Orhun Parmaksiz: on line 6. We are installing a error handler method. This is for also panic handling. You don't need to know the details, but color, air, or color error. I don't know how to pronounce it. It's a library that helps with handling those type of errors so that we don't like need to define our own error type, and so on. So that's what we're doing there. And then

91
00:11:52.160 --> 00:12:09.779
Orhun Parmaksiz: we are creating a terminal with the row to see in its call on line 7. This is our like. This is like the main thing that we need when it comes to building user interfaces. We're going to just use this terminal struct to render the application.

92
00:12:09.980 --> 00:12:13.330
Orhun Parmaksiz: And like, render like, kind of

93
00:12:13.660 --> 00:12:16.139
Orhun Parmaksiz: don't need it for event handling.

94
00:12:16.350 --> 00:12:26.349
Orhun Parmaksiz: But we need that for basically your your widgets, and so on. And then we run the app I'm going to talk about App in a second.

95
00:12:26.800 --> 00:12:53.439
Orhun Parmaksiz: and we, after we run the app, we get a result from the whole application and then restore the terminal on line 9. If you don't restore it. It's basically something like, you know, you have a terminal user interface. You basically go to a new screen. And then you got to come back to your original terminal. That's why we need to restore it. If you don't restore it. You kind of like, get stuck in that alternate screen.

96
00:12:53.500 --> 00:13:02.829
Orhun Parmaksiz: And you're gonna get like corrupted terminal output. So you need to do a terminal in it. Sorry respiratory in it, and respiratory. Restore when we use

97
00:13:03.310 --> 00:13:07.830
Orhun Parmaksiz: term. When we when we are building 3 apps, which is a C,

98
00:13:07.960 --> 00:13:11.249
Orhun Parmaksiz: and then, lastly, we just return the results from main.

99
00:13:11.400 --> 00:13:20.449
Orhun Parmaksiz: So our app is the is the struct that holds our application state. We can go to app dot. Rs, now.

100
00:13:22.410 --> 00:13:26.220
Orhun Parmaksiz: okay, so you can see on Line 11,

101
00:13:26.340 --> 00:13:49.709
Orhun Parmaksiz: we have our struct, and that's basically like where our state is. And we have one boolean that's just running, which designates, if you're still running the application or not. If this is false, we're going to quit the application. And in the input, block we just create a new app with the running state false. If you scroll down a bit.

102
00:13:49.960 --> 00:13:51.599
Orhun Parmaksiz: we have the run function

103
00:13:51.730 --> 00:14:18.749
Orhun Parmaksiz: which is being called from the main. And this is basically creating a loop for us. And in this loop we are rendering some frames and handling the events. The recipe is a immediate mode rendering library, which means you gotta redraw your whole ui for each frame. It's not like, you know. You kind of draw everything once and update it.

104
00:14:18.830 --> 00:14:36.749
Orhun Parmaksiz: It's like, no, it's not like that. It's just like kind of the opposite. So in this loop, while the self running is true, while the application is still running, we are drawing some frames. This is being handled in the draw function.

105
00:14:36.750 --> 00:14:58.700
Orhun Parmaksiz: and also we are handling some cross term. Events. Cross term is the backend that we are using in. You can use different backends. But cross term is like a cross platform backend. So we're just using cross term there. So in this loop we are doing 2 things, drawing the frames and also handling events. If you scroll down a bit you'll see the draw function.

106
00:14:58.980 --> 00:15:01.070
Orhun Parmaksiz: So in the draw function.

107
00:15:01.370 --> 00:15:09.440
Orhun Parmaksiz: you are rendering your user interface. So this is where we're gonna actually do most of the work for this app.

108
00:15:09.720 --> 00:15:12.469
Orhun Parmaksiz: And right now it's just rendering some

109
00:15:12.630 --> 00:15:19.850
Orhun Parmaksiz: text, and that's basically it. We're gonna run this in a second. You'll you'll see it scroll down a bit.

110
00:15:20.290 --> 00:15:47.809
Orhun Parmaksiz: You'll see the event handling function, handle Cross term events. Here we are calling event read. And this is a cross term function. This is giving us some key events, mouse events, or some resize events or anything else. And for the key events we are just matching on this return value. Like, I think it's a yeah enum. So we are matching on enum and calling on key event

111
00:15:47.980 --> 00:15:54.580
Orhun Parmaksiz: and on Q and function is just checking the keys. If it is

112
00:15:54.930 --> 00:16:07.829
Orhun Parmaksiz: Q or control. C, we just call self quit. And if you scroll down a bit you'll see that the self quit function is basically setting running to false.

113
00:16:08.460 --> 00:16:13.583
Orhun Parmaksiz: That's it. So that's very simple. If you can run this

114
00:16:14.280 --> 00:16:23.930
Gabor Szabo: And then this will turn off right? Yeah. So how do you run? You run it in inside, in in a terminal like this, or just usually to regular terminal.

115
00:16:24.460 --> 00:16:28.039
Orhun Parmaksiz: I think, for now you can run it in the integrated terminal there. Yes.

116
00:16:28.040 --> 00:16:34.950
Gabor Szabo: Okay, because it's like smaller. And and I don't know whatever how. Well do they? The terminal sync.

117
00:16:35.670 --> 00:16:36.630
Orhun Parmaksiz: Yeah, so

118
00:16:37.260 --> 00:16:50.880
Orhun Parmaksiz: it's resizing, based on your layouts and so on. And I'm going to talk about it in a second. But that's what you're gonna get when you run this, basically some text, and when you press Q, it will just exit.

119
00:16:52.430 --> 00:16:53.220
Gabor Szabo: Yep.

120
00:16:54.210 --> 00:16:58.509
Orhun Parmaksiz: So that way, you can just use cargo. Generate to

121
00:16:59.130 --> 00:17:02.549
Orhun Parmaksiz: quickly start your own 3 application.

122
00:17:02.830 --> 00:17:05.450
Orhun Parmaksiz: So, okay, that that was the

123
00:17:05.609 --> 00:17:08.499
Orhun Parmaksiz: 1st chapter in my mind. All right.

124
00:17:09.250 --> 00:17:10.349
Orhun Parmaksiz: So any.

125
00:17:10.660 --> 00:17:12.040
Gabor Szabo: Good.

126
00:17:13.789 --> 00:17:15.639
Orhun Parmaksiz: You can, you can commit it. Yes.

127
00:17:25.710 --> 00:17:26.520
Gabor Szabo: Okay.

128
00:17:28.880 --> 00:17:29.609
Orhun Parmaksiz: All right.

129
00:17:29.960 --> 00:17:38.029
Orhun Parmaksiz: So let's let's move on and create some layouts. Because.

130
00:17:39.340 --> 00:17:41.914
Orhun Parmaksiz: yeah, can you send the repo now? Oh, yeah.

131
00:17:42.200 --> 00:17:45.590
Gabor Szabo: Let's set up a report because I don't. Don't have a report yet.

132
00:17:45.810 --> 00:17:48.170
Gabor Szabo: and it's what what is it called.

133
00:17:52.340 --> 00:17:53.139
Orhun Parmaksiz: Just create a.

134
00:17:53.140 --> 00:17:54.130
Gabor Szabo: Prototype.

135
00:17:54.650 --> 00:17:57.310
Orhun Parmaksiz: Yeah, just rest us up, or something. I don't know.

136
00:17:59.056 --> 00:18:06.403
Orhun Parmaksiz: In the meantime, if if anybody has any questions feel free to ask in. Ask it in the chat.

137
00:18:07.080 --> 00:18:08.960
Orhun Parmaksiz: here to ask for.

138
00:18:30.710 --> 00:18:31.690
Gabor Szabo: Okay.

139
00:18:31.860 --> 00:18:35.310
Gabor Szabo: I put the link to the report in the chat.

140
00:18:36.090 --> 00:18:38.770
Orhun Parmaksiz: Alright, perfect, so.

141
00:18:39.030 --> 00:18:43.709
Gabor Szabo: Video as well. So just those people who are watching the video. Okay, if you find it there.

142
00:18:44.420 --> 00:18:52.410
Orhun Parmaksiz: Alright. So let's create some layouts for our app. Because if you remember what I showed you, we have like different blocks.

143
00:18:53.351 --> 00:18:56.230
Orhun Parmaksiz: I'm just hearing myself for some reason.

144
00:18:57.950 --> 00:18:59.620
Orhun Parmaksiz: Test test test.

145
00:19:00.010 --> 00:19:01.240
Orhun Parmaksiz: Is anybody.

146
00:19:01.700 --> 00:19:02.580
Orhun Parmaksiz: Wait.

147
00:19:03.710 --> 00:19:06.109
Gabor Szabo: I can hear you well, but sometimes it.

148
00:19:06.610 --> 00:19:08.160
Orhun Parmaksiz: Okay, no, it's gone.

149
00:19:08.160 --> 00:19:09.430
Gabor Szabo: Okay. Cool.

150
00:19:09.730 --> 00:19:19.210
Orhun Parmaksiz: Yeah, I added a little bit of echo, but now it's gone. So let's create some layouts. If you remember the the app that I showed you. We have the CPU memory.

151
00:19:19.500 --> 00:19:19.900
Gabor Szabo: Correct.

152
00:19:19.900 --> 00:19:24.670
Orhun Parmaksiz: And so on. So to do that you can actually delete.

153
00:19:24.780 --> 00:19:28.219
Orhun Parmaksiz: It's not I should delete it, but you can keep the title.

154
00:19:29.070 --> 00:19:29.999
Orhun Parmaksiz: Delete the rest.

155
00:19:30.710 --> 00:19:32.979
Orhun Parmaksiz: We don't need the rest, for now.

156
00:19:33.611 --> 00:19:36.429
Gabor Szabo: So the text and the frame I can. All this.

157
00:19:36.430 --> 00:19:44.690
Orhun Parmaksiz: Just yeah. Just remove those actually remove title 2. We don't. Let's start from scratch. Remove title 2.

158
00:19:44.690 --> 00:19:45.390
Gabor Szabo: Okay.

159
00:19:45.700 --> 00:19:52.150
Orhun Parmaksiz: So to kind of split your area into different parts and

160
00:19:52.320 --> 00:20:01.830
Orhun Parmaksiz: build like render different blocks. You need to define some layouts for that you, you can basically

161
00:20:04.340 --> 00:20:08.690
Orhun Parmaksiz: called the layout functions. So if you

162
00:20:09.040 --> 00:20:14.119
Orhun Parmaksiz: if you just start typing, I'll try to guide you a little bit.

163
00:20:14.120 --> 00:20:16.140
Gabor Szabo: Leota, something.

164
00:20:16.725 --> 00:20:23.169
Orhun Parmaksiz: Yeah, just let layout. Yeah, exactly. And layout just the struct.

165
00:20:24.180 --> 00:20:32.059
Orhun Parmaksiz: And we want to like create a vertical layout. We want to split vertically. So 2 columns and vertical.

166
00:20:33.780 --> 00:20:34.700
Gabor Szabo: Okay.

167
00:20:36.550 --> 00:20:40.059
Orhun Parmaksiz: And in the constraints. So this is how you are basically

168
00:20:40.440 --> 00:20:47.610
Orhun Parmaksiz: splitting your your layout, your your area for constraints. You need to create a

169
00:20:47.890 --> 00:20:51.809
Orhun Parmaksiz: a. Vector no, not a. Vector but like like use brackets.

170
00:20:53.970 --> 00:20:55.650
Gabor Szabo: Like this one, Curtis.

171
00:20:56.390 --> 00:20:57.150
Gabor Szabo: No one knows.

172
00:20:57.310 --> 00:21:00.239
Orhun Parmaksiz: No, not currently the the other brackets.

173
00:21:00.630 --> 00:21:04.710
Orhun Parmaksiz: And in in these constraints thing.

174
00:21:04.710 --> 00:21:06.660
Gabor Szabo: Just writing, writing or code. Yeah.

175
00:21:06.660 --> 00:21:17.529
Orhun Parmaksiz: It's crazy. You gotta like, define your constraints. So let's do something like constraint. Percentage constraints destruct.

176
00:21:18.530 --> 00:21:20.649
Gabor Szabo: So writing in constraints.

177
00:21:22.025 --> 00:21:31.740
Orhun Parmaksiz: No, just the uppercase, like no, no. Just cost constraint, constraint. And to Colon.

178
00:21:32.270 --> 00:21:39.450
Orhun Parmaksiz: sorry it's not extracted. So enum. Yeah, exactly. And percentage. Let's give it like, I don't know 25

179
00:21:40.920 --> 00:21:46.759
Orhun Parmaksiz: percentage. It's a yeah, 25%.

180
00:21:47.170 --> 00:21:48.980
Gabor Szabo: Here I type in 25.

181
00:21:50.010 --> 00:21:51.622
Orhun Parmaksiz: Yep, and then

182
00:21:52.160 --> 00:21:58.999
Gabor Szabo: But yeah, so do I have to fill a hundred there, or just.

183
00:21:59.920 --> 00:22:11.610
Orhun Parmaksiz: No, it's a single constraint. So you get you just have a 25 for this one, and you'll have you need to define another one. So this is just 25,

184
00:22:12.050 --> 00:22:18.659
Orhun Parmaksiz: and put like a comma after this 1st constraint and define another one.

185
00:22:20.505 --> 00:22:21.160
Orhun Parmaksiz: Constraints.

186
00:22:21.620 --> 00:22:27.089
Orhun Parmaksiz: Let's do fill this time not percentage.

187
00:22:27.940 --> 00:22:28.600
Gabor Szabo: Field.

188
00:22:29.080 --> 00:22:31.929
Orhun Parmaksiz: Yeah, fill, just filling the rest of the area.

189
00:22:31.930 --> 00:22:35.270
Gabor Szabo: Or it's a it's a string, or or a word, or what is what is it?

190
00:22:35.270 --> 00:22:41.920
Orhun Parmaksiz: It's the enum variance. So instead of percentage, you just gotta say fail.

191
00:22:42.520 --> 00:22:43.750
Gabor Szabo: Constrained.

192
00:22:44.930 --> 00:22:49.120
Orhun Parmaksiz: No, not not not like that. But you're kind of

193
00:22:49.610 --> 00:22:52.310
Orhun Parmaksiz: just remove one of the percentages.

194
00:22:54.260 --> 00:22:56.920
Gabor Szabo: Oh, so instead of this, okay, I understand. So.

195
00:22:56.920 --> 00:22:57.520
Orhun Parmaksiz: Yep.

196
00:22:58.050 --> 00:22:58.770
Orhun Parmaksiz: Yep.

197
00:22:59.030 --> 00:23:03.010
Gabor Szabo: Okay, Bill, something. There.

198
00:23:03.700 --> 00:23:06.099
Orhun Parmaksiz: And in there you can just say one

199
00:23:09.380 --> 00:23:18.820
Orhun Parmaksiz: and add another fill so constrained. Fill another comma after that and another one.

200
00:23:19.050 --> 00:23:20.820
Orhun Parmaksiz: I will explain this in a second.

201
00:23:23.090 --> 00:23:23.920
Gabor Szabo: Okay.

202
00:23:25.730 --> 00:23:38.190
Orhun Parmaksiz: So right now, we are splitting our current area vertically into 3 parts, because we had. If you remember, we have the CPU disk and then network.

203
00:23:38.560 --> 00:23:40.639
Orhun Parmaksiz: This is not gonna yeah.

204
00:23:40.980 --> 00:23:56.840
Orhun Parmaksiz: And then percentage means 25% of the available area and fill means fill the rest. And when you have like 2 fill constraints. It's just going to take the rest split in, split it into 2 parts and

205
00:23:56.950 --> 00:24:12.320
Orhun Parmaksiz: create 2 separate areas. So if you check the return value of this. This is a layout, right? You need to give your area to this. So after you define your layout, if you go to the end of this

206
00:24:12.580 --> 00:24:17.889
Orhun Parmaksiz: definition to line 43, you can say, dot areas

207
00:24:21.490 --> 00:24:27.960
Orhun Parmaksiz: areas and basically frame frame dot area.

208
00:24:30.350 --> 00:24:37.820
Orhun Parmaksiz: So remove that the frame is already defined. It's a it's a parameter to this function.

209
00:24:37.820 --> 00:24:40.010
Gabor Szabo: Oh, so it's the is the parameter frame. Okay?

210
00:24:40.010 --> 00:24:41.100
Orhun Parmaksiz: Yeah, exactly.

211
00:24:41.920 --> 00:24:45.190
Gabor Szabo: Brim, and it has something here. Area.

212
00:24:45.190 --> 00:24:47.430
Orhun Parmaksiz: Yeah, frame, frame, area. Exactly.

213
00:24:47.970 --> 00:24:49.740
Gabor Szabo: Okay. Now, he didn't like me.

214
00:24:50.360 --> 00:24:54.699
Orhun Parmaksiz: Yeah, because this is a, this is returning a

215
00:24:55.000 --> 00:24:57.660
Orhun Parmaksiz: I think it's a wait. Let me check. What is that?

216
00:24:57.660 --> 00:24:58.530
Gabor Szabo: You shouldn't get.

217
00:24:59.350 --> 00:25:06.660
Orhun Parmaksiz: Yeah, so you kind of need to use brackets there for the return values. So you can say something like.

218
00:25:07.250 --> 00:25:09.399
Orhun Parmaksiz: I'm just going to type it in the chat.

219
00:25:10.480 --> 00:25:16.120
Orhun Parmaksiz: 3, rd you can say something like this instead of layout.

220
00:25:16.750 --> 00:25:20.340
Orhun Parmaksiz: and that way you will be defining 3 layouts.

221
00:25:20.670 --> 00:25:21.620
Gabor Szabo: Oh, okay.

222
00:25:26.690 --> 00:25:30.750
Gabor Szabo: okay, I don't have better idea than what you typed. Okay, so

223
00:25:31.280 --> 00:25:33.740
Gabor Szabo: we can rename it later. We can rename build it.

224
00:25:33.740 --> 00:25:34.390
Orhun Parmaksiz: Yeah.

225
00:25:34.680 --> 00:25:38.869
Gabor Szabo: There was a question whether it works in in older terminals.

226
00:25:39.700 --> 00:25:41.140
Gabor Szabo: Oh, yes.

227
00:25:42.010 --> 00:25:51.429
Orhun Parmaksiz: Yes, it is working in all the terminals. I'm not sure about windows, though I haven't tested it, and we don't have anybody testing windows in our

228
00:25:52.560 --> 00:25:57.610
Orhun Parmaksiz: in our team. So not sure. But yeah, definitely works in all the terminals.

229
00:25:57.840 --> 00:26:02.260
Gabor Szabo: We always want more people on windows testing stuff, because that's always exactly.

230
00:26:03.310 --> 00:26:04.240
Gabor Szabo: Yeah.

231
00:26:05.610 --> 00:26:06.420
Orhun Parmaksiz: Yep, so.

232
00:26:07.125 --> 00:26:07.830
Gabor Szabo: Yeah.

233
00:26:08.360 --> 00:26:15.040
Orhun Parmaksiz: So we have 3 3 layouts, 3 areas or rectangles. Rects. Right?

234
00:26:15.250 --> 00:26:17.790
Orhun Parmaksiz: So let's just render something on each.

235
00:26:18.345 --> 00:26:21.679
Gabor Szabo: Vertically. So 3 columns. Right? Okay?

236
00:26:21.680 --> 00:26:25.630
Orhun Parmaksiz: 3 rows, I think no. 3 columns.

237
00:26:26.120 --> 00:26:32.219
Orhun Parmaksiz: Wait, is it called? It's like, you know, one and the second one on the bottom, and the 3rd one on the like.

238
00:26:32.690 --> 00:26:34.140
Gabor Szabo: Okay, on the bottom as well.

239
00:26:34.530 --> 00:26:40.290
Orhun Parmaksiz: So yeah, let's do frame dot Brander widget.

240
00:26:42.540 --> 00:26:43.280
Gabor Szabo: Sorry.

241
00:26:43.710 --> 00:26:48.580
Orhun Parmaksiz: Let's do frame dot render widget frame is the parameter frame.

242
00:26:49.130 --> 00:26:50.909
Gabor Szabo: Of the the variable frame, right.

243
00:26:50.910 --> 00:26:51.969
Orhun Parmaksiz: Yeah, yeah, exactly.

244
00:26:54.580 --> 00:26:55.450
Gabor Szabo: Okay.

245
00:26:55.700 --> 00:27:03.549
Orhun Parmaksiz: Render widget, and in the widget you can use in a widget that implements the widget trait. So let's just render a block

246
00:27:03.750 --> 00:27:04.940
Orhun Parmaksiz: so block.

247
00:27:07.100 --> 00:27:12.780
Orhun Parmaksiz: Bordered to 2 columns. There.

248
00:27:16.120 --> 00:27:16.970
Gabor Szabo: Okay.

249
00:27:17.370 --> 00:27:21.399
Orhun Parmaksiz: Bordered. Yep, and for the area I just put first.st

250
00:27:23.510 --> 00:27:25.959
Gabor Szabo: Okay, so that's an area. Okay.

251
00:27:25.960 --> 00:27:31.970
Orhun Parmaksiz: Yep, and just copy that line and do it 3 times, and just change the area.

252
00:27:41.110 --> 00:27:42.000
Gabor Szabo: Okay.

253
00:27:42.490 --> 00:27:46.490
Orhun Parmaksiz: All right, so let's run it and see how it looks like.

254
00:27:55.850 --> 00:27:57.849
Gabor Szabo: Okay, it didn't like something. But

255
00:27:58.910 --> 00:28:02.219
Gabor Szabo: okay, so this is these horizontal blocks. Okay.

256
00:28:02.220 --> 00:28:02.960
Orhun Parmaksiz: Yep.

257
00:28:04.050 --> 00:28:08.689
Orhun Parmaksiz: So we're basically gonna fill those blocks with some widgets.

258
00:28:09.160 --> 00:28:11.630
Orhun Parmaksiz: And yeah, that's the goal.

259
00:28:11.970 --> 00:28:15.300
Gabor Szabo: Okay, complaining about parentheses. But okay.

260
00:28:15.690 --> 00:28:19.505
Orhun Parmaksiz: Yeah, that's fine, for now, alright

261
00:28:21.160 --> 00:28:23.700
Orhun Parmaksiz: If you want, you can also split

262
00:28:23.970 --> 00:28:31.570
Orhun Parmaksiz: horizontally and like, for example, you can split the second area horizontally to have 2 blocks next to each other.

263
00:28:32.020 --> 00:28:38.869
Orhun Parmaksiz: But I guess you get the idea. You basically need to do the same thing again. But with layouts horizontal.

264
00:28:39.340 --> 00:28:40.390
Orhun Parmaksiz: So yeah.

265
00:28:40.390 --> 00:28:48.559
Gabor Szabo: Let's let's try to do this. So so I I create the let's say the upper and lower

266
00:28:48.800 --> 00:29:01.088
Gabor Szabo: thing. And I say it's allow layout horizontal. It it already wrote me the constraints percentage I could have written here, I guess, Phil, but

267
00:29:01.580 --> 00:29:02.940
Orhun Parmaksiz: Yeah, that's that's fine.

268
00:29:03.150 --> 00:29:07.060
Gabor Szabo: There's this, I mean, I can calculate up to 100. So it's fine.

269
00:29:07.240 --> 00:29:07.580
Orhun Parmaksiz: Yeah.

270
00:29:07.580 --> 00:29:14.949
Gabor Szabo: A and the areas. And and here it put it in the second, because it hurt you. I guess.

271
00:29:15.370 --> 00:29:19.250
Orhun Parmaksiz: Kind of, yeah, yeah, I think I think that's a

272
00:29:20.180 --> 00:29:24.180
Orhun Parmaksiz: let's why. And it won't do anything for now, because you're not rendering.

273
00:29:24.490 --> 00:29:25.090
Gabor Szabo: Oh, okay.

274
00:29:25.600 --> 00:29:27.970
Orhun Parmaksiz: You need to render the upper and lower.

275
00:29:28.490 --> 00:29:35.760
Gabor Szabo: So I it's I still have. And when when do I render? It doesn't matter if I.

276
00:29:36.100 --> 00:29:39.789
Orhun Parmaksiz: Yeah, it doesn't really matter. You kind of need to remove the

277
00:29:40.490 --> 00:29:43.580
Orhun Parmaksiz: no, it's the same like frame. Render widget.

278
00:29:44.070 --> 00:29:44.940
Orhun Parmaksiz: But for.

279
00:29:44.940 --> 00:29:45.639
Gabor Szabo: Oh, okay.

280
00:29:45.640 --> 00:29:46.230
Orhun Parmaksiz: Just change it.

281
00:29:46.230 --> 00:29:48.679
Gabor Szabo: But here I tell it to render the upper.

282
00:29:49.330 --> 00:29:50.100
Orhun Parmaksiz: Exactly.

283
00:29:52.390 --> 00:29:53.350
Gabor Szabo: Okay.

284
00:29:53.460 --> 00:29:55.944
Gabor Szabo: And why do I need to write so much?

285
00:29:57.240 --> 00:29:59.859
Gabor Szabo: I mean, why don't just render everything.

286
00:30:01.850 --> 00:30:05.240
Orhun Parmaksiz: Yeah, because there's like different ways of rendering it.

287
00:30:05.960 --> 00:30:07.459
Gabor Szabo: Okay, so we have the.

288
00:30:07.460 --> 00:30:15.039
Orhun Parmaksiz: Since, since you're also rendering a second area. You kind of like rendering on top of each other.

289
00:30:15.180 --> 00:30:17.820
Orhun Parmaksiz: you continue to remove line 54.

290
00:30:21.500 --> 00:30:24.950
Gabor Szabo: Because by 1st room.

291
00:30:25.550 --> 00:30:33.419
Orhun Parmaksiz: Your 1st rendering. By the way, it's not upper lower, but should be left and right, I think. But you're kind of rendering

292
00:30:33.540 --> 00:30:36.280
Orhun Parmaksiz: 2 widgets, and also

293
00:30:36.490 --> 00:30:45.279
Orhun Parmaksiz: then you're rendering on top of those again. So the borders just overriding each other. So it looks like that.

294
00:30:46.300 --> 00:30:57.270
Gabor Szabo: So let let me see this. Okay, so if I run this, now, I get something.

295
00:30:58.080 --> 00:30:59.050
Gabor Szabo: Okay?

296
00:30:59.230 --> 00:31:04.609
Gabor Szabo: And yeah, if I remove this this one. You say right.

297
00:31:05.295 --> 00:31:05.980
Orhun Parmaksiz: Yep.

298
00:31:12.980 --> 00:31:17.110
Gabor Szabo: There was a line here, right? It is connected. That's the difference

299
00:31:17.800 --> 00:31:24.700
Gabor Szabo: that now these 2 separate square rectangles, and earlier, if I'm not mistaken.

300
00:31:33.050 --> 00:31:40.429
Gabor Szabo: It had, yeah, okay, so it was connected. And then the the separator was somehow it. It had some spaces interesting. Okay.

301
00:31:41.130 --> 00:31:48.569
Orhun Parmaksiz: Yeah, because you're rendering an area on top of another area. And they both have orders.

302
00:31:48.770 --> 00:31:50.089
Orhun Parmaksiz: So yeah, that's why.

303
00:31:50.920 --> 00:31:53.750
Gabor Szabo: Okay, wait. A second.

304
00:31:54.140 --> 00:31:54.680
Orhun Parmaksiz: Yep.

305
00:32:06.770 --> 00:32:11.619
Gabor Szabo: Okay, someone is asking about hot reload.

306
00:32:12.873 --> 00:32:16.549
Orhun Parmaksiz: You can use cargo watch for hot reloading.

307
00:32:17.158 --> 00:32:21.210
Orhun Parmaksiz: There's 1 parameter that you need to use, though it's called.

308
00:32:21.570 --> 00:32:26.180
Orhun Parmaksiz: Wait, I'm just actually send it to the chat.

309
00:32:30.420 --> 00:32:35.570
Orhun Parmaksiz: Yep, this command, if you run this, if you have a cargo watch installed, it will

310
00:32:35.860 --> 00:32:38.289
Orhun Parmaksiz: reload when you save the code.

311
00:32:52.670 --> 00:32:58.040
Gabor Szabo: Okay, where is the border now to make it?

312
00:32:59.510 --> 00:33:01.269
Gabor Szabo: Okay, let's see.

313
00:33:02.290 --> 00:33:08.550
Gabor Szabo: Now, I I just have to show a little real estate. Now.

314
00:33:13.110 --> 00:33:13.880
Gabor Szabo: yeah.

315
00:33:14.370 --> 00:33:15.280
Gabor Szabo: Bro.

316
00:33:17.190 --> 00:33:24.159
Orhun Parmaksiz: Oh, you, you need to also like, clear it. So you're gonna just do dash, dash clear.

317
00:33:24.570 --> 00:33:28.580
Orhun Parmaksiz: because you need to clear the logs put on the screen.

318
00:33:30.230 --> 00:33:31.050
Orhun Parmaksiz: Sorry.

319
00:33:31.400 --> 00:33:36.090
Gabor Szabo: Dash dish, clear him. Okay?

320
00:33:37.300 --> 00:33:38.240
Gabor Szabo: Good.

321
00:33:46.560 --> 00:33:47.440
Gabor Szabo: Well.

322
00:33:48.100 --> 00:33:49.689
Orhun Parmaksiz: It's not perfect.

323
00:33:50.080 --> 00:33:58.580
Gabor Szabo: Okay, okay, no. I I because of the long, well, sort of, okay, okay, whatever.

324
00:33:58.920 --> 00:34:00.200
Gabor Szabo: We'll keep it.

325
00:34:05.060 --> 00:34:07.390
Gabor Szabo: I added as a comment, here somewhere.

326
00:34:07.610 --> 00:34:08.650
Gabor Szabo: Okay.

327
00:34:16.610 --> 00:34:23.310
Gabor Szabo: okay, we will see if it can be used later.

328
00:34:23.960 --> 00:34:27.589
Orhun Parmaksiz: Yeah. So let's sorry. Go ahead.

329
00:34:27.590 --> 00:34:32.370
Gabor Szabo: Yeah, that was one. I was thinking, if I if I put it in in a separate terminal. But it doesn't really matter.

330
00:34:32.510 --> 00:34:41.480
Gabor Szabo: Anyway, I don't have so much real estate now. So I'm using my my notebook screen and so and large letters large font. So

331
00:34:41.760 --> 00:34:47.389
Gabor Szabo: I will just play play around the with the terminal, enlarging it and making it smaller.

332
00:34:48.219 --> 00:34:49.370
Orhun Parmaksiz: Yeah, okay?

333
00:34:50.929 --> 00:35:01.439
Orhun Parmaksiz: All right. So let's show some data. Let's show how much CPU you're using with some chart widget.

334
00:35:01.649 --> 00:35:11.059
Orhun Parmaksiz: So there's 1 crate that can be helpful here. It's not very hard to use, and I think it's the perfect for this job. It's called Sysinfo.

335
00:35:11.289 --> 00:35:14.299
Orhun Parmaksiz: So bran cargo, add Sysinfo.

336
00:35:16.229 --> 00:35:16.759
Orhun Parmaksiz: Yep.

337
00:35:16.760 --> 00:35:17.300
Gabor Szabo: Yep.

338
00:35:18.190 --> 00:35:19.180
Orhun Parmaksiz: Yep, exactly.

339
00:35:25.660 --> 00:35:45.860
Gabor Szabo: I I mean, I have one big issue with reading documentation on rust, and many times there I see examples. I try them. They don't work, and it takes me a long time till I figure out. Sometimes I even have to ask people that I actually need to enable some feature in the create.

340
00:35:46.840 --> 00:35:52.929
Gabor Szabo: and and I don't know if how other people figure it out faster than I do. I do.

341
00:35:53.850 --> 00:35:56.089
Gabor Szabo: anyway. Okay, I added, it.

342
00:35:57.550 --> 00:35:58.690
Orhun Parmaksiz: Perfect. So

343
00:35:59.390 --> 00:36:07.609
Orhun Parmaksiz: to use this crate, you, you basically need to go to documentation. But I already know. So I'm going to guide you.

344
00:36:08.000 --> 00:36:10.680
Orhun Parmaksiz: we need basically a

345
00:36:12.030 --> 00:36:23.189
Orhun Parmaksiz: system struct that that allows us to interact with the system. And we're going to keep that in the application state. So if you scroll up to the app struct

346
00:36:24.649 --> 00:36:29.939
Orhun Parmaksiz: you can add a new new entry there called System.

347
00:36:32.790 --> 00:36:37.679
Orhun Parmaksiz: And the type is going to be system. This is from Sysinfo.

348
00:36:38.580 --> 00:36:42.370
Gabor Szabo: Oh, so I need to cis entry system right?

349
00:36:43.200 --> 00:36:45.059
Orhun Parmaksiz: Yeah, I think it says infosys.

350
00:36:45.060 --> 00:36:48.030
Gabor Szabo: Or it already, added CC. Info here. No.

351
00:36:48.610 --> 00:36:50.590
Orhun Parmaksiz: Okay, yeah, that that works

352
00:36:50.710 --> 00:37:01.670
Orhun Parmaksiz: alright and to initialize this because I think default won't work. Now, I don't know what default is doing, maybe go to line 21,

353
00:37:02.000 --> 00:37:11.870
Orhun Parmaksiz: and instead of calling default, just initialize with the explicit values so self curly braces.

354
00:37:12.740 --> 00:37:14.809
Orhun Parmaksiz: you can just remove that line, or just.

355
00:37:15.040 --> 00:37:16.380
Gabor Szabo: Do it? Correct? Yeah.

356
00:37:16.660 --> 00:37:20.500
Orhun Parmaksiz: Yeah, yeah, that looks correct to me.

357
00:37:23.340 --> 00:37:29.080
Orhun Parmaksiz: So new, all means, it's gonna fetch all the information that is available.

358
00:37:29.240 --> 00:37:39.109
Orhun Parmaksiz: You can also filter it a bit like you can say something like new CPU, or like memory, and so on. But for now let's just fetch everything

359
00:37:39.240 --> 00:37:46.470
Orhun Parmaksiz: and for rendering, and something like they're in rendering this widget. We need to have some data.

360
00:37:46.570 --> 00:37:52.060
Orhun Parmaksiz: and we need to define this data in the app

361
00:37:52.280 --> 00:37:56.629
Orhun Parmaksiz: again. So if you go to the abstract again.

362
00:37:56.920 --> 00:38:01.840
Orhun Parmaksiz: Let's do something like CPU data, or like CPU.

363
00:38:09.690 --> 00:38:12.100
Orhun Parmaksiz: And this is going to be a vector.

364
00:38:16.860 --> 00:38:21.399
Orhun Parmaksiz: and let's let's do a tuple of 2 floats.

365
00:38:22.760 --> 00:38:29.410
Orhun Parmaksiz: So just the parentheses F, 64 and F. 64. I'll explain this in a second.

366
00:38:36.360 --> 00:38:37.100
Orhun Parmaksiz: Yep.

367
00:38:39.540 --> 00:38:59.569
Orhun Parmaksiz: So we we're gonna render a chart for the CPU right, and the chart consists of X axis and y axis. That's why you have 2 F 64 s. In there. And we're gonna populate that vector in the in the loop. So if you scroll down in the run.

368
00:38:59.570 --> 00:39:01.760
Orhun Parmaksiz: Yeah, we have to add something here as well.

369
00:39:02.320 --> 00:39:06.309
Orhun Parmaksiz: Yeah, no, it's just empty. Let's just start with empty.

370
00:39:06.310 --> 00:39:07.979
Gabor Szabo: Like it. That is okay.

371
00:39:07.980 --> 00:39:08.550
Orhun Parmaksiz: Yep.

372
00:39:10.270 --> 00:39:15.479
Orhun Parmaksiz: So now we're going to fill that struct with, fill that vector with some data.

373
00:39:16.816 --> 00:39:22.399
Orhun Parmaksiz: In self-running after you handle events or before you can.

374
00:39:23.450 --> 00:39:31.499
Orhun Parmaksiz: Yeah, exactly. Just push to that. Vector, but we're not gonna push that data in there. Instead.

375
00:39:32.350 --> 00:39:37.240
Orhun Parmaksiz: we need to push the frame counts and also

376
00:39:37.640 --> 00:39:41.400
Orhun Parmaksiz: wait. Let me just find it. Give me a second.

377
00:39:42.380 --> 00:39:45.835
Gabor Szabo: Okay, I'm just letting the

378
00:39:49.970 --> 00:39:50.770
Orhun Parmaksiz: Alright, we.

379
00:39:51.690 --> 00:39:57.859
Orhun Parmaksiz: The data is system, global CPU usage. So that's that is going to be the second one

380
00:39:58.000 --> 00:39:59.790
Orhun Parmaksiz: for the Y-axis.

381
00:40:00.410 --> 00:40:00.770
Gabor Szabo: Okay.

382
00:40:00.770 --> 00:40:05.600
Orhun Parmaksiz: That's fine, and the 1st one will be the frame. Count.

383
00:40:05.720 --> 00:40:11.990
Orhun Parmaksiz: This is for calculating. You know how how we are like.

384
00:40:12.970 --> 00:40:26.670
Orhun Parmaksiz: how the CPU is changing over time and frame count is not available in the system, but this is actually available in the frame itself. So you kind of need to remove that sorry move that line into the drove call.

385
00:40:26.940 --> 00:40:32.049
Orhun Parmaksiz: If that makes sense, because frame frame count is available in the frame dot count.

386
00:40:32.050 --> 00:40:34.579
Gabor Szabo: Oh, we need, okay.

387
00:40:34.830 --> 00:40:46.050
Orhun Parmaksiz: You can like do it in a draw call. But let's just actually not do that, because draw should be just responsible for drawing stuff. So let's scroll up. Let's do something else.

388
00:40:46.755 --> 00:40:51.659
Orhun Parmaksiz: Scroll up. You see the the self dot draw call right?

389
00:40:51.860 --> 00:40:53.350
Orhun Parmaksiz: There is a closure.

390
00:40:55.990 --> 00:41:01.920
Orhun Parmaksiz: Yeah, he's lying 25. Yeah, you can add like a scope there with curly braces.

391
00:41:02.360 --> 00:41:03.570
Orhun Parmaksiz: Exactly.

392
00:41:04.040 --> 00:41:07.959
Orhun Parmaksiz: And before you draw it you can push

393
00:41:08.350 --> 00:41:14.299
Orhun Parmaksiz: there. And now you have access to frame so frame, dot count should just do it.

394
00:41:19.130 --> 00:41:20.150
Gabor Szabo: Okay.

395
00:41:20.300 --> 00:41:21.859
Orhun Parmaksiz: Is the frame counts. Wait.

396
00:41:23.020 --> 00:41:24.930
Gabor Szabo: Why it doesn't. Why doesn't look like that.

397
00:41:25.130 --> 00:41:31.460
Gabor Szabo: Okay, mismatch type. Okay, that's U size. Expected it found the U size.

398
00:41:32.240 --> 00:41:32.980
Gabor Szabo: So this is.

399
00:41:32.980 --> 00:41:36.900
Orhun Parmaksiz: The of course, use as sorry.

400
00:41:37.080 --> 00:41:38.340
Gabor Szabo: It's the use size.

401
00:41:38.960 --> 00:41:42.580
Orhun Parmaksiz: Yeah. So just convert it to F, 64 by just.

402
00:41:42.580 --> 00:41:42.910
Gabor Szabo: Oh!

403
00:41:42.910 --> 00:41:45.229
Orhun Parmaksiz: Doing as F. 64.

404
00:41:45.230 --> 00:41:46.879
Gabor Szabo: Okay, you prefer that.

405
00:41:47.220 --> 00:41:47.850
Orhun Parmaksiz: Yeah.

406
00:41:48.190 --> 00:41:48.960
Gabor Szabo: Okay.

407
00:41:55.380 --> 00:41:58.370
Orhun Parmaksiz: And this is the same for the other data as well.

408
00:41:59.090 --> 00:41:59.850
Orhun Parmaksiz: All right.

409
00:42:01.200 --> 00:42:03.980
Orhun Parmaksiz: So now we have the CPU data.

410
00:42:04.763 --> 00:42:08.300
Orhun Parmaksiz: Scroll down a bit. Let's render what we have.

411
00:42:11.970 --> 00:42:14.370
Orhun Parmaksiz: So we're gonna render this in the

412
00:42:14.960 --> 00:42:19.129
Orhun Parmaksiz: which area I think the 1st right we call it the first.st

413
00:42:20.090 --> 00:42:23.010
Gabor Szabo: Yeah, we'll see what which one is which at the end.

414
00:42:23.270 --> 00:42:24.316
Orhun Parmaksiz: Yeah. So

415
00:42:26.130 --> 00:42:40.719
Orhun Parmaksiz: let's render there, we're gonna use the charts widget for for this part I would suggest you to go to the Docs. Rs, and take the take like a snippet from from there.

416
00:42:48.040 --> 00:42:52.009
Orhun Parmaksiz: I need to. I guess you need to. Yeah. Change the other one, too. Yep.

417
00:42:57.200 --> 00:42:59.370
Orhun Parmaksiz: So search for charts.

418
00:43:05.790 --> 00:43:06.490
Orhun Parmaksiz: Yep.

419
00:43:15.300 --> 00:43:16.140
Gabor Szabo: Okay.

420
00:43:17.500 --> 00:43:23.630
Orhun Parmaksiz: So yep, you can take that one.

421
00:43:25.420 --> 00:43:28.679
Orhun Parmaksiz: We can define the data sets later.

422
00:43:37.830 --> 00:43:39.609
Gabor Szabo: And where is this charge from.

423
00:43:41.032 --> 00:43:45.050
Orhun Parmaksiz: It's from Rota. 3 widgets. You should probably.

424
00:43:46.530 --> 00:43:47.210
Orhun Parmaksiz: Yeah.

425
00:43:49.960 --> 00:43:51.970
Orhun Parmaksiz: So in widgets exactly.

426
00:43:52.950 --> 00:44:02.680
Gabor Szabo: Hmm, and the data set is is what is the is the application.

427
00:44:03.250 --> 00:44:04.490
Gabor Szabo: The CPU.

428
00:44:05.150 --> 00:44:08.370
Orhun Parmaksiz: No, let's define it. We need to

429
00:44:08.550 --> 00:44:13.159
Orhun Parmaksiz: to find out, too. If you go back to the docs again you'll see that.

430
00:44:14.010 --> 00:44:15.170
Gabor Szabo: This is what it says.

431
00:44:18.320 --> 00:44:23.320
Orhun Parmaksiz: I don't think that's that's gonna work. You need to define the data set. So scroll up.

432
00:44:24.560 --> 00:44:28.699
Orhun Parmaksiz: You see that there's let's data sets.

433
00:44:29.120 --> 00:44:29.930
Gabor Szabo: Okay.

434
00:44:30.340 --> 00:44:35.040
Orhun Parmaksiz: So just take one of them. This defines 2 data sets.

435
00:44:36.040 --> 00:44:43.240
Orhun Parmaksiz: Yep, just copy that one exactly.

436
00:44:47.610 --> 00:44:48.530
Gabor Szabo: Okay.

437
00:44:49.180 --> 00:44:50.030
Orhun Parmaksiz: And.

438
00:44:50.540 --> 00:44:51.070
Gabor Szabo: Yeah.

439
00:44:52.560 --> 00:44:55.370
Orhun Parmaksiz: Formatting, or what happens.

440
00:44:55.370 --> 00:44:58.500
Gabor Szabo: Yeah, no, no, just it's okay.

441
00:44:59.080 --> 00:45:01.809
Gabor Szabo: And you, we need only one data set here.

442
00:45:01.970 --> 00:45:02.610
Orhun Parmaksiz: Yep.

443
00:45:03.520 --> 00:45:06.889
Gabor Szabo: And the the data is the is the vector then right.

444
00:45:07.250 --> 00:45:08.060
Orhun Parmaksiz: Exactly.

445
00:45:09.290 --> 00:45:14.999
Gabor Szabo: So it's a, is it? Good?

446
00:45:15.550 --> 00:45:15.930
Orhun Parmaksiz: So.

447
00:45:15.930 --> 00:45:17.350
Gabor Szabo: Self- self

448
00:45:18.220 --> 00:45:22.990
Gabor Szabo: and CPU, and it wants to clone it. I don't know if it, if you want it or not.

449
00:45:23.980 --> 00:45:28.419
Orhun Parmaksiz: Should be fine. You need to close that. Vector it's a bracket.

450
00:45:29.310 --> 00:45:30.000
Gabor Szabo: Yeah.

451
00:45:36.570 --> 00:45:37.440
Orhun Parmaksiz: Alright!

452
00:45:37.440 --> 00:45:40.970
Gabor Szabo: So where is data set from? It's also a widget.

453
00:45:42.619 --> 00:45:45.310
Orhun Parmaksiz: Yes, if I remember correctly, it should be a widget.

454
00:45:46.470 --> 00:45:47.250
Gabor Szabo: Okay.

455
00:45:51.900 --> 00:45:53.020
Gabor Szabo: okay.

456
00:45:56.060 --> 00:45:57.650
Gabor Szabo: So now

457
00:45:58.450 --> 00:46:05.430
Gabor Szabo: there are the symbols as well. How can I? Can I tell it somehow to edit to import it to.

458
00:46:06.958 --> 00:46:08.510
Orhun Parmaksiz: I think there should be a

459
00:46:09.530 --> 00:46:13.630
Orhun Parmaksiz: way to do it. Do you use like any rust? Lsp.

460
00:46:14.770 --> 00:46:16.900
Orhun Parmaksiz: trust analyzer, or something like that?

461
00:46:16.900 --> 00:46:22.210
Gabor Szabo: Yeah, it uses it. I think I mean, that's how it tells me to that. It has an issue.

462
00:46:23.500 --> 00:46:25.900
Orhun Parmaksiz: So there should be some kind of code actions.

463
00:46:25.900 --> 00:46:28.830
Gabor Szabo: Oh, it's like, quick fix. Okay, here's the quick fix.

464
00:46:31.190 --> 00:46:32.790
Gabor Szabo: What is the quick fix

465
00:46:35.850 --> 00:46:37.930
Gabor Szabo: import data to symbols.

466
00:46:38.170 --> 00:46:41.150
Gabor Szabo: Okay, good.

467
00:46:42.230 --> 00:46:43.590
Gabor Szabo: And the graph type.

468
00:46:43.900 --> 00:46:47.869
Gabor Szabo: Quick fix import rather to a Widget graph type. Okay?

469
00:46:48.750 --> 00:46:53.940
Gabor Szabo: And the style import style.

470
00:46:54.950 --> 00:46:57.070
Gabor Szabo: Not Ratatouille. Style.

471
00:46:59.375 --> 00:46:59.780
Orhun Parmaksiz: Yep.

472
00:47:01.180 --> 00:47:13.069
Gabor Szabo: Let's see what Rotatouille style. Okay, there are 2 styles, and then the clone so expected reference.

473
00:47:14.300 --> 00:47:19.640
Gabor Szabo: and it got a clone. So I this clone oops.

474
00:47:22.010 --> 00:47:26.630
Gabor Szabo: Instead of that, we need the reference to it. Right? Do I understand it correctly?

475
00:47:26.920 --> 00:47:28.950
Gabor Szabo: Yeah, it seems so. Okay.

476
00:47:28.950 --> 00:47:32.639
Orhun Parmaksiz: Sorry I got disconnected should be back now.

477
00:47:34.740 --> 00:47:43.459
Gabor Szabo: Yeah, yeah, okay, yeah, yeah. I can hear you. You didn't. You didn't much. Just I replaced the vector, cloning with an ampersand here.

478
00:47:43.910 --> 00:47:44.359
Orhun Parmaksiz: Okay.

479
00:47:44.360 --> 00:47:45.010
Gabor Szabo: Beef.

480
00:47:45.280 --> 00:47:49.460
Orhun Parmaksiz: Cool. Now we need to define the axis values

481
00:47:51.050 --> 00:47:59.649
Orhun Parmaksiz: for those. Just go ahead and do something like, let X access equals access default.

482
00:48:02.690 --> 00:48:03.410
Gabor Szabo: Dory.

483
00:48:03.810 --> 00:48:04.630
Orhun Parmaksiz: Taxes.

484
00:48:05.850 --> 00:48:06.640
Orhun Parmaksiz: Joseph.

485
00:48:06.640 --> 00:48:06.980
Gabor Szabo: Makes sense.

486
00:48:06.980 --> 00:48:10.390
Orhun Parmaksiz: Struct, yeah, just a struct name like Axis.

487
00:48:11.700 --> 00:48:12.540
Gabor Szabo: Like this.

488
00:48:12.820 --> 00:48:19.150
Orhun Parmaksiz: Yep, default and let's do.

489
00:48:19.300 --> 00:48:23.919
Orhun Parmaksiz: Let's set some bounce for this axis. So dot bounce.

490
00:48:33.930 --> 00:48:43.210
Orhun Parmaksiz: and this takes a 2 stakes, 2 values or like not 2, but array.

491
00:48:44.650 --> 00:48:50.010
Orhun Parmaksiz: So use brackets, and we're gonna start from 0.

492
00:48:50.790 --> 00:48:51.950
Gabor Szabo: Like this right?

493
00:48:51.950 --> 00:49:00.330
Orhun Parmaksiz: Yes, 0, and exactly, not 100, but self. CPU data.

494
00:49:02.640 --> 00:49:05.390
Gabor Szabo: Self CPU.

495
00:49:05.720 --> 00:49:09.599
Orhun Parmaksiz: So CPU and LAN, just the length of the CPU.

496
00:49:10.610 --> 00:49:12.769
Orhun Parmaksiz: Sorry not data. Just land.

497
00:49:12.770 --> 00:49:22.860
Gabor Szabo: Okay, okay, what's what's complaining with this touch? By F 64,

498
00:49:23.720 --> 00:49:26.480
Gabor Szabo: does it? Does it make it happy? Yeah. And this one.

499
00:49:26.880 --> 00:49:30.670
Orhun Parmaksiz: Yeah, just the emergency. F, 64, 2.

500
00:49:31.790 --> 00:49:34.279
Gabor Szabo: The SF. 64. Right.

501
00:49:34.930 --> 00:49:35.850
Orhun Parmaksiz: I have.

502
00:49:36.330 --> 00:49:42.569
Gabor Szabo: Diabete. Okay, good. And then the the Y-axis.

503
00:49:43.800 --> 00:49:47.490
Orhun Parmaksiz: Yeah, just do the same. But it's gonna be 0 and 100.

504
00:49:51.890 --> 00:49:55.709
Gabor Szabo: A y-axis, and then here it's going to be

505
00:49:56.920 --> 00:50:00.110
Gabor Szabo: 100. I guess it's also F 64.

506
00:50:01.410 --> 00:50:02.710
Gabor Szabo: Okay.

507
00:50:03.720 --> 00:50:04.799
Orhun Parmaksiz: Alright! Just run it.

508
00:50:04.800 --> 00:50:11.390
Gabor Szabo: Let's clean it up a little bit and cargo run.

509
00:50:25.140 --> 00:50:26.400
Gabor Szabo: Do B.

510
00:50:26.900 --> 00:50:29.550
Orhun Parmaksiz: We if we didn't render it. That's why.

511
00:50:29.550 --> 00:50:31.509
Gabor Szabo: You don't use charts.

512
00:50:32.260 --> 00:50:33.370
Gabor Szabo: Okay?

513
00:50:37.080 --> 00:50:40.259
Gabor Szabo: So yeah, we gotta render it in.

514
00:50:42.260 --> 00:50:46.430
Gabor Szabo: So we we put it in one of them. The the yeah, instead of.

515
00:50:46.970 --> 00:50:49.060
Orhun Parmaksiz: Let's do it in the front. 1st area.

516
00:50:49.060 --> 00:50:53.720
Gabor Szabo: 1st one. So we I copy this one like, and and then render.

517
00:50:53.720 --> 00:50:58.150
Orhun Parmaksiz: No you. You need to change the block

518
00:51:02.164 --> 00:51:07.499
Orhun Parmaksiz: you can delete that. Change the block bordered Widget to just chart

519
00:51:12.080 --> 00:51:14.119
Orhun Parmaksiz: online 86.

520
00:51:14.940 --> 00:51:17.419
Orhun Parmaksiz: Just remove that, and just say chart.

521
00:51:18.770 --> 00:51:21.769
Gabor Szabo: And that's what that was the name of the variable that we created.

522
00:51:22.210 --> 00:51:24.850
Orhun Parmaksiz: Yeah, it's down there.

523
00:51:25.430 --> 00:51:26.320
Gabor Szabo: Oh, okay.

524
00:51:27.335 --> 00:51:28.060
Orhun Parmaksiz: Sure.

525
00:51:28.720 --> 00:51:29.260
Orhun Parmaksiz: Yep.

526
00:51:30.190 --> 00:51:31.200
Gabor Szabo: Okay.

527
00:51:34.540 --> 00:51:36.740
Orhun Parmaksiz: Alright! Just let's run it.

528
00:51:43.260 --> 00:51:47.139
Gabor Szabo: Okay, let's let me try.

529
00:51:50.700 --> 00:51:53.240
Orhun Parmaksiz: So. Can you try something for me?

530
00:51:53.700 --> 00:51:54.360
Gabor Szabo: Yeah.

531
00:51:54.540 --> 00:51:56.630
Orhun Parmaksiz: Just press any key.

532
00:52:00.465 --> 00:52:03.739
Gabor Szabo: Pressing. HHHJ.

533
00:52:04.670 --> 00:52:09.640
Gabor Szabo: Okay, every time I press there is a dot new dot.

534
00:52:10.620 --> 00:52:14.089
Orhun Parmaksiz: Yeah. Can you guess what's happening here, or can anybody guess?

535
00:52:14.560 --> 00:52:18.439
Orhun Parmaksiz: No, the frame that's the the re-rendering the frame right.

536
00:52:18.830 --> 00:52:23.559
Gabor Szabo: Every time. Every I mean the while loop. Only only the

537
00:52:24.380 --> 00:52:32.539
Gabor Szabo: yeah. It's waiting for the key key. Input, okay? And then the that loop the while loop that should should re-render the new whole frame.

538
00:52:32.760 --> 00:52:35.949
Gabor Szabo: and gets activated only when I press the key.

539
00:52:37.180 --> 00:52:38.200
Orhun Parmaksiz: Exactly.

540
00:52:38.430 --> 00:52:40.380
Orhun Parmaksiz: So we're going to change that now.

541
00:52:40.950 --> 00:52:42.540
Gabor Szabo: If you go back.

542
00:52:45.040 --> 00:52:45.890
Orhun Parmaksiz: It's in the.

543
00:52:45.890 --> 00:52:49.400
Gabor Szabo: Just clicked on the queue, and then it wouldn't work for you.

544
00:52:50.350 --> 00:52:50.890
Orhun Parmaksiz: Yeah.

545
00:52:52.730 --> 00:52:53.890
Gabor Szabo: Yeah, okay.

546
00:52:53.890 --> 00:52:55.292
Orhun Parmaksiz: So go to

547
00:52:56.600 --> 00:52:58.699
Gabor Szabo: Wait a second! Wait a second.

548
00:53:00.330 --> 00:53:02.060
Orhun Parmaksiz: Yeah, sure.

549
00:53:18.450 --> 00:53:19.360
Gabor Szabo: Okay.

550
00:53:21.500 --> 00:53:22.320
Gabor Szabo: Good.

551
00:53:24.810 --> 00:53:27.886
Orhun Parmaksiz: Okay, so let's change that

552
00:53:28.480 --> 00:53:31.919
Orhun Parmaksiz: scroll down. There's this handle. Crossover events function.

553
00:53:31.920 --> 00:53:32.700
Gabor Szabo: Yeah.

554
00:53:33.040 --> 00:53:40.910
Orhun Parmaksiz: And if you if you check the docs for the read call, can you just hover

555
00:53:41.010 --> 00:53:43.620
Orhun Parmaksiz: on the read, call events? Read.

556
00:53:46.120 --> 00:53:50.379
Orhun Parmaksiz: yeah, it says, Yeah, it's blocking the current thread.

557
00:53:50.640 --> 00:54:07.009
Orhun Parmaksiz: So when we need to basically like, yeah, exactly, scroll down a bit. There you go. We need to basically do that here. And we need to 1st poll for new events and then read for events. Because if you call read, it's gonna block

558
00:54:07.200 --> 00:54:27.269
Orhun Parmaksiz: instead, we need to call poll first.st And if it returns a value, we're gonna just read the event and poll takes a duration. So it waits for 100 ms. For example, in this case you can. You can tweak that you can like, make it 60 fps, and so on.

559
00:54:27.420 --> 00:54:35.479
Orhun Parmaksiz: But let's just keep it simple and just, you know, 100 ms here. So before you match.

560
00:54:35.880 --> 00:54:42.210
Orhun Parmaksiz: you need to do something like events, Paul.

561
00:54:42.510 --> 00:54:43.370
Orhun Parmaksiz: So.

562
00:54:44.240 --> 00:54:44.880
Gabor Szabo: Yeah.

563
00:54:45.280 --> 00:54:47.539
Gabor Szabo: So here before, that is the.

564
00:54:47.540 --> 00:54:51.169
Orhun Parmaksiz: Before that. Just say something like, if events, Paul.

565
00:54:55.980 --> 00:54:56.790
Orhun Parmaksiz: that's dude.

566
00:54:57.080 --> 00:55:02.260
Orhun Parmaksiz: Just I don't. I put the whole match there? No, exactly. Yep.

567
00:55:08.500 --> 00:55:16.700
Orhun Parmaksiz: So now, it's not gonna block. It's actually gonna wait for 100 ms for each each time. So if you run it.

568
00:55:23.260 --> 00:55:25.540
Gabor Szabo: And why is it all flat?

569
00:55:27.590 --> 00:55:29.900
Orhun Parmaksiz: You're not using much. CPU. I don't know.

570
00:55:30.570 --> 00:55:34.960
Gabor Szabo: Yeah, I mean, let's let's just take a look at what the other thing says.

571
00:55:39.480 --> 00:55:43.060
Orhun Parmaksiz: You can actually render the current value somewhere.

572
00:55:43.250 --> 00:55:46.970
Gabor Szabo: I have, like a 7 7 load average here.

573
00:55:47.950 --> 00:55:49.990
Orhun Parmaksiz: Case. Let's see.

574
00:55:54.090 --> 00:55:55.540
Orhun Parmaksiz: probably.

575
00:55:56.185 --> 00:55:59.099
Orhun Parmaksiz: Okay. Can I see the code again? What are you.

576
00:55:59.100 --> 00:55:59.620
Gabor Szabo: Yeah.

577
00:55:59.620 --> 00:56:02.499
Orhun Parmaksiz: Going in the scroll up a bit.

578
00:56:03.520 --> 00:56:05.209
Gabor Szabo: Give me a second, I'll close it.

579
00:56:09.337 --> 00:56:12.290
Orhun Parmaksiz: Scroll up into draw function

580
00:56:16.370 --> 00:56:18.090
Orhun Parmaksiz: a little bit more.

581
00:56:18.460 --> 00:56:19.280
Orhun Parmaksiz: Just want to see like.

582
00:56:19.280 --> 00:56:24.249
Gabor Szabo: Data. It looks like we might not collecting the data correctly or.

583
00:56:26.240 --> 00:56:34.890
Orhun Parmaksiz: I think it's because, yeah, okay, I know why it's happening. You need to refresh the system struct. So.

584
00:56:35.830 --> 00:56:37.710
Gabor Szabo: Read this.

585
00:56:37.830 --> 00:56:41.109
Gabor Szabo: Yeah, I use this script once for something I don't.

586
00:56:41.740 --> 00:56:48.510
Orhun Parmaksiz: Yeah, before you turn on Joe, just to self system, refresh

587
00:56:58.640 --> 00:57:01.659
Orhun Parmaksiz: or refresh CPU, doesn't really matter.

588
00:57:01.660 --> 00:57:02.050
Gabor Szabo: Yeah.

589
00:57:04.770 --> 00:57:16.000
Gabor Szabo: this one. Okay, I see there are various ones. Ref, that's the sad part about copilot. It offered us the one function that didn't exist.

590
00:57:16.350 --> 00:57:16.920
Orhun Parmaksiz: Yeah.

591
00:57:16.920 --> 00:57:25.010
Gabor Szabo: I mean, maybe it should have existed. But it doesn't. Maybe it maybe it used to exist. I don't know. Refresh CPU.

592
00:57:33.830 --> 00:57:36.580
Gabor Szabo: Oh, okay, now it's at least changing.

593
00:57:40.030 --> 00:57:40.650
Orhun Parmaksiz: Yep.

594
00:57:41.410 --> 00:57:48.600
Gabor Szabo: Okay, not much. But maybe if I like, I take another window and start moving it around doesn't

595
00:57:49.480 --> 00:57:55.189
Gabor Szabo: doesn't do much. Okay, I just on the other screen. I try to make some noise. Okay.

596
00:57:57.070 --> 00:58:01.870
Orhun Parmaksiz: So, yeah, I have some ideas to make this look better

597
00:58:02.020 --> 00:58:10.469
Orhun Parmaksiz: right now. We just kind of copied the code from the docs. It's not looking great, so I have some tips for you.

598
00:58:11.040 --> 00:58:16.549
Orhun Parmaksiz: If you go to the draw function, or like the the place where we are creating the chart.

599
00:58:17.100 --> 00:58:19.599
Orhun Parmaksiz: I can go there real quick.

600
00:58:21.970 --> 00:58:24.889
Orhun Parmaksiz: Yeah. So line 77.

601
00:58:27.840 --> 00:58:28.450
Gabor Szabo: Right.

602
00:58:30.100 --> 00:58:31.850
Orhun Parmaksiz: There you can.

603
00:58:32.500 --> 00:58:38.769
Orhun Parmaksiz: Sorry not not there. But on the data set line 66. You can change the marker.

604
00:58:39.290 --> 00:58:41.339
Orhun Parmaksiz: So symbols marker dots.

605
00:58:41.560 --> 00:58:46.820
Orhun Parmaksiz: If you change that to something else, let's do a braille symbol.

606
00:58:47.110 --> 00:58:48.699
Orhun Parmaksiz: Just remove dots.

607
00:58:48.700 --> 00:58:54.809
Gabor Szabo: I was out of focus for a second reading the chat. But okay, sorry. Go say again.

608
00:58:54.810 --> 00:58:57.550
Orhun Parmaksiz: No worries, no worries. Just remove the dots

609
00:58:57.860 --> 00:59:00.579
Orhun Parmaksiz: variant. We're going to change the variance to something else.

610
00:59:02.660 --> 00:59:05.810
Orhun Parmaksiz: Just braille beat.

611
00:59:10.050 --> 00:59:16.199
Orhun Parmaksiz: Yeah, there you go and graph type. Let's do line.

612
00:59:21.300 --> 00:59:22.120
Gabor Szabo: Okay.

613
00:59:22.460 --> 00:59:26.869
Orhun Parmaksiz: And one other thing, 100 ms is

614
00:59:27.030 --> 00:59:34.550
Orhun Parmaksiz: not very smooth. Let's change 100 to 16 in the handle events function.

615
00:59:38.200 --> 00:59:45.079
Orhun Parmaksiz: You might be asking why 16. And the reason is, 16 is usually just 60 fps.

616
00:59:46.260 --> 00:59:51.310
Orhun Parmaksiz: Yeah, there you go. If you run it again, it should look better.

617
01:00:00.500 --> 01:00:01.030
Orhun Parmaksiz: Yep.

618
01:00:03.630 --> 01:00:04.460
Gabor Szabo: Okay?

619
01:00:06.940 --> 01:00:07.920
Gabor Szabo: And it's

620
01:00:09.330 --> 01:00:15.169
Gabor Szabo: but it's not. It is moving out here. So or what? It's showing the whole thing all the time. Right?

621
01:00:15.500 --> 01:00:17.279
Gabor Szabo: Yes, it doesn't.

622
01:00:18.000 --> 01:00:25.420
Gabor Szabo: Yeah, okay, of course, because we are just collecting to the vector we don't. We don't remove from the vector we don't look at the at the

623
01:00:27.550 --> 01:00:30.930
Gabor Szabo: slice of the vector, right? We just look the whole vector.

624
01:00:31.790 --> 01:00:32.440
Orhun Parmaksiz: Yep.

625
01:00:32.960 --> 01:00:40.800
Orhun Parmaksiz: you can kind of tweak that by just taking the last 100 elements or something like that. But you get the idea. This is how it works

626
01:00:43.285 --> 01:00:46.609
Orhun Parmaksiz: to make it even prettier.

627
01:00:47.460 --> 01:00:49.099
Orhun Parmaksiz: If you go to.

628
01:00:50.030 --> 01:00:51.560
Gabor Szabo: Just a second, just a second.

629
01:00:51.560 --> 01:00:52.769
Orhun Parmaksiz: Yep no worries.

630
01:01:10.120 --> 01:01:10.480
Gabor Szabo: Okay.

631
01:01:13.080 --> 01:01:15.179
Orhun Parmaksiz: Yeah. So in the draw function.

632
01:01:16.060 --> 01:01:23.300
Orhun Parmaksiz: you can actually scroll up a little bit. Let's go to. Let's let me shoot. Let me see the chart

633
01:01:24.100 --> 01:01:25.230
Orhun Parmaksiz: definition

634
01:01:26.190 --> 01:01:41.459
Orhun Parmaksiz: down line 77, or something. Yeah. So you can actually attach blocks to widgets. So line 78, we are doing that. We are defining a block and attaching that to chart.

635
01:01:41.660 --> 01:01:45.189
Orhun Parmaksiz: But the block does not have any borders.

636
01:01:45.970 --> 01:01:54.180
Orhun Parmaksiz: so if you change block new to block bordered, you'll have a bordered area for the chart.

637
01:01:55.640 --> 01:02:05.639
Orhun Parmaksiz: So yeah, and remove the new yeah. And instead of charts, let's just say, CPU, maybe.

638
01:02:11.850 --> 01:02:14.980
Gabor Szabo: And I also wanted to change this, to

639
01:02:18.390 --> 01:02:22.220
Gabor Szabo: talk right.

640
01:02:23.460 --> 01:02:24.280
Orhun Parmaksiz: Yeah.

641
01:02:24.480 --> 01:02:25.310
Gabor Szabo: Okay.

642
01:02:26.400 --> 01:02:27.070
Orhun Parmaksiz: All right.

643
01:02:27.470 --> 01:02:31.319
Orhun Parmaksiz: If you run it again you'll see a block surrounding the chart.

644
01:02:45.960 --> 01:02:46.620
Gabor Szabo: Correct.

645
01:02:46.850 --> 01:02:47.969
Gabor Szabo: What do I see?

646
01:02:48.590 --> 01:02:53.620
Orhun Parmaksiz: Now you have a block around the chart before you didn't have it.

647
01:02:54.630 --> 01:02:55.779
Gabor Szabo: The the border.

648
01:02:56.080 --> 01:02:57.049
Orhun Parmaksiz: Yeah. The border.

649
01:02:57.930 --> 01:02:59.470
Gabor Szabo: I didn't have it already. Okay.

650
01:02:59.780 --> 01:03:02.260
Orhun Parmaksiz: So where did it disappear?

651
01:03:04.313 --> 01:03:05.100
Orhun Parmaksiz: You didn't.

652
01:03:05.100 --> 01:03:06.509
Gabor Szabo: Because originally you had.

653
01:03:06.980 --> 01:03:10.530
Gabor Szabo: I guess we we removed it when we replaced.

654
01:03:11.180 --> 01:03:15.660
Gabor Szabo: Yeah, here, where we have replaced it, is the border which is just a new here. Okay.

655
01:03:15.810 --> 01:03:16.360
Orhun Parmaksiz: Yep.

656
01:03:16.940 --> 01:03:17.350
Gabor Szabo: Okay.

657
01:03:17.350 --> 01:03:22.480
Orhun Parmaksiz: So, yeah, and yeah, that's basically the whole flow.

658
01:03:22.981 --> 01:03:25.228
Orhun Parmaksiz: You, if you want to like, add

659
01:03:25.700 --> 01:03:28.380
Orhun Parmaksiz: the memory. That's again a chart.

660
01:03:28.490 --> 01:03:45.890
Orhun Parmaksiz: The disks are a bar chart widget, and you know, network is sparkline widget. But like that, that's just the whole flow. So next time, if you want to like. Add some data. You go ahead and update the abstract, and then you kind of.

661
01:03:46.780 --> 01:03:50.269
Orhun Parmaksiz: you know, define your widget and render it.

662
01:03:50.380 --> 01:03:59.709
Orhun Parmaksiz: And yeah, that's how it works. So let's not actually reiterate that if you want, and I can show you something new.

663
01:04:00.110 --> 01:04:04.799
Orhun Parmaksiz: or I can just take over right now, if you want. It depends on what you want to do.

664
01:04:05.890 --> 01:04:16.960
Gabor Szabo: Okay, I have 1 1 thing that I definitely would like to to know. But let's read what's going on in the chat. Block and charts are all internal widgets of the library correct?

665
01:04:17.090 --> 01:04:21.059
Gabor Szabo: Is it possible to implement custom widgets by implementing a trade.

666
01:04:22.470 --> 01:04:23.890
Orhun Parmaksiz: That's possible.

667
01:04:24.530 --> 01:04:26.280
Orhun Parmaksiz: It's a good question.

668
01:04:26.620 --> 01:04:34.989
Orhun Parmaksiz: Actually, there was a new 3rd party, widget for specifically for this purpose,

669
01:04:36.320 --> 01:04:41.969
Orhun Parmaksiz: came out, just recently, if you go to the link in the chat, you can also check it out.

670
01:04:43.460 --> 01:04:50.400
Orhun Parmaksiz: Yes. So yeah, you can like, implement your own widgets, and you can just add the dependency to your

671
01:04:50.870 --> 01:04:55.269
Orhun Parmaksiz: app and use it as any widget. So yeah.

672
01:04:57.140 --> 01:04:57.920
Gabor Szabo: Nice.

673
01:04:58.520 --> 01:05:20.454
Gabor Szabo: So I wanted to ask you one. I mean, one thing that I was playing around is is having lists of of texts so like where I would like type in some text. It will search in some databases, let's say, and then show me some like a search. Show me top 5, and then I can

674
01:05:20.960 --> 01:05:22.959
Gabor Szabo: pick one of the one of the

675
01:05:24.400 --> 01:05:27.620
Gabor Szabo: answers, and then it would do something else.

676
01:05:29.890 --> 01:05:30.530
Orhun Parmaksiz: Yep.

677
01:05:31.750 --> 01:05:35.060
Orhun Parmaksiz: So sorry.

678
01:05:35.620 --> 01:05:36.290
Gabor Szabo: Yeah, yeah.

679
01:05:38.322 --> 01:05:47.677
Orhun Parmaksiz: So that's the that's that's the functionality that we will implement with the table the processes, actually. But

680
01:05:48.780 --> 01:05:53.609
Orhun Parmaksiz: if you want, I can just take over now, because, you know, we just like kind of

681
01:05:54.370 --> 01:05:57.490
Orhun Parmaksiz: kind of past 1 h. So.

682
01:05:57.490 --> 01:05:58.859
Gabor Szabo: Okay, I just don't like.

683
01:05:58.860 --> 01:06:00.550
Orhun Parmaksiz: Fast forward a little bit.

684
01:06:01.110 --> 01:06:03.289
Gabor Szabo: Okay, go. Sorry.

685
01:06:10.640 --> 01:06:12.739
Gabor Szabo: Okay. I pulled it out. Now.

686
01:06:14.210 --> 01:06:20.260
Gabor Szabo: And I'll give you so would you want us to scare? Share your screen right.

687
01:06:20.890 --> 01:06:21.370
Orhun Parmaksiz: Yep.

688
01:06:21.520 --> 01:06:23.060
Gabor Szabo: That's the idea. Okay.

689
01:06:23.060 --> 01:06:26.099
Orhun Parmaksiz: Yeah, I can just show it to you what you have asked

690
01:06:26.280 --> 01:06:30.140
Orhun Parmaksiz: specifically. Let me just find the repo and clone it. First.st

691
01:06:31.060 --> 01:06:40.109
Orhun Parmaksiz: Okay, share screen desktop one, all right.

692
01:06:41.980 --> 01:06:43.359
Orhun Parmaksiz: You see my screen right.

693
01:06:43.840 --> 01:06:44.730
Gabor Szabo: Yeah.

694
01:06:46.190 --> 01:06:47.870
Orhun Parmaksiz: So cloning that.

695
01:06:55.790 --> 01:06:58.939
Orhun Parmaksiz: So, yeah, this is how it looks like on my computer.

696
01:06:59.330 --> 01:07:04.319
Orhun Parmaksiz: So you said a list of text and searching right.

697
01:07:06.220 --> 01:07:07.320
Gabor Szabo: Yeah.

698
01:07:10.190 --> 01:07:19.030
Orhun Parmaksiz: So let's create a table of processes and and search in them.

699
01:07:20.510 --> 01:07:21.200
Gabor Szabo: Okay.

700
01:07:21.710 --> 01:07:22.349
Orhun Parmaksiz: All right.

701
01:07:23.490 --> 01:07:28.520
Orhun Parmaksiz: So for the processes. Let me actually

702
01:07:29.080 --> 01:07:32.499
Orhun Parmaksiz: go to Sys info cause. I don't remember

703
01:07:33.490 --> 01:07:35.519
Orhun Parmaksiz: how we do that, but it should be

704
01:07:38.420 --> 01:07:42.229
Orhun Parmaksiz: something easy. One second, I'm just checking something in my

705
01:07:42.630 --> 01:07:45.529
Orhun Parmaksiz: other screen. Maybe I have a snippet.

706
01:07:49.600 --> 01:07:50.300
Orhun Parmaksiz: Yep.

707
01:07:51.450 --> 01:07:56.350
Orhun Parmaksiz: So it's basically processes call.

708
01:07:56.600 --> 01:08:01.820
Orhun Parmaksiz: This gives us some data for the

709
01:08:02.090 --> 01:08:06.000
Orhun Parmaksiz: list we we need. We need a table. Actually. So

710
01:08:07.660 --> 01:08:12.909
Orhun Parmaksiz: table. But table and list are just internally very similar.

711
01:08:13.420 --> 01:08:15.510
Orhun Parmaksiz: So it doesn't really matter here.

712
01:08:20.399 --> 01:08:24.870
Orhun Parmaksiz: So instead of the second, let's render it

713
01:08:25.779 --> 01:08:30.200
Orhun Parmaksiz: here on in the the bottom block. So it's the 3rd one.

714
01:08:31.120 --> 01:08:42.779
Orhun Parmaksiz: I'm going to actually create a new function here and render processes should mute self frame frame.

715
01:08:46.100 --> 01:08:46.830
Orhun Parmaksiz: Yeah.

716
01:08:51.069 --> 01:08:53.330
Orhun Parmaksiz: also, it should take an area.

717
01:09:01.130 --> 01:09:11.020
Orhun Parmaksiz: Yep, there you go in here. Let's actually create the data. 1st to do that.

718
01:09:12.060 --> 01:09:20.359
Orhun Parmaksiz: So let's do something like, let's mute data empty.

719
01:09:23.370 --> 01:09:24.729
Orhun Parmaksiz: Yeah, why not?

720
01:09:26.910 --> 01:09:30.330
Orhun Parmaksiz: So gets processes. Pid process.

721
01:09:32.407 --> 01:09:34.310
Orhun Parmaksiz: What is? Why is it complaining.

722
01:09:42.930 --> 01:09:48.080
Orhun Parmaksiz: Yeah, that's why I don't like copilot. It's just not great for these cases.

723
01:09:49.001 --> 01:09:56.210
Orhun Parmaksiz: The the data is right now, a, vector, so for the table, we need to create some rows.

724
01:09:57.800 --> 01:10:00.030
Orhun Parmaksiz: as you can see from here

725
01:10:02.780 --> 01:10:18.900
Orhun Parmaksiz: to do that. Just row new row new with a vector and PID,

726
01:10:19.280 --> 01:10:21.390
Orhun Parmaksiz: name, let's just do that for now.

727
01:10:30.540 --> 01:10:34.810
Orhun Parmaksiz: Wait. What? Why is it complaining, mute frame.

728
01:10:36.620 --> 01:10:46.590
Gabor Szabo: Now, as I'm thinking, I don't understand why copilot doesn't ask the the internal, the system, I mean

729
01:10:46.980 --> 01:10:54.937
Gabor Szabo: visual studio code knows which functions can I call on on a certain object, and it offer

730
01:10:55.680 --> 01:10:57.459
Gabor Szabo: offered me something else.

731
01:10:58.490 --> 01:11:04.319
Gabor Szabo: So it I mean, it could ask as well, and and say, okay, well.

732
01:11:04.620 --> 01:11:08.369
Gabor Szabo: this one is not not correct because the internal system doesn't know about it.

733
01:11:08.720 --> 01:11:11.620
Gabor Szabo: The the introspection system.

734
01:11:13.820 --> 01:11:15.489
Orhun Parmaksiz: Yeah, it's not great.

735
01:11:16.390 --> 01:11:17.170
Gabor Szabo: Strange

736
01:11:17.500 --> 01:11:23.729
Gabor Szabo: because I I played played around with it today and and it could figure out all kind of things

737
01:11:23.900 --> 01:11:25.420
Gabor Szabo: above my code.

738
01:11:27.090 --> 01:11:27.840
Gabor Szabo: So.

739
01:11:31.810 --> 01:11:40.743
Orhun Parmaksiz: Specifically, we realized it's actually using the description or from

740
01:11:41.550 --> 01:11:44.600
Orhun Parmaksiz: from the types to generate some code.

741
01:11:44.840 --> 01:11:47.600
Orhun Parmaksiz: And sometimes

742
01:11:47.750 --> 01:11:56.299
Orhun Parmaksiz: in in some specific case the description was a bit wrong. That's why it was just generating the wrong code. So we kind of updated the documentation

743
01:11:56.960 --> 01:11:58.319
Orhun Parmaksiz: for the co-pilot.

744
01:11:58.960 --> 01:12:00.030
Gabor Szabo: So, yeah.

745
01:12:02.200 --> 01:12:04.960
Orhun Parmaksiz: So that's how we create a role and

746
01:12:05.570 --> 01:12:08.639
Orhun Parmaksiz: data push. So these are just the rows.

747
01:12:10.750 --> 01:12:13.790
Orhun Parmaksiz: And let's create a table.

748
01:12:15.330 --> 01:12:16.230
Orhun Parmaksiz: Wait.

749
01:12:16.530 --> 01:12:19.549
Orhun Parmaksiz: It's a bit weird. My workspaces.

750
01:12:20.210 --> 01:12:21.960
Orhun Parmaksiz: All right. Let table.

751
01:12:22.210 --> 01:12:23.860
Orhun Parmaksiz: Just table.

752
01:12:33.260 --> 01:12:39.230
Orhun Parmaksiz: So table takes rows and some wits.

753
01:12:39.710 --> 01:12:43.820
Orhun Parmaksiz: So the widths are just constraints again.

754
01:12:45.120 --> 01:12:48.800
Orhun Parmaksiz: it's basically. So now we're just saying a

755
01:12:49.310 --> 01:12:54.170
Orhun Parmaksiz: process. Id should take 10%. But the 10% is not a good

756
01:12:54.590 --> 01:13:03.820
Orhun Parmaksiz: constraint. So let's do minimum of 10 characters, or like maximum of maximum of 10 characters and just fill

757
01:13:05.180 --> 01:13:06.680
Orhun Parmaksiz: like min. 0.

758
01:13:06.820 --> 01:13:13.860
Orhun Parmaksiz: So maximum 10 characters for the pid, a role or column.

759
01:13:14.940 --> 01:13:20.540
Orhun Parmaksiz: it's a it's a column, and just take the rest for for the rest. So yeah.

760
01:13:20.790 --> 01:13:24.669
Orhun Parmaksiz: and then you render it. So here we create a table.

761
01:13:25.440 --> 01:13:28.020
Orhun Parmaksiz: It's bordered lock.

762
01:13:31.320 --> 01:13:35.930
Orhun Parmaksiz: This heather is basically the name of the the role.

763
01:13:38.430 --> 01:13:40.450
Orhun Parmaksiz: Yep, let me try it.

764
01:13:42.190 --> 01:13:46.899
Orhun Parmaksiz: Yeah. So processes, these are just the processes that we have.

765
01:13:47.900 --> 01:13:53.780
Orhun Parmaksiz: And it's not really updating right now, because we're not calling the update method.

766
01:13:54.120 --> 01:13:56.970
Orhun Parmaksiz: Once again, let me just go here.

767
01:13:57.730 --> 01:14:01.000
Orhun Parmaksiz: Self system refresh processes.

768
01:14:01.240 --> 01:14:11.460
Orhun Parmaksiz: This takes a enom process processes the update and remove that processes, processes the updates.

769
01:14:13.020 --> 01:14:17.140
Orhun Parmaksiz: Let's update everything and remove the dead ones.

770
01:14:19.240 --> 01:14:20.170
Orhun Parmaksiz: All right.

771
01:14:20.280 --> 01:14:26.070
Orhun Parmaksiz: They should update. Now, is it updating.

772
01:14:27.940 --> 01:14:29.270
Gabor Szabo: I don't see changing.

773
01:14:29.830 --> 01:14:34.279
Orhun Parmaksiz: Yeah, I don't know what's going wrong.

774
01:14:41.720 --> 01:14:43.749
Orhun Parmaksiz: Wait, let me run it again.

775
01:14:51.490 --> 01:14:52.250
Orhun Parmaksiz: Hmm!

776
01:14:55.780 --> 01:15:00.010
Orhun Parmaksiz: Is it another call? Maybe process?

777
01:15:14.420 --> 01:15:16.619
Orhun Parmaksiz: Oh, is it because I'm not sorting them.

778
01:15:27.080 --> 01:15:34.660
Orhun Parmaksiz: Okay, give me a second. Let me just sort them roll new collect.

779
01:15:42.340 --> 01:15:45.029
Orhun Parmaksiz: So rows sort by.

780
01:15:45.340 --> 01:15:48.709
Orhun Parmaksiz: Actually, let's do CPU

781
01:15:53.520 --> 01:15:54.720
Orhun Parmaksiz: CPU.

782
01:16:16.720 --> 01:16:18.499
Orhun Parmaksiz: Now, we can sort.

783
01:16:20.350 --> 01:16:23.010
Orhun Parmaksiz: Yeah, we are sourcing by CPU now.

784
01:16:23.230 --> 01:16:25.229
Orhun Parmaksiz: So you see that it's updating.

785
01:16:26.140 --> 01:16:31.040
Orhun Parmaksiz: It's not great, though, because it's like updating every 60 ms.

786
01:16:32.405 --> 01:16:32.860
Orhun Parmaksiz: So

787
01:16:36.100 --> 01:16:38.680
Orhun Parmaksiz: you can also, like, render the CPU here.

788
01:16:40.980 --> 01:16:43.160
Orhun Parmaksiz: But we are not doing that.

789
01:16:44.170 --> 01:16:45.240
Orhun Parmaksiz: CPU,

790
01:16:50.830 --> 01:16:56.190
Orhun Parmaksiz: you get the idea right? You basically like, add some data, render it

791
01:16:56.290 --> 01:17:00.170
Orhun Parmaksiz: change. Constraints, define layouts, and so on. This is basically the flow.

792
01:17:00.460 --> 01:17:01.370
Orhun Parmaksiz: There you go.

793
01:17:03.390 --> 01:17:05.000
Orhun Parmaksiz: But this is too fast.

794
01:17:05.210 --> 01:17:10.400
Orhun Parmaksiz: So let me do something like

795
01:17:18.190 --> 01:17:25.280
Orhun Parmaksiz: something like this here. So if frame count.

796
01:17:27.270 --> 01:17:34.259
Orhun Parmaksiz: So every 60 frames we update the wait. This is wrong.

797
01:17:36.250 --> 01:17:40.550
Orhun Parmaksiz: Refresh CPU, refresh.

798
01:17:47.730 --> 01:17:48.490
Orhun Parmaksiz: Yep.

799
01:17:48.970 --> 01:17:55.230
Orhun Parmaksiz: So every 60 frames we update the CPU.

800
01:17:55.410 --> 01:18:00.549
Orhun Parmaksiz: But for each frame we update the sorry

801
01:18:01.450 --> 01:18:10.249
Orhun Parmaksiz: for for every 60 frame we update the processes, but for each frame you update the CPU, so the processes should just

802
01:18:10.840 --> 01:18:16.399
Orhun Parmaksiz: be updated less frequently. Now, does that make sense.

803
01:18:16.920 --> 01:18:17.960
Gabor Szabo: Yeah, yeah.

804
01:18:18.550 --> 01:18:19.880
Orhun Parmaksiz: Okay, great.

805
01:18:20.870 --> 01:18:29.730
Orhun Parmaksiz: So yeah. Now, we have a table and to add interactivity to the table. We need to

806
01:18:30.360 --> 01:18:36.760
Orhun Parmaksiz: define a Table State. Because, let's say, we are running this. But we want to like select one of the items.

807
01:18:36.760 --> 01:18:38.109
Gabor Szabo: Committed, please.

808
01:18:39.130 --> 01:18:39.820
Orhun Parmaksiz: Sorry.

809
01:18:40.519 --> 01:18:42.439
Gabor Szabo: Could. Could you just make a commit? Now?

810
01:18:43.286 --> 01:18:47.719
Gabor Szabo: You won't be able to push it out because you're okay later on, whatever.

811
01:18:48.822 --> 01:18:52.369
Orhun Parmaksiz: Just you can just add me to the repo. Now, if you want.

812
01:18:53.340 --> 01:18:59.110
Gabor Szabo: Yeah, I'll give. I'll I'll yeah, just a second. Okay, so just comment. And I'll figure it out. In the meantime.

813
01:19:00.380 --> 01:19:05.450
Orhun Parmaksiz: Okay and processes.

814
01:19:19.370 --> 01:19:22.670
Gabor Szabo: Not that easy we have to authenticate and whatnot.

815
01:19:23.070 --> 01:19:25.030
Orhun Parmaksiz: I can send you a Pr.

816
01:19:27.730 --> 01:19:28.860
Gabor Szabo: No, it's okay.

817
01:19:29.600 --> 01:19:32.380
Gabor Szabo: I just, is this what you.

818
01:19:32.380 --> 01:19:33.040
Orhun Parmaksiz: Prefer.

819
01:19:33.890 --> 01:19:35.579
Gabor Szabo: No, no, what's your username.

820
01:19:40.590 --> 01:19:42.659
Gabor Szabo: what is your github username

821
01:19:47.390 --> 01:19:49.669
Orhun Parmaksiz: Can you show code? Again with the table?

822
01:19:50.120 --> 01:19:50.780
Orhun Parmaksiz: Yeah.

823
01:19:57.460 --> 01:19:58.220
Gabor Szabo: Okay.

824
01:19:58.750 --> 01:19:59.640
Gabor Szabo: I sent you.

825
01:19:59.640 --> 01:20:02.210
Orhun Parmaksiz: Function is not great, but like

826
01:20:02.610 --> 01:20:06.510
Orhun Parmaksiz: sorry I was not hearing, so were you saying saying something.

827
01:20:07.180 --> 01:20:12.000
Gabor Szabo: That I figured it out, and I sent you a collaboration. Whatever dingy.

828
01:20:12.290 --> 01:20:16.109
Orhun Parmaksiz: Sorry my my headphone died for a second. So sorry.

829
01:20:16.350 --> 01:20:18.059
Gabor Szabo: No problem, no problem.

830
01:20:18.590 --> 01:20:19.500
Orhun Parmaksiz: Alright!

831
01:20:26.700 --> 01:20:27.880
Orhun Parmaksiz: Alright! Pushed.

832
01:20:30.870 --> 01:20:35.310
Gabor Szabo: What did you do? I mean? 1st it gave you access, deny, and then.

833
01:20:37.631 --> 01:20:39.340
Orhun Parmaksiz: Github is just slow, probably.

834
01:20:40.020 --> 01:20:40.639
Gabor Szabo: Oh, okay.

835
01:20:44.100 --> 01:20:48.729
Orhun Parmaksiz: But yeah, this sort source is not great, but works. So yeah.

836
01:20:50.150 --> 01:20:54.310
Orhun Parmaksiz: alright. So let's add some interactivity to the table.

837
01:20:55.420 --> 01:20:58.870
Orhun Parmaksiz: Let's say you want to like. Select something right, or.

838
01:20:59.410 --> 01:21:05.140
Orhun Parmaksiz: you know, like highlight, something here to do that

839
01:21:05.360 --> 01:21:07.949
Orhun Parmaksiz: you need to define a table state.

840
01:21:11.040 --> 01:21:18.389
Orhun Parmaksiz: This is once again from Reciti. The Table State Table State should be just default.

841
01:21:24.430 --> 01:21:30.030
Orhun Parmaksiz: Now, to change the selected item in the table.

842
01:21:30.720 --> 01:21:35.170
Orhun Parmaksiz: you need to actually add a new key here

843
01:21:35.270 --> 01:21:44.299
Orhun Parmaksiz: say something like, yeah, exactly so, modifiers just do that.

844
01:21:45.470 --> 01:21:53.130
Orhun Parmaksiz: So table State select next with, select the next item similarly

845
01:21:53.810 --> 01:21:57.050
Orhun Parmaksiz: select previous with, select the previous item.

846
01:21:57.530 --> 01:22:04.879
Orhun Parmaksiz: and to attach that table state to the table. Widget, you need to change something

847
01:22:06.020 --> 01:22:21.180
Orhun Parmaksiz: in the in the render call. So now, since you have a state for the table. This is no longer a render widget, but a render stateful widget, because we want to give the state to the table

848
01:22:21.650 --> 01:22:26.430
Orhun Parmaksiz: and to do that basically render statefulvisit. You give the table

849
01:22:26.740 --> 01:22:35.299
Orhun Parmaksiz: area and whatever state you have, which is the table State. So now, if I rerun this and press some keys.

850
01:22:36.110 --> 01:22:42.650
Orhun Parmaksiz: You are actually selecting it, but you're not seeing it, because we're not changing the style of the selected ones.

851
01:22:43.150 --> 01:22:52.019
Orhun Parmaksiz: So selected or highlight highlight row highlight style.

852
01:22:53.070 --> 01:22:54.860
Orhun Parmaksiz: Let's do something like this.

853
01:22:55.860 --> 01:23:05.890
Orhun Parmaksiz: You can just basically give it a color, too. So color red, for example, it's from reside color.

854
01:23:06.180 --> 01:23:11.160
Orhun Parmaksiz: which is, make the selected one red in

855
01:23:11.610 --> 01:23:16.300
Orhun Parmaksiz: normal circumstances. But it's not working. Wait, what's going on?

856
01:23:17.920 --> 01:23:21.580
Orhun Parmaksiz: Let me actually change this to something else.

857
01:23:26.150 --> 01:23:35.159
Orhun Parmaksiz: Okay, to to set, not not a style, but like a symbol. Maybe

858
01:23:36.190 --> 01:23:39.379
Orhun Parmaksiz: it would be an easier way to debug this.

859
01:23:40.580 --> 01:23:41.910
Orhun Parmaksiz: Okay, it's not working.

860
01:23:42.120 --> 01:23:43.380
Orhun Parmaksiz: Give me a second.

861
01:23:43.530 --> 01:23:45.538
Orhun Parmaksiz: I gotta figure this out

862
01:23:46.220 --> 01:23:46.880
Orhun Parmaksiz: So.

863
01:23:46.880 --> 01:23:51.340
Gabor Szabo: Highlight means that there's the selected one. So how do? How does it look.

864
01:23:53.760 --> 01:23:54.460
Orhun Parmaksiz: Sorry.

865
01:23:55.600 --> 01:23:59.730
Gabor Szabo: Yeah, I'm trying to figure out. Understand that? Oh, okay, it's working.

866
01:23:59.730 --> 01:24:06.120
Orhun Parmaksiz: So, yeah, I I'm now selecting the 1st one before I run the app.

867
01:24:06.230 --> 01:24:08.769
Orhun Parmaksiz: you can call table state select

868
01:24:08.970 --> 01:24:15.020
Orhun Parmaksiz: and just index. So then you, you're basically like highlighting it.

869
01:24:17.040 --> 01:24:20.209
Orhun Parmaksiz: But for some reason the keys didn't work.

870
01:24:21.390 --> 01:24:23.579
Orhun Parmaksiz: Deal state select next.

871
01:24:24.050 --> 01:24:29.600
Orhun Parmaksiz: Oh, J, and Q, okay, yeah. I was just pressing one up and down instead. So yeah.

872
01:24:30.540 --> 01:24:39.000
Orhun Parmaksiz: it happens. So now, you can like select items in the table using table state.

873
01:24:41.540 --> 01:24:44.129
Orhun Parmaksiz: All right, I'm gonna commit this to.

874
01:24:45.300 --> 01:24:46.699
Gabor Szabo: Nice, very nice.

875
01:24:55.610 --> 01:25:04.250
Orhun Parmaksiz: Oops, so for the other inputs, like the you know, text input and so on.

876
01:25:04.910 --> 01:25:06.149
Orhun Parmaksiz: What do we do?

877
01:25:08.120 --> 01:25:14.120
Orhun Parmaksiz: Well, you can use the cross term calls, like the

878
01:25:14.600 --> 01:25:22.469
Orhun Parmaksiz: the Underlying Cross term library to handle the key ones and render something like, you know, you can just

879
01:25:22.940 --> 01:25:27.879
Orhun Parmaksiz: create a custom text input, but it's gonna be a bit of a

880
01:25:31.480 --> 01:25:38.940
Orhun Parmaksiz: bit of a like you. You'll add some complexity to your app. So I would actually recommend using 3

881
01:25:40.840 --> 01:25:42.180
Orhun Parmaksiz: to a text area.

882
01:25:42.180 --> 01:25:44.529
Gabor Szabo: Pop pop things like, for example, right?

883
01:25:44.680 --> 01:25:45.820
Gabor Szabo: Or that's what you mean.

884
01:25:45.820 --> 01:25:46.450
Orhun Parmaksiz: Sorry.

885
01:25:47.410 --> 01:25:51.710
Gabor Szabo: I saw this like, for example, a pop up thing that like shows

886
01:25:52.750 --> 01:25:55.970
Gabor Szabo: above the rest of the the Widget, something.

887
01:25:56.930 --> 01:26:01.700
Orhun Parmaksiz: Yeah, so this 3 text area is basically.

888
01:26:01.700 --> 01:26:02.660
Gabor Szabo: Basically.

889
01:26:02.660 --> 01:26:03.879
Orhun Parmaksiz: What were you? Gonna implement.

890
01:26:03.880 --> 01:26:04.810
Gabor Szabo: For us.

891
01:26:10.030 --> 01:26:10.580
Orhun Parmaksiz: Yep.

892
01:26:11.450 --> 01:26:14.960
Orhun Parmaksiz: So this is another recipe. Widget.

893
01:26:15.800 --> 01:26:18.320
Orhun Parmaksiz: To add this to your app.

894
01:26:19.750 --> 01:26:23.889
Orhun Parmaksiz: Let me go to the minimal example. Well, it's not found here one sec.

895
01:26:34.320 --> 01:26:37.380
Orhun Parmaksiz: So cargo, add to the text area.

896
01:26:41.480 --> 01:26:43.710
Orhun Parmaksiz: Let me see if it's building. Okay.

897
01:26:56.160 --> 01:27:03.240
Orhun Parmaksiz: alright. So we can create a text area pretty much like this.

898
01:27:04.760 --> 01:27:09.910
Orhun Parmaksiz: and then we can just render it in some area. So let's decide where to render this thing.

899
01:27:11.550 --> 01:27:16.059
Orhun Parmaksiz: Any ideas you we want to like, search, or what do you want to do here?

900
01:27:17.250 --> 01:27:26.520
Gabor Szabo: Yeah. So I mean, I didn't think about this one. I had my own text, and I wanted to search. But yeah, I wanted to search something, and

901
01:27:27.780 --> 01:27:32.039
Gabor Szabo: like, like, have some place place whereby I can type in some text

902
01:27:32.140 --> 01:27:34.400
Gabor Szabo: and enter. And then it will search.

903
01:27:34.560 --> 01:27:37.419
Gabor Szabo: And but but basically, it's yeah.

904
01:27:40.140 --> 01:27:48.999
Gabor Szabo: just the inputting. The text is already enough. I guess the rest is the rest. I can. I guess we can figure it out. We just

905
01:27:49.220 --> 01:27:53.090
Gabor Szabo: have the table, have new list of values.

906
01:27:54.880 --> 01:27:55.460
Orhun Parmaksiz: Yep.

907
01:27:56.510 --> 01:27:57.700
Orhun Parmaksiz: So

908
01:28:02.500 --> 01:28:08.240
Orhun Parmaksiz: yeah, I'm just thinking, if we need 3 texture for this to be honest, because it's a bit like an overkill.

909
01:28:09.210 --> 01:28:12.970
Orhun Parmaksiz: We don't need this much of text. Right?

910
01:28:13.960 --> 01:28:16.110
Orhun Parmaksiz: Yeah, the search. Actually.

911
01:28:16.300 --> 01:28:21.278
Orhun Parmaksiz: something like this, a single line. So okay, let's just yeah. Let's just do it.

912
01:28:22.240 --> 01:28:24.309
Orhun Parmaksiz: me go back to the minimal again.

913
01:28:32.010 --> 01:28:35.420
Orhun Parmaksiz: Let me render it. After we render the table.

914
01:28:36.170 --> 01:28:38.420
Orhun Parmaksiz: I want to actually create a new function here.

915
01:28:39.100 --> 01:28:45.670
Orhun Parmaksiz: Surrender processes, render search.

916
01:28:50.080 --> 01:28:53.359
Orhun Parmaksiz: create a new text area.

917
01:28:54.850 --> 01:29:01.340
Orhun Parmaksiz: Is there a way to make this one line sets block

918
01:29:10.150 --> 01:29:14.229
Orhun Parmaksiz: oops? Self render search?

919
01:29:15.610 --> 01:29:18.459
Orhun Parmaksiz: Actually, it's better to call it here. Yeah.

920
01:29:21.350 --> 01:29:22.240
Orhun Parmaksiz: Turned.

921
01:29:32.400 --> 01:29:35.129
Orhun Parmaksiz: Let me see what happens when we run this.

922
01:29:37.780 --> 01:29:40.060
Orhun Parmaksiz: Now we have a cursor here

923
01:29:41.370 --> 01:29:43.420
Orhun Parmaksiz: which is not what we want.

924
01:29:44.930 --> 01:29:46.470
Orhun Parmaksiz: Let's do something like.

925
01:29:47.870 --> 01:29:53.469
Gabor Szabo: I mean, I saw that when when you press the button, let's say S. Then it pops up a window. No.

926
01:29:54.770 --> 01:29:59.290
Gabor Szabo: we wanted to have it constantly there, and an empty text area.

927
01:30:00.830 --> 01:30:05.219
Orhun Parmaksiz: So you need to handle when you want to type something, and

928
01:30:05.520 --> 01:30:09.700
Orhun Parmaksiz: when when you when you finish typing and stuff like that. So

929
01:30:10.020 --> 01:30:12.820
Orhun Parmaksiz: what you said is something like a pop up.

930
01:30:13.140 --> 01:30:19.549
Orhun Parmaksiz: And yeah, that also has a state. But I'm not sure. What are you asking? Exactly.

931
01:30:20.690 --> 01:30:25.699
Gabor Szabo: Yeah, that's what I thought about the about the pop up to having a pop up. So so let's say you press

932
01:30:26.810 --> 01:30:30.310
Gabor Szabo: letter s it. It shows you a

933
01:30:31.110 --> 01:30:34.340
Gabor Szabo: a pop-up widget like this like this one.

934
01:30:34.540 --> 01:30:37.430
Orhun Parmaksiz: You type in something you press, enter.

935
01:30:38.270 --> 01:30:39.540
Gabor Szabo: And then.

936
01:30:40.740 --> 01:30:41.650
Orhun Parmaksiz: Yeah, okay.

937
01:30:42.340 --> 01:30:48.846
Orhun Parmaksiz: Yep, I see. Yeah. You can also do that, I think,

938
01:30:49.980 --> 01:30:54.679
Gabor Szabo: But I mean we don't need. We don't have to do it now. If if it takes too long.

939
01:30:55.030 --> 01:30:57.940
Gabor Szabo: we've already taken up a lot of of your time.

940
01:30:58.320 --> 01:31:01.040
Orhun Parmaksiz: That's fine. We can just like

941
01:31:02.040 --> 01:31:04.670
Orhun Parmaksiz: kind of show you how it's

942
01:31:05.490 --> 01:31:08.489
Orhun Parmaksiz: being done, and just leave the rest to you, I guess.

943
01:31:09.510 --> 01:31:14.090
Orhun Parmaksiz: And okay, I want to actually render this on the bottom

944
01:31:17.520 --> 01:31:19.269
Orhun Parmaksiz: of this block.

945
01:31:31.080 --> 01:31:37.750
Orhun Parmaksiz: or maybe let's just keep it simple here and rendered here instead.

946
01:31:38.920 --> 01:31:43.280
Orhun Parmaksiz: Okay, so I'll just run it here for now.

947
01:31:43.770 --> 01:31:45.730
Orhun Parmaksiz: but you can just change it.

948
01:31:45.850 --> 01:31:49.340
Orhun Parmaksiz: So on this block right.

949
01:31:51.490 --> 01:31:55.810
Orhun Parmaksiz: To actually type in you. You need to

950
01:31:56.060 --> 01:32:06.450
Orhun Parmaksiz: first, st I think, have another state here. So usually, what we recommend is having an enum for your app state. So app

951
01:32:06.960 --> 01:32:08.040
Orhun Parmaksiz: state.

952
01:32:09.230 --> 01:32:13.450
Orhun Parmaksiz: And I don't know normal a search.

953
01:32:14.870 --> 01:32:20.339
Orhun Parmaksiz: I don't know, input and so on. So this is like a more idiomatic way of handling the app

954
01:32:20.730 --> 01:32:26.619
Orhun Parmaksiz: application state, but for the sake of simplicity, let's just go ahead and do something like

955
01:32:27.902 --> 01:32:34.760
Orhun Parmaksiz: something like this here. So search Boolean false.

956
01:32:38.920 --> 01:32:41.390
Orhun Parmaksiz: and I want to enable search when you press. S.

957
01:32:43.838 --> 01:32:45.680
Orhun Parmaksiz: No, this is not correct.

958
01:32:45.840 --> 01:32:54.550
Orhun Parmaksiz: Yeah, alright. And when it's enabled, just render when it's enabled right self search.

959
01:33:00.940 --> 01:33:06.220
Orhun Parmaksiz: And when this is enabled we want to

960
01:33:09.040 --> 01:33:14.960
Orhun Parmaksiz: redirect all the key presses to the text area so that we are actually typing

961
01:33:15.540 --> 01:33:22.829
Orhun Parmaksiz: to do that, you're gonna go ahead and do something like if if you're searching

962
01:33:32.380 --> 01:33:35.650
Orhun Parmaksiz: update text area right?

963
01:33:35.770 --> 01:33:38.590
Orhun Parmaksiz: But now we cannot update it because we don't have a

964
01:33:38.920 --> 01:33:42.479
Orhun Parmaksiz: state for it. We are defining the text array here.

965
01:33:42.740 --> 01:33:46.319
Orhun Parmaksiz: and we don't have access to it in another in other functions.

966
01:33:46.630 --> 01:33:50.060
Orhun Parmaksiz: So we're gonna do it. Something like this.

967
01:33:51.420 --> 01:33:57.360
Orhun Parmaksiz: Another state text area text area static lifetime.

968
01:34:01.510 --> 01:34:03.799
Orhun Parmaksiz: And I'm going to initialize it here.

969
01:34:04.690 --> 01:34:09.149
Orhun Parmaksiz: I'm basically making a part of the application

970
01:34:12.740 --> 01:34:19.209
Orhun Parmaksiz: and self exterior.

971
01:34:20.900 --> 01:34:25.720
Orhun Parmaksiz: Here we say something like self text area input

972
01:34:29.990 --> 01:34:31.310
Orhun Parmaksiz: input and key.

973
01:34:33.400 --> 01:34:37.979
Orhun Parmaksiz: So we are sending the the key to a text area.

974
01:34:38.100 --> 01:34:40.429
Orhun Parmaksiz: So let's try it out when I press S.

975
01:34:40.630 --> 01:34:44.729
Orhun Parmaksiz: Now, there's a cursor there, and I can type.

976
01:34:45.610 --> 01:34:49.329
Orhun Parmaksiz: Well, I cannot type Q. Because Q is quit.

977
01:34:49.880 --> 01:34:52.110
Orhun Parmaksiz: That's not great. Actually, hold on.

978
01:34:54.150 --> 01:34:59.289
Orhun Parmaksiz: Yeah. But you got the idea. You can just press S type something here right?

979
01:34:59.750 --> 01:35:05.140
Orhun Parmaksiz: And then press something else like enter. Well, now, enter is just new line.

980
01:35:05.330 --> 01:35:08.990
Orhun Parmaksiz: But like, you can just assign an action to enter.

981
01:35:09.110 --> 01:35:12.819
Orhun Parmaksiz: And then when you do that, you basically filter the list.

982
01:35:13.500 --> 01:35:16.029
Orhun Parmaksiz: So yeah, yeah, that's how it works.

983
01:35:19.500 --> 01:35:20.240
Gabor Szabo: Excellent.

984
01:35:21.090 --> 01:35:23.070
Orhun Parmaksiz: I'm going to actually make this a pop up

985
01:35:23.320 --> 01:35:25.830
Orhun Parmaksiz: because you wanted to pop up. Let's do that.

986
01:35:26.370 --> 01:35:28.909
Gabor Szabo: But wait a second. Just comment it down. Okay.

987
01:35:28.910 --> 01:35:30.320
Orhun Parmaksiz: Okay? Okay, sure.

988
01:35:31.650 --> 01:35:32.450
Orhun Parmaksiz: Oops.

989
01:35:35.640 --> 01:35:39.200
Orhun Parmaksiz: Add search area.

990
01:35:39.770 --> 01:35:44.769
Orhun Parmaksiz: I'm going to change the location of this. Now render search.

991
01:35:45.590 --> 01:35:54.930
Orhun Parmaksiz: 3.rd So yeah, let let me see.

992
01:35:56.860 --> 01:35:59.099
Orhun Parmaksiz: Alright. So when you press S,

993
01:35:59.990 --> 01:36:04.379
Orhun Parmaksiz: I want the search to appear in this box as a pop-up.

994
01:36:04.900 --> 01:36:08.110
Orhun Parmaksiz: Right? That's the idea to do that.

995
01:36:09.440 --> 01:36:26.649
Orhun Parmaksiz: You basically, need to do something like this. This search area will will occupied 3 lines.

996
01:36:28.520 --> 01:36:29.560
Orhun Parmaksiz: So

997
01:36:32.580 --> 01:36:34.250
Orhun Parmaksiz: search area.

998
01:36:38.830 --> 01:36:42.680
Orhun Parmaksiz: let me actually create a new rectangle here.

999
01:36:46.340 --> 01:36:47.630
Orhun Parmaksiz: XY.

1000
01:36:48.610 --> 01:36:50.850
Orhun Parmaksiz: Height is just 3.

1001
01:36:52.980 --> 01:36:58.750
Orhun Parmaksiz: Okay, search area. I basically create a a custom rectangle here.

1002
01:36:59.170 --> 01:37:05.160
Orhun Parmaksiz: And when I run this again, you see that now we are rendering inside the

1003
01:37:05.540 --> 01:37:11.369
Orhun Parmaksiz: the area. But the thing is, it's still rendering the table.

1004
01:37:12.310 --> 01:37:19.930
Orhun Parmaksiz: So it's not actually pop up right now to do that, to like, remove the other table area

1005
01:37:20.630 --> 01:37:30.520
Orhun Parmaksiz: like the the background you need to render a widget called clear, so render widget clear.

1006
01:37:30.770 --> 01:37:36.330
Orhun Parmaksiz: This clears the background and then renders the text area.

1007
01:37:37.640 --> 01:37:39.369
Orhun Parmaksiz: So now, if you run this.

1008
01:37:40.030 --> 01:37:42.219
Orhun Parmaksiz: you see that we have a pop-up.

1009
01:37:42.390 --> 01:37:45.010
Orhun Parmaksiz: and it disappears. When I press S again.

1010
01:37:46.760 --> 01:37:52.610
Orhun Parmaksiz: Yeah, that's how you can create pop-ups. And is it so just.

1011
01:37:52.610 --> 01:37:57.480
Gabor Szabo: Frame that that was the general frame that we had, or or that was a separate frame.

1012
01:37:59.382 --> 01:38:02.550
Orhun Parmaksiz: Here, you mean it's the same frame.

1013
01:38:02.550 --> 01:38:09.240
Gabor Szabo: In the code. I think you. You put it on the frame, and that was a

1014
01:38:10.010 --> 01:38:11.840
Gabor Szabo: that was the the

1015
01:38:16.040 --> 01:38:17.459
Gabor Szabo: which frame is that.

1016
01:38:17.910 --> 01:38:22.059
Orhun Parmaksiz: It's the frame that we get from the draw call here.

1017
01:38:22.470 --> 01:38:25.580
Orhun Parmaksiz: So everything is the same frame, basically.

1018
01:38:27.500 --> 01:38:31.170
Orhun Parmaksiz: So we get one frame, and we just.

1019
01:38:31.170 --> 01:38:34.490
Gabor Szabo: Clear. But how does it know with what? What part to clear.

1020
01:38:37.470 --> 01:38:40.870
Orhun Parmaksiz: Because I create a new area here. You see.

1021
01:38:42.013 --> 01:38:47.130
Gabor Szabo: You told the clear to search, to clear the search area. I see.

1022
01:38:47.510 --> 01:38:48.660
Orhun Parmaksiz: Yeah, exactly.

1023
01:38:48.930 --> 01:38:55.309
Orhun Parmaksiz: So I'm clearing the area. So it's empty. So let me just show it to you. If I remove the text area.

1024
01:38:57.420 --> 01:38:59.630
Orhun Parmaksiz: You see, it's just clearing it.

1025
01:38:59.940 --> 01:39:01.220
Orhun Parmaksiz: Yeah, I think is there?

1026
01:39:01.980 --> 01:39:08.630
Orhun Parmaksiz: And I'm on top of that, I'm rendering the the input so yeah.

1027
01:39:08.630 --> 01:39:09.020
Gabor Szabo: Yeah.

1028
01:39:09.020 --> 01:39:09.649
Orhun Parmaksiz: That's a good word.

1029
01:39:09.650 --> 01:39:11.129
Gabor Szabo: Okay. Nice. Very nice.

1030
01:39:13.480 --> 01:39:15.800
Orhun Parmaksiz: Add search.

1031
01:39:15.990 --> 01:39:23.610
Orhun Parmaksiz: Let's pop up and to actually search.

1032
01:39:23.610 --> 01:39:24.340
Gabor Szabo: Back to

1033
01:39:24.770 --> 01:39:35.043
Gabor Szabo: it could actually work even without. And the enter, I mean, as you type it can update the the text below it. Let's say so we can.

1034
01:39:35.530 --> 01:39:37.750
Gabor Szabo: You can see the the table below it.

1035
01:39:38.480 --> 01:39:41.429
Gabor Szabo: except of those 3 lines that it was hiding.

1036
01:39:42.750 --> 01:39:54.529
Orhun Parmaksiz: Yeah, it's not very economic right now, but just to show you the the pop up, that's how it works, and to filter the values you can.

1037
01:39:54.798 --> 01:39:58.290
Gabor Szabo: Can you show me the code where you put it in in the

1038
01:39:58.400 --> 01:40:00.670
Gabor Szabo: in the top part of the lower frame?

1039
01:40:03.840 --> 01:40:05.460
Gabor Szabo: That's the X and the Y.

1040
01:40:06.140 --> 01:40:12.100
Orhun Parmaksiz: Yeah, so X, it's basically saying that push the area, one.

1041
01:40:12.680 --> 01:40:17.840
Orhun Parmaksiz: one line further for X and y, so it's like one line.

1042
01:40:18.490 --> 01:40:24.359
Orhun Parmaksiz: right and one bottom. So it's just like pushing it to the center. A bit

1043
01:40:24.550 --> 01:40:32.250
Orhun Parmaksiz: and width is just like the complete area of it, minus 2 2, because we have 2 borders.

1044
01:40:32.930 --> 01:40:35.700
Gabor Szabo: The area is the lower.

1045
01:40:36.510 --> 01:40:39.160
Gabor Szabo: Whatever was, yeah, exactly.

1046
01:40:39.460 --> 01:40:40.759
Orhun Parmaksiz: Yeah, exactly. That's the.

1047
01:40:41.030 --> 01:40:41.980
Gabor Szabo: It's.

1048
01:40:43.720 --> 01:40:44.340
Orhun Parmaksiz: Sorry.

1049
01:40:44.340 --> 01:40:45.090
Gabor Szabo: Yeah.

1050
01:40:46.880 --> 01:40:54.129
Gabor Szabo: So the whole thing is a frame. But the lower these these blocks are called widgets. That's what you you call them. The areas.

1051
01:40:55.880 --> 01:40:57.650
Orhun Parmaksiz: So the frame is.

1052
01:40:57.650 --> 01:41:00.200
Gabor Szabo: You gave it the 3rd right. I I see. I see.

1053
01:41:00.200 --> 01:41:06.999
Orhun Parmaksiz: Yeah, it's the 3rd area. I'm just passing the 3rd area to this function. It's called area here

1054
01:41:07.760 --> 01:41:17.280
Orhun Parmaksiz: and then creating a new area in there. You can also do something like area area inner.

1055
01:41:17.990 --> 01:41:19.630
Orhun Parmaksiz: So inner is also like

1056
01:41:22.700 --> 01:41:25.589
Orhun Parmaksiz: as a margin to the rectangle.

1057
01:41:26.150 --> 01:41:35.900
Orhun Parmaksiz: But I kind of like, did that explicitly here. You can also do it here like that. And yeah, it also works with the inner function.

1058
01:41:36.684 --> 01:41:42.095
Orhun Parmaksiz: But we're gonna rename those functions in the new release. So yeah, it's gonna they're gonna be called

1059
01:41:42.490 --> 01:41:48.850
Orhun Parmaksiz: area. Move by or resize, buy or something like that. So yeah.

1060
01:41:49.920 --> 01:41:56.220
Orhun Parmaksiz: so that's that's what we're doing. To get the text from the text area.

1061
01:41:56.220 --> 01:41:56.610
Gabor Szabo: Yeah.

1062
01:41:56.610 --> 01:42:03.140
Orhun Parmaksiz: You can do something like rows filter.

1063
01:42:05.790 --> 01:42:12.929
Orhun Parmaksiz: Okay, let me actually get the text. Let text self text area, get text.

1064
01:42:13.100 --> 01:42:17.600
Orhun Parmaksiz: It's not get text. It was content or something.

1065
01:42:19.040 --> 01:42:25.769
Orhun Parmaksiz: Yeah, I don't remember. It had. It had a weird name, that method.

1066
01:42:25.920 --> 01:42:27.409
Orhun Parmaksiz: I gotta find it. Now.

1067
01:42:30.880 --> 01:42:34.170
Orhun Parmaksiz: string content.

1068
01:42:35.670 --> 01:42:36.620
Orhun Parmaksiz: Text.

1069
01:42:42.032 --> 01:42:45.339
Orhun Parmaksiz: okay, you're gonna find it here lines.

1070
01:42:46.240 --> 01:42:48.970
Orhun Parmaksiz: Yeah, you can just call lines.

1071
01:42:51.680 --> 01:42:58.919
Orhun Parmaksiz: And the 1st 1 1st on the wrap.

1072
01:43:01.450 --> 01:43:02.999
Orhun Parmaksiz: Yeah, just unwrap it.

1073
01:43:05.070 --> 01:43:09.349
Orhun Parmaksiz: So this is a string now. So we get the 1st line from the text area.

1074
01:43:09.890 --> 01:43:13.160
Orhun Parmaksiz: and we can just filter rows

1075
01:43:16.490 --> 01:43:18.319
Orhun Parmaksiz: like this. Probably.

1076
01:43:20.090 --> 01:43:23.869
Orhun Parmaksiz: Yep, cell lowercase contains.

1077
01:43:24.260 --> 01:43:25.810
Orhun Parmaksiz: Yeah, okay, let's try it.

1078
01:43:26.990 --> 01:43:31.520
Orhun Parmaksiz: So I want to search for nail them. Yeah, it works.

1079
01:43:34.190 --> 01:43:35.680
Gabor Szabo: Very nice, very nice.

1080
01:43:36.780 --> 01:43:38.079
Orhun Parmaksiz: Yeah, there you go.

1081
01:43:40.850 --> 01:43:44.039
Orhun Parmaksiz: So yeah, that's how you can get the text from

1082
01:43:44.740 --> 01:43:47.399
Orhun Parmaksiz: the text area. Let me just commit this to

1083
01:43:56.430 --> 01:43:59.620
Orhun Parmaksiz: implement actual search, I don't know.

1084
01:44:04.270 --> 01:44:05.730
Orhun Parmaksiz: And yeah.

1085
01:44:11.564 --> 01:44:14.399
Orhun Parmaksiz: is there anything else that you're wondering about?

1086
01:44:16.920 --> 01:44:19.960
Gabor Szabo: Way more than I. I was expecting. Yeah.

1087
01:44:20.530 --> 01:44:21.100
Orhun Parmaksiz: Yep.

1088
01:44:21.910 --> 01:44:22.820
Orhun Parmaksiz: Cool.

1089
01:44:22.820 --> 01:44:23.580
Gabor Szabo: Master.

1090
01:44:25.670 --> 01:44:26.320
Orhun Parmaksiz: Yep.

1091
01:44:28.380 --> 01:44:29.000
Gabor Szabo: Well.

1092
01:44:29.980 --> 01:44:32.290
Orhun Parmaksiz: Yeah, I'm gonna leave these sorry.

1093
01:44:33.090 --> 01:44:34.740
Gabor Szabo: Did you want to show anything else.

1094
01:44:36.323 --> 01:44:37.370
Orhun Parmaksiz: I can.

1095
01:44:37.890 --> 01:44:51.159
Orhun Parmaksiz: I can like fill these 2 blocks. But I would. I just want to leave this to you, and if anybody's watching and want to work on this. Go ahead and implement the memory and

1096
01:44:51.550 --> 01:44:55.700
Orhun Parmaksiz: disk space the disk stuff. You can use

1097
01:44:56.730 --> 01:45:03.730
Orhun Parmaksiz: route to see this, not this line, gauge this widget.

1098
01:45:05.170 --> 01:45:08.060
Orhun Parmaksiz: And this is like, you know, the

1099
01:45:08.490 --> 01:45:15.229
Orhun Parmaksiz: good good one for that case. And if you're wondering which widgets are available, you just go to this link.

1100
01:45:15.580 --> 01:45:18.860
Orhun Parmaksiz: It's just in the previous module, and you can see that you know.

1101
01:45:18.860 --> 01:45:19.213
Gabor Szabo: You know.

1102
01:45:19.390 --> 01:45:22.749
Orhun Parmaksiz: We don't have many widgets, but they just work well, so

1103
01:45:23.110 --> 01:45:30.029
Orhun Parmaksiz: you can just like pick one. The sparkline is good for the network tab. Let me just show it to you once again

1104
01:45:30.190 --> 01:45:33.170
Orhun Parmaksiz: the other application that I had.

1105
01:45:34.540 --> 01:45:36.450
Orhun Parmaksiz: Alright. So I gotta go to

1106
01:45:40.310 --> 01:45:44.780
Orhun Parmaksiz: here. Yeah. So this is Parkline.

1107
01:45:45.280 --> 01:45:47.040
Orhun Parmaksiz: This is line gauge.

1108
01:45:47.170 --> 01:45:49.199
Orhun Parmaksiz: This is once again a chart.

1109
01:45:49.350 --> 01:45:53.289
Orhun Parmaksiz: This chart is the using a different

1110
01:45:53.400 --> 01:45:56.929
Orhun Parmaksiz: marker. That's why it's like a line here.

1111
01:45:57.240 --> 01:46:05.009
Orhun Parmaksiz: And for adding these text, you basically need to use the attributes on the widgets.

1112
01:46:05.370 --> 01:46:07.705
Orhun Parmaksiz: These are also just some

1113
01:46:08.450 --> 01:46:13.190
Orhun Parmaksiz: attributes on the chart. For example, if you want to like, add that

1114
01:46:13.390 --> 01:46:15.540
Orhun Parmaksiz: text to it. You kind of like.

1115
01:46:16.470 --> 01:46:21.610
Orhun Parmaksiz: add it to the data set, I think. Well, like.

1116
01:46:21.730 --> 01:46:34.040
Orhun Parmaksiz: yeah, just dive into the documentation, and you'll see that these little functions that allows you to customize how this looks. So just add those for the colors. It's just like the color name.

1117
01:46:34.320 --> 01:46:39.200
Orhun Parmaksiz: So, for example, the styled, you know, this is how you can create a style.

1118
01:46:39.840 --> 01:46:46.189
Orhun Parmaksiz: And this is language like, I said, the titles are just blocks title.

1119
01:46:47.050 --> 01:46:52.430
Orhun Parmaksiz: And yeah, rest is just customizing how things look

1120
01:46:52.830 --> 01:46:56.960
Orhun Parmaksiz: and adding a bit of a finesse to the tree.

1121
01:46:57.210 --> 01:46:58.719
Orhun Parmaksiz: And then, yeah.

1122
01:46:59.120 --> 01:47:07.777
Orhun Parmaksiz: you got your stuff running now, right now it's not looking that impressive. But once you start to add colors, and

1123
01:47:09.120 --> 01:47:26.399
Orhun Parmaksiz: more like customizations, it'll look better. You can use also the palette module that we already have here. This one has like 2 themes for now. So if you want to use the material theme, you can just like import this and use it

1124
01:47:27.140 --> 01:47:34.350
Orhun Parmaksiz: in your app like this, just, you know, colors from this palette.

1125
01:47:34.920 --> 01:47:38.370
Orhun Parmaksiz: and that's what I did here. I used the tailwind

1126
01:47:38.670 --> 01:47:43.040
Orhun Parmaksiz: colors, and just, you know, blue, red, yellow, whatever.

1127
01:47:43.310 --> 01:47:44.549
Orhun Parmaksiz: And then you get a

1128
01:47:44.900 --> 01:47:54.289
Orhun Parmaksiz: better looking interface. But like I said, it's not really just easy to make everything in just 2 2 HI think so. I'm just gonna leave it up to you.

1129
01:47:54.480 --> 01:48:01.729
Orhun Parmaksiz: But yep, that's that's I think all I'm going to show it to you. And I guess, like, take the code, play around

1130
01:48:01.930 --> 01:48:04.970
Orhun Parmaksiz: and let us know what you have built.

1131
01:48:05.370 --> 01:48:06.950
Orhun Parmaksiz: So yeah, that's all.

1132
01:48:06.950 --> 01:48:12.159
Gabor Szabo: Yes, well, thank you very much. It was really excellent.

1133
01:48:13.060 --> 01:48:25.250
Gabor Szabo: and thank you for everyone else also, who participated and asked questions and give recommendations and suggestions. And everyone who watched the video. Now is the time to

1134
01:48:25.770 --> 01:48:32.360
Gabor Szabo: like the video. Follow the channel. And I'll and

1135
01:48:33.020 --> 01:48:41.689
Gabor Szabo: put the link below the video for all kind of things, especially for the stuff that orphan does you also have a youtube channel, I think right or

1136
01:48:42.129 --> 01:48:52.339
Gabor Szabo: so so the 2 and start writing applications with, and start contributing.

1137
01:48:54.350 --> 01:48:55.639
Orhun Parmaksiz: Yeah, thank you very much.

1138
01:48:56.170 --> 01:49:02.410
Gabor Szabo: Everyone. Now, I just need to find the button because I made the screen

1139
01:49:02.600 --> 01:49:08.550
Gabor Szabo: different size. So thank you very much, and see you next time zombie bye, bye.

1140
01:49:08.820 --> 01:49:09.420
Orhun Parmaksiz: Bye.

