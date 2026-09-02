---
title: "Tauri: Cross-Platform desktop applications with Rust and web technologies"
timestamp: 2026-09-02T10:30:01
author:
published: true
show_related: true
description:
tags:
---

## Description:

Developing desktop applications with web technologies has been the norm for quite some time (at least for non-realtime applications). Slack, VS Code, Discord, Figma, 1Password, and more apps have been built this way. While Electron was, and still remains, a great choice for doing this, there is a new contender in town. Meet [Tauri](https://tauri.app/), designed for secure and lightweight desktop applications, without sacrificing on user experience, and while still using the mature web technologies we know and love.
In this session, I will introduce Tauri, and we will build a simple application using it together.

## Bio:

[Liel Friedman](https://www.linkedin.com/in/lielfr/) has been into open source for years, beginning with some translation work, and also contributing code to a few projects.
Professionally, he has about 4.5 years of experience in full-stack web development.

## Links and Notes

* [Tauri clipboard manager demo](https://github.com/lielfr/tauri-clipboard-manager-demo)
* [Tauri Slides](https://media.code-maven.com/slides/TauriSlideshow.pdf)


{% youtube id="DQwu0F-xdS0" file="2026-09-01-tauri-cross-platform-desktop-applications-with-rust-and-web-technologies-with-liel-friedman.mp4" %}

## Transcript

1
00:00:01.940 --> 00:00:14.360
Gabor Szabo: So, hello and welcome to this new episode of the Code Maven YouTube channel, if you are watching on YouTube, and the Rust Maven session, that's a soft session of the Code Maven thingy.

2
00:00:14.510 --> 00:00:26.789
Gabor Szabo: Below the video, if you are there, you will find links to where the further sessions are going to be, and where details about this session can be found, links, and

3
00:00:26.790 --> 00:00:49.439
Gabor Szabo: I don't know if we all have code examples, whatever. You will be able to find links below the video. I'm going to also share them here in the meeting. If you are in the meeting, then welcome, I'm really happy. It's very… it's always fun to present in front of people. And you have the opportunity to ask questions while we have

4
00:00:49.440 --> 00:01:06.799
Gabor Szabo: the presentation, and even after that, as I mentioned earlier, we can… you can stay around, and then we can have free chat about anything, whatever Leah does, or doesn't do, or about the cat, or… I don't know. So, we can discuss anything.

5
00:01:06.810 --> 00:01:12.590
Gabor Szabo: My name is Gabor Szabo, I run this channel, I run these, events,

6
00:01:13.260 --> 00:01:32.779
Gabor Szabo: And I think with this… that said, I can give you the floor, Liel. I'm really happy that you agreed to give this presentation. We already did this, or an earlier version of these ones, but now we can do it into the international community in English, so…

7
00:01:32.840 --> 00:01:38.769
Gabor Szabo: Let's get started. Liel. The floor is yours, please. Introduce yourself, whatever you like.

8
00:01:39.570 --> 00:01:40.280
Liel: Okay.

9
00:01:40.620 --> 00:01:41.729
Gabor Szabo: So I'm thinking that before.

10
00:01:42.060 --> 00:01:46.919
Gabor Szabo: Sorry, sorry, one second. How long do you feel… do you expect this is going to be… take?

11
00:01:47.520 --> 00:01:49.429
Gabor Szabo: The session, the whole thing.

12
00:01:49.430 --> 00:01:53.979
Liel: So… I think it will be less than the allocated 2 hours.

13
00:01:54.330 --> 00:01:56.060
Gabor Szabo: Okay, okay, good.

14
00:01:56.240 --> 00:02:03.750
Gabor Szabo: I just wanted to make sure that, so people will allow, we might, if it takes a long time, then we might have a break in the middle, if you like, okay?

15
00:02:04.020 --> 00:02:05.129
Gabor Szabo: Are you a short report.

16
00:02:05.130 --> 00:02:12.430
Liel: Last time we did go over time, but I, like, scaled down the second demo, so hopefully it won't happen this time.

17
00:02:12.430 --> 00:02:13.200
Gabor Szabo: Okay.

18
00:02:14.610 --> 00:02:15.760
Gabor Szabo: Go ahead, sorry.

19
00:02:16.290 --> 00:02:26.760
Liel: Okay, so as Gabor said, welcome, hi everyone, I'm really glad you either joined live, or that you're watching this video on YouTube.

20
00:02:27.230 --> 00:02:33.420
Liel: My name is Liel. I've been a professional software developer for about 4.5 years.

21
00:02:34.130 --> 00:02:36.399
Liel: Mostly full stack.

22
00:02:37.300 --> 00:02:40.940
Liel: More on the back end, to be honest, but also front-end as well.

23
00:02:41.760 --> 00:02:44.210
Liel: And, for the cat.

24
00:02:44.460 --> 00:02:49.129
Liel: This is Sasha, she is my assistant, and also an avidos station.

25
00:02:50.870 --> 00:02:56.879
Liel: And, yeah, she will be here as she has been with me for… since COVID days.

26
00:02:59.940 --> 00:03:09.389
Liel: This presentation will be about Tori, which is… cross-platform application development in Rust.

27
00:03:10.150 --> 00:03:11.760
Liel: And let's begin!

28
00:03:15.540 --> 00:03:23.920
Liel: Let's use… Presentation… Or maybe Advanced?

29
00:03:24.540 --> 00:03:31.920
Liel: And… Slides… Like that?

30
00:03:49.520 --> 00:03:58.010
Liel: Let's do this… Yeah. Let's just do a portion of screen, probably. We'll be fine.

31
00:03:59.640 --> 00:04:04.310
Liel: Okay, so now we see a personal screen, and… yeah.

32
00:04:04.660 --> 00:04:06.969
Liel: This one needs to go away.

33
00:04:07.570 --> 00:04:12.979
Liel: And I can actually resize this to be just the… the presentation.

34
00:04:13.820 --> 00:04:17.329
Liel: One second… Great.

35
00:04:19.690 --> 00:04:22.849
Liel: Okay, so totally in a nutshell.

36
00:04:23.310 --> 00:04:28.239
Liel: It's a library for graphical user interface application development.

37
00:04:28.650 --> 00:04:33.539
Liel: Mainly for desktop, but since version 2.0, it also supports mobile.

38
00:04:34.010 --> 00:04:38.249
Liel: We won't focus on the mobile side too much.

39
00:04:39.020 --> 00:04:46.390
Liel: But, in general, it works the same way, and also note that unlike React Native and Flutter.

40
00:04:46.570 --> 00:04:50.610
Liel: It just functions… it just opens a web view.

41
00:04:51.460 --> 00:04:55.649
Liel: It's not like rendering native components whatsoever.

42
00:04:57.560 --> 00:05:00.289
Liel: And it uses web technologies.

43
00:05:01.530 --> 00:05:13.149
Liel: So, yeah, inside the WebView browser, basically, you can use any front-end framework you like, whether it's Rust-based, like Dioxus, Leptos, Vue.

44
00:05:13.290 --> 00:05:18.980
Liel: or more traditional ones, like React, Angular overview.

45
00:05:19.400 --> 00:05:21.540
Liel: The backend code is in Rust.

46
00:05:22.570 --> 00:05:25.169
Liel: But you can also embed…

47
00:05:25.350 --> 00:05:37.399
Liel: like, additional binaries, if you need, as what's called a sidecar. You have, like, a link. We'll add a PDF of the presentation after that, so you can…

48
00:05:38.090 --> 00:05:41.229
Liel: Use all the links. It's written in Rust.

49
00:05:41.430 --> 00:05:51.450
Liel: And what's nice is that there are so many plugins that come with Tori that you don't really have to mess with the Rust code so much.

50
00:05:51.710 --> 00:05:52.930
Liel: If you want.

51
00:05:53.370 --> 00:06:00.110
Liel: But, you can, and this is a Rust talk, so I will be focusing on that part.

52
00:06:02.540 --> 00:06:07.050
Liel: So, there are two nice applications. I'm not affiliated with any of them.

53
00:06:07.250 --> 00:06:10.450
Liel: But, I think they show quite a nice.

54
00:06:11.800 --> 00:06:15.540
Liel: like, example of what you can do with Tori.

55
00:06:16.300 --> 00:06:20.930
Liel: Let's see… oh, it opened it through Lara Dufre.

56
00:06:21.580 --> 00:06:26.260
Liel: So, it's basically, like, chatting with Postgres.

57
00:06:26.950 --> 00:06:31.520
Liel: with the native language, so probably LLM-based.

58
00:06:32.390 --> 00:06:35.750
Liel: Based… And… yeah.

59
00:06:41.320 --> 00:06:51.909
Liel: That's one, and the other is RustDesk, which is basically an AnyDesk alternative, which is open source for remote desktop.

60
00:06:54.670 --> 00:07:01.869
Liel: So, yeah, that's, like, how it looks, and I think… It's also a nice example.

61
00:07:03.900 --> 00:07:20.079
Liel: Yeah, I probably wouldn't use Tori for, like, low-latency stuff. If you need, like, real-time display for DSP applications and so on, I wouldn't use it, but for most other applications, I think it's fine, and it's a great

62
00:07:20.570 --> 00:07:24.760
Liel: Solution, especially if you're already familiar with web technologies.

63
00:07:26.540 --> 00:07:31.929
Liel: So… Yeah, one other very popular

64
00:07:32.180 --> 00:07:38.779
Liel: solution for desktop applications based with web technologies is Electron.

65
00:07:39.410 --> 00:07:47.120
Liel: And we will compare and talk a little bit about the difference, because… okay, so first of all.

66
00:07:48.280 --> 00:07:50.710
Liel: Electron is Node.js-based.

67
00:07:52.490 --> 00:08:02.450
Liel: And Tory is Rust-based, but even beyond that, they are, like, using a little bit of different philosophies.

68
00:08:02.880 --> 00:08:04.699
Liel: regarding using a browser.

69
00:08:05.030 --> 00:08:13.069
Liel: While electron bundles chromium, Tori tries to use the OS web view.

70
00:08:13.380 --> 00:08:18.370
Liel: Which is… Usually WebKid-based.

71
00:08:18.480 --> 00:08:21.729
Liel: On Linux, it's WebKit GTK.

72
00:08:23.540 --> 00:08:27.739
Liel: On Windows, it's Edge WebView.

73
00:08:28.030 --> 00:08:32.600
Liel: And macOS, it's… yeah, it's WebKit as well, probably based on Safari.

74
00:08:33.140 --> 00:08:38.230
Liel: It has the advantage of being light, and potentially.

75
00:08:38.590 --> 00:08:44.949
Liel: It can, or… it can be more beneficial security-wise, because

76
00:08:45.220 --> 00:08:49.990
Liel: You kind of get the browser updates for free without needing to update your own app.

77
00:08:52.460 --> 00:08:59.460
Liel: But, the disadvantage is that you need to account for that, and, like, don't use

78
00:09:00.600 --> 00:09:04.170
Liel: Features that may not be supported.

79
00:09:06.460 --> 00:09:12.229
Liel: Regarding the file size, it's about a minimum of 600 kilobytes.

80
00:09:12.980 --> 00:09:17.950
Liel: Which is, like, way lighter than Electron apps, which are, like.

81
00:09:18.270 --> 00:09:20.829
Liel: placed a few hundred megabytes, I guess?

82
00:09:22.240 --> 00:09:25.780
Liel: Haven't done this in a while, but they are very large.

83
00:09:29.290 --> 00:09:36.160
Liel: And… In the future, it might be possible to embed the browser engine if you would like.

84
00:09:36.580 --> 00:09:43.830
Liel: But right now, it's like… you can technically fork Tory and try to do it, but… Broly.

85
00:09:44.630 --> 00:09:46.489
Liel: Quite a bit of work.

86
00:09:49.670 --> 00:09:57.900
Liel: And also, regarding security, while Electron apps can be very secure, the link here is the… I think it's the…

87
00:09:58.530 --> 00:10:03.389
Liel: guidelines for security in Electron, but TORI is built

88
00:10:03.940 --> 00:10:08.259
Liel: So, that it's both harder to misuse, and…

89
00:10:08.640 --> 00:10:14.420
Liel: It can be, in some cases, less vulnerable to supply chain attacks.

90
00:10:15.130 --> 00:10:21.709
Liel: So we'll talk a little bit about how the frontend communicates with the Rust part, but

91
00:10:21.840 --> 00:10:34.539
Liel: If you build it right, the front end, even if you have, like, a malicious code inside of it, can do as much damage as it potentially can

92
00:10:35.660 --> 00:10:36.870
Liel: in electron.

93
00:10:38.350 --> 00:10:45.200
Liel: But the caveat is that the Rust code is not isolated, so…

94
00:10:45.480 --> 00:10:50.880
Liel: And as we saw, the Rosa supply chain attack.

95
00:10:51.360 --> 00:10:53.460
Liel: A few weeks ago, I think.

96
00:10:57.630 --> 00:11:03.240
Liel: So, yeah. It's not perfect, but I think it's a step in the right direction.

97
00:11:05.010 --> 00:11:09.120
Liel: Oh, I also forgot to… To tell that…

98
00:11:09.260 --> 00:11:15.969
Liel: the front end only communicates with the backend via message passing, so there is no, like, FFI between the front end and

99
00:11:16.740 --> 00:11:17.720
Liel: there.

100
00:11:18.460 --> 00:11:20.879
Liel: The core process, as it's called.

101
00:11:21.070 --> 00:11:22.559
Liel: Which is the arrest.

102
00:11:22.810 --> 00:11:24.549
Liel: Part of the app.

103
00:11:28.870 --> 00:11:33.570
Liel: Let's… See if we have any questions…

104
00:11:35.410 --> 00:11:38.330
Liel: I don't know, not so far, so let's continue.

105
00:11:38.780 --> 00:11:42.470
Liel: Alright, yeah, feel free to write questions on the chat.

106
00:11:42.640 --> 00:11:48.690
Liel: And, I'll stop every once in a while and… see them.

107
00:11:49.950 --> 00:11:51.030
Liel: Oh, sorry.

108
00:11:51.810 --> 00:11:59.480
Liel: So, we have two primary CLI tools, Creatoria App for, like, scaffolding projects from scratch.

109
00:12:00.000 --> 00:12:02.870
Liel: And we have the Tori CLI.

110
00:12:03.010 --> 00:12:05.359
Liel: Which can be used to, like.

111
00:12:05.510 --> 00:12:08.540
Liel: Introduce story to an existing front-end app.

112
00:12:08.960 --> 00:12:13.019
Liel: And it can be also used for other stuff, like

113
00:12:13.310 --> 00:12:17.630
Liel: Building the app, bundling it for distribution, and so on.

114
00:12:18.330 --> 00:12:27.530
Liel: And we need Rust, as well as Node.js, and their favorite build tool, which is, like, NPM, BAN, PNPM, YARN,

115
00:12:27.650 --> 00:12:31.620
Liel: Oh, the .NET equivalent, if you're… you can also use .NET.

116
00:12:31.980 --> 00:12:36.430
Liel: as… For the front end, using Blazor.

117
00:12:36.900 --> 00:12:39.059
Liel: But I haven't tried that, to be honest.

118
00:12:39.530 --> 00:12:43.979
Liel: So, you can try that, see the documentation, it's really good.

119
00:12:48.240 --> 00:12:52.730
Liel: And… okay. So, regarding project structure…

120
00:12:53.470 --> 00:13:01.169
Liel: as I told you before, you can kind of inject Tory into an existing front-end project.

121
00:13:01.620 --> 00:13:06.020
Liel: the way it does all, it's, like, we have SRC-story.

122
00:13:06.560 --> 00:13:11.169
Liel: file, folder for the Rust code and the toggle configuration.

123
00:13:14.410 --> 00:13:18.550
Liel: And we can also run the frontend separately if we'd like to.

124
00:13:23.970 --> 00:13:29.660
Liel: Regarding the process model, we have the core, which is, like, the mandatory process.

125
00:13:30.280 --> 00:13:34.549
Liel: It's the only thing with full operating system access.

126
00:13:36.810 --> 00:13:39.740
Liel: So, like, every Rust code runs here.

127
00:13:41.990 --> 00:13:48.979
Liel: And it opens one or more WebView processes, which each has one WebView.

128
00:13:49.380 --> 00:13:53.100
Liel: Which is, like, what's rendering the front end.

129
00:13:56.220 --> 00:14:09.400
Liel: Regarding security, when you are, like, doing message passing, it's best practice to treat the web view as, like, an actual browser, meaning that you don't trust

130
00:14:09.840 --> 00:14:11.929
Liel: Like, the inputs.

131
00:14:12.280 --> 00:14:22.289
Liel: From the web view blindly, but you do, like, do validations and sanitize it, whatever you do, if it was, like, an actual web app and

132
00:14:22.720 --> 00:14:27.970
Liel: you'd want… To secure it from potentially malicious inputs.

133
00:14:30.850 --> 00:14:39.070
Liel: It's also recommended to write the logic in Rust, but yeah, you can reuse plugins if you want, and…

134
00:14:39.370 --> 00:14:40.770
Liel: many.

135
00:14:46.810 --> 00:14:49.550
Liel: Okay, so we talked about method passing.

136
00:14:49.660 --> 00:14:53.959
Liel: And the core of message passing is called commands.

137
00:14:54.130 --> 00:15:03.870
Liel: So, basically, You can have, like, commands that… which are, like, which simulate

138
00:15:04.350 --> 00:15:10.829
Liel: RPC, remote procedural call, but are actually Just messages passing me around?

139
00:15:13.720 --> 00:15:23.809
Liel: And… yeah, you define them in the Rust, and you can call them from the front end. We'll see that soon. You can also, like.

140
00:15:23.940 --> 00:15:26.200
Liel: Restrict the inputs.

141
00:15:27.260 --> 00:15:29.999
Liel: That, it allows.

142
00:15:30.410 --> 00:15:33.449
Liel: Via, what's called the permission system.

143
00:15:34.170 --> 00:15:38.880
Liel: So you can, like, make sure that some out… some inputs are

144
00:15:39.390 --> 00:15:43.300
Liel: Yeah, restricted even before they go into the call.

145
00:15:44.470 --> 00:15:50.890
Liel: And, you can also have pretty much everything that you can deserialize with ZRD.

146
00:15:51.940 --> 00:15:54.860
Liel: So, behind the scenes, it probably uses JSON.

147
00:15:57.180 --> 00:16:09.209
Liel: And… yeah, it's a really nice mechanism, we'll see it in the next slide. Just a reminder that the Rust code is not isolated, so you'll need to treat that as

148
00:16:10.210 --> 00:16:15.040
Liel: Code that has elevated privileges, and so on.

149
00:16:15.890 --> 00:16:23.190
Liel: So the way you define commands is that you have the attribute macro of ToriCommand.

150
00:16:23.720 --> 00:16:35.750
Liel: And yeah, it's a little… yeah, it's honestly a silly example, because it wouldn't validate password and user like that, but for example, it would work.

151
00:16:36.320 --> 00:16:39.970
Liel: So, you can… optionally have.

152
00:16:40.690 --> 00:16:44.510
Liel: As many params as you'd like, or you can have none.

153
00:16:45.890 --> 00:16:50.150
Liel: And you can also… Return a result type.

154
00:16:50.700 --> 00:16:55.429
Liel: So, if you have, like, an error, in this case.

155
00:16:55.740 --> 00:17:09.180
Liel: We just return the error, and what happens in the JavaScript side is that it returns a promise. So if we return OK, then the promise resolves with our output.

156
00:17:09.329 --> 00:17:14.420
Liel: But if we're down… An error, it rejects.

157
00:17:15.890 --> 00:17:21.240
Liel: We can also use the async keyword, as Tori uses Tokyo.

158
00:17:21.410 --> 00:17:28.280
Liel: So if we want something that doesn't run in the main thread, we can… like, Ed Essing?

159
00:17:28.550 --> 00:17:33.339
Liel: function login. From the JavaScript side, it's actually the same, it was a promise.

160
00:17:36.030 --> 00:17:39.399
Liel: By the way, if you're interested on how this works.

161
00:17:39.580 --> 00:17:43.249
Liel: Basically, a web view allows you to register a custom protocol.

162
00:17:43.450 --> 00:17:50.820
Liel: So… a command from Tauri is actually just… a fetch call.

163
00:17:51.190 --> 00:17:55.760
Liel: So, it's almost like an HTTP call, but instead of HTTP,

164
00:17:57.490 --> 00:18:08.180
Liel: let me just… yeah, let me just have myself something I can write on. So instead of… http.slash-less. Something?

165
00:18:09.920 --> 00:18:12.420
Liel: it calls, I think, IPC.

166
00:18:13.380 --> 00:18:14.980
Liel: Colon slash slash.

167
00:18:15.220 --> 00:18:16.130
Liel: Something.

168
00:18:16.280 --> 00:18:25.360
Liel: And that is registered, so it doesn't actually go over the wire, but goes directly to the toll record responsible for handling.

169
00:18:26.780 --> 00:18:28.330
Liel: Nicole's.

170
00:18:33.470 --> 00:18:41.399
Liel: Also, another tip, if you have, like, a large array, You'd like to return?

171
00:18:41.820 --> 00:18:48.929
Liel: You can return a type called the Tory… Happy to see your response.

172
00:18:49.100 --> 00:18:57.500
Liel: Which… is optimized for this kind of thing, so if you, like, if you're, like.

173
00:18:57.670 --> 00:19:07.579
Liel: pass a large image, or a sound, or an audio file, so that would be a use case for that IPC response type.

174
00:19:10.190 --> 00:19:18.350
Liel: We also have… A mechanism for small events, so we can, like, Post an event, and…

175
00:19:22.080 --> 00:19:24.940
Liel: And listen to events?

176
00:19:25.260 --> 00:19:27.299
Liel: There are simpler than commands.

177
00:19:27.420 --> 00:19:33.450
Liel: So, they only use JSON, And… there aren't.

178
00:19:34.110 --> 00:19:37.720
Liel: Any type checks, and no permissions or capabilities.

179
00:19:38.460 --> 00:19:44.000
Liel: But they are flexible, so you can have as many producers and as many consumers as you like.

180
00:19:45.260 --> 00:19:50.900
Liel: There is also a similar mechanism called channels, which is low latency.

181
00:19:52.560 --> 00:19:57.910
Liel: And, yeah, let's see an example. We can have global events.

182
00:19:58.040 --> 00:20:02.170
Liel: So, let's say we have a command for download a file.

183
00:20:02.610 --> 00:20:06.009
Liel: We can emit an event. Download started…

184
00:20:06.230 --> 00:20:09.930
Liel: And we can omit the process… the progress, and when it's finished.

185
00:20:10.100 --> 00:20:19.459
Liel: And we can also… emit to a specific screen, so it's specific… it's for one specific web view.

186
00:20:22.780 --> 00:20:29.759
Liel: We can also have something called an emit filter, which allows us to, like, allow

187
00:20:30.250 --> 00:20:31.659
Liel: A bunch of web views.

188
00:20:35.580 --> 00:20:41.640
Liel: But, yeah, it can be anything that's serializable.

189
00:20:45.480 --> 00:20:50.240
Liel: And how we listen to events in TypeScript is… looks like that.

190
00:20:50.880 --> 00:20:59.249
Liel: So we define a type for download started, and we listen to an event, and we have our callback, which in this case is just a

191
00:20:59.390 --> 00:21:00.650
Liel: Console log.

192
00:21:01.800 --> 00:21:03.910
Liel: Note that the type isn't, like.

193
00:21:04.090 --> 00:21:12.620
Liel: actually checked, so it's only for, like, developer experience, but it doesn't actually check that all these fields exist.

194
00:21:18.960 --> 00:21:23.489
Liel: for WorkView-specific events, this is how we'll listen to them.

195
00:21:24.310 --> 00:21:30.600
Liel: We first get the WorkView window, and then… We do, like, listen.

196
00:21:31.280 --> 00:21:36.530
Liel: And yeah, in this case, we just set a session token. Again, please don't do that.

197
00:21:36.950 --> 00:21:38.700
Liel: In a real app.

198
00:21:42.110 --> 00:21:49.799
Liel: And finally, for cleanup, every listen, also every call to listen, also returns

199
00:21:50.480 --> 00:21:58.449
Liel: A function, which can be used to deregister the Listener. But…

200
00:21:58.860 --> 00:22:04.520
Liel: Unless you're doing, like, single-page applications, You can, like.

201
00:22:05.760 --> 00:22:11.969
Liel: ignore that part. But if you're using, like, React, whenever you are switching components and so on.

202
00:22:12.790 --> 00:22:16.720
Liel: Please, do and listen to… like, clean up.

203
00:22:17.060 --> 00:22:19.669
Liel: Any event listeners you don't use.

204
00:22:22.250 --> 00:22:33.670
Liel: And behind the scenes, events work, like, really similarly to commands. It's also, like, IPC call, just an internal one tutorial.

205
00:22:34.200 --> 00:22:41.339
Liel: And what it does, it basically runs the event listeners, so, like, it keeps some kind of a map.

206
00:22:41.990 --> 00:22:50.070
Liel: Between every event and its listeners, and it just runs the listener code every time,

207
00:22:50.390 --> 00:22:54.120
Liel: the actual event happens. So, let's say I logged in.

208
00:22:54.400 --> 00:23:02.800
Liel: There is, like, a map with the name logged in as a key, And… Like, all the handlers.

209
00:23:03.530 --> 00:23:08.290
Liel: inside it, so it calls the JavaScript via the web view.

210
00:23:09.880 --> 00:23:16.700
Liel: to run each one of these listeners. If you're interested, you can check out the source code

211
00:23:17.950 --> 00:23:21.420
Liel: It's in the Tory… Runtime.

212
00:23:22.020 --> 00:23:25.989
Liel: Great, I think. Or the Tory runtime WRY.

213
00:23:26.170 --> 00:23:29.469
Liel: Just, like, picked at it before

214
00:23:30.050 --> 00:23:33.309
Liel: So I remember how that works.

215
00:23:34.650 --> 00:23:40.160
Liel: There's also a utility function that's not listed That's not listed here.

216
00:23:41.190 --> 00:23:48.750
Liel: that allows to listen to an event exactly once. So it's similar to, like, a one-shot channel.

217
00:23:50.820 --> 00:23:57.389
Liel: And… Yeah, that's that. Sorry, I accidentally…

218
00:24:00.070 --> 00:24:08.189
Liel: Is Tory ready for a mobile app? I'll answer that once we are, done with events and channels.

219
00:24:12.990 --> 00:24:17.630
Liel: So that's that, and let's do… Damn.

220
00:24:17.910 --> 00:24:19.680
Liel: Longer example.

221
00:24:20.410 --> 00:24:24.160
Liel: So, we have a download event, we serialize it.

222
00:24:25.030 --> 00:24:31.830
Liel: It's all the nice derived attributes, so, like, it… the camel case, and so on.

223
00:24:34.460 --> 00:24:44.640
Liel: And we define a command. Once again, download, because that's the example they got at the official documentation. We'll do another one in the demos.

224
00:24:47.250 --> 00:24:51.679
Liel: So, we have the app handle… which is injected.

225
00:24:53.800 --> 00:24:56.649
Liel: We have URL and unevent.

226
00:24:56.980 --> 00:25:03.390
Liel: So, we can basically send Any, like, progress event or whatever.

227
00:25:03.760 --> 00:25:07.210
Liel: On the channel, and it's faster.

228
00:25:07.950 --> 00:25:11.820
Liel: then… Just using the events.

229
00:25:13.570 --> 00:25:14.700
Liel: And…

230
00:25:18.780 --> 00:25:24.160
Liel: Yeah, it's a… it's kind of similar to a command, but…

231
00:25:25.290 --> 00:25:30.660
Liel: It also allows you to send multiple messages.

232
00:25:33.770 --> 00:25:37.919
Liel: On the front-end side, how it looks is that we define the channel.

233
00:25:38.330 --> 00:25:39.359
Liel: From here…

234
00:25:39.780 --> 00:25:46.999
Liel: And also note that the channels are only created on the front-end side. You cannot create them from Rust. That's, like, how they're designed.

235
00:25:50.820 --> 00:26:00.610
Liel: And, yeah, they're useful when we have, like, a command, and we want, instead of just one, output?

236
00:26:01.770 --> 00:26:04.729
Liel: We can return, like, a bunch of outputs.

237
00:26:04.970 --> 00:26:11.700
Liel: And, yeah, again, these are ordered, so… That can be.

238
00:26:12.100 --> 00:26:13.589
Liel: Really nice as well.

239
00:26:18.620 --> 00:26:20.750
Liel: We can also listen to events.

240
00:26:21.400 --> 00:26:22.540
Liel: in Rust.

241
00:26:23.770 --> 00:26:30.160
Liel: So we do, in the setup, we have the app handle, and we listen to the event like that.

242
00:26:33.310 --> 00:26:38.259
Liel: It can also be done in a WebView-specific version as well, but…

243
00:26:38.970 --> 00:26:43.689
Liel: Yeah, we won't show that, at least in here. We might show that in the second demo.

244
00:26:45.150 --> 00:26:52.309
Liel: We also have unlisten and a way to send events only once, or listen to just one message.

245
00:26:56.020 --> 00:27:01.039
Liel: Okay, so… Before we go to the first demo.

246
00:27:01.700 --> 00:27:05.210
Liel: I see a question from Raw.

247
00:27:05.840 --> 00:27:09.039
Liel: I hope I didn't butcher your name.

248
00:27:11.670 --> 00:27:14.049
Liel: Is store ready for mobile app?

249
00:27:15.290 --> 00:27:16.470
Liel: So…

250
00:27:17.650 --> 00:27:29.039
Liel: It is in a stable state, but the caveat is that you only have a web view, so it's like having one giant browser window which shows your app.

251
00:27:32.460 --> 00:27:36.979
Liel: So, personally, I wouldn't use it for a mobile app unless to

252
00:27:37.390 --> 00:27:44.220
Liel: I'm like, okay, and you already have, like, a fantastic and responsive web app. You'd just like to…

253
00:27:44.520 --> 00:27:45.790
Liel: Have a mobile?

254
00:27:46.810 --> 00:27:48.480
Liel: But you can use that.

255
00:27:48.660 --> 00:27:53.889
Liel: In my opinion, a better approach would be, like, to have some crate

256
00:27:54.010 --> 00:28:01.330
Liel: as, like, a shared Rust core, and use… Like, either native, or…

257
00:28:01.450 --> 00:28:06.610
Liel: any of the other solutions, like React Native or Flutter, which…

258
00:28:07.000 --> 00:28:10.450
Liel: Flutter has nice support for Rust as well.

259
00:28:18.780 --> 00:28:21.769
Liel: Yeah, I hope that answered the question.

260
00:28:22.770 --> 00:28:25.889
Liel: Anything else before we move on to the first demo?

261
00:28:39.610 --> 00:28:41.490
Gabor Szabo: Nice again, we can, we can go ahead.

262
00:28:42.620 --> 00:28:45.060
Liel: Okay, so for the first demo.

263
00:28:45.570 --> 00:28:53.299
Liel: what I'll do is I… I'll stop the share, and… We'll switch to Gabor.

264
00:28:53.470 --> 00:28:57.350
Liel: And I'll kinda instruct him.

265
00:28:57.470 --> 00:29:02.049
Liel: how we can basically bootstrap Tori app from scratch.

266
00:29:17.930 --> 00:29:19.629
Gabor Szabo: Something is broken here.

267
00:29:25.570 --> 00:29:26.480
Gabor Szabo: Okay.

268
00:29:26.480 --> 00:29:27.710
Liel: Oh, great.

269
00:29:29.190 --> 00:29:34.529
Liel: Okay, so you probably have Rust installed, and you have NPM as well.

270
00:29:35.400 --> 00:29:44.830
Gabor Szabo: Probably… probably from the previous time, I still have NPM. I don't… Rust, I keep upgrading. NPM, probably not.

271
00:29:45.450 --> 00:29:49.299
Gabor Szabo: So, whatever we have. Tell me what to do.

272
00:29:49.700 --> 00:29:50.300
Gabor Szabo: Okay.

273
00:29:50.300 --> 00:29:56.390
Liel: Okay, so first, we need to, like, run… And…

274
00:29:56.840 --> 00:30:03.790
Liel: the command to create the Tory app. So, what we'll do is npm space Create.

275
00:30:04.170 --> 00:30:07.079
Gabor Szabo: Is it going to create a… is it going to create a folder?

276
00:30:07.480 --> 00:30:09.219
Liel: Yeah, it is.

277
00:30:13.430 --> 00:30:15.840
Liel: I'll just paste that.

278
00:30:16.860 --> 00:30:18.080
Liel: Into the chat.

279
00:30:19.450 --> 00:30:23.960
Liel: Let's hope that the version is okay. If not, I will do it.

280
00:30:24.900 --> 00:30:26.029
Liel: On my machine.

281
00:30:38.700 --> 00:30:41.660
Liel: Yeah, we need the creator reactor.

282
00:30:46.530 --> 00:30:52.509
Liel: Okay, so product name, it will be the folder, let's say Tory Demo, or whatever you'd like.

283
00:30:53.130 --> 00:30:56.000
Gabor Szabo: Yeah, I just verified that it doesn't exist here.

284
00:30:56.750 --> 00:30:58.810
Gabor Szabo: Double up is good, or…

285
00:31:00.210 --> 00:31:02.139
Liel: Yeah, that could be good as well.

286
00:31:03.030 --> 00:31:03.850
Gabor Szabo: Okay.

287
00:31:04.890 --> 00:31:12.280
Liel: identifier, yeah, it could be… it probably makes more sense for mobile, because in, like.

288
00:31:12.500 --> 00:31:17.049
Liel: at least in iOS and Android, you do have that convention of.

289
00:31:17.420 --> 00:31:19.760
Gabor Szabo: It's like my domain name should be, right? So…

290
00:31:25.380 --> 00:31:25.840
Liel: Yeah.

291
00:31:25.840 --> 00:31:31.350
Gabor Szabo: That… that would be… because SubGov.com is my personal domain, or whatever.

292
00:31:32.270 --> 00:31:32.890
Gabor Szabo: Okay.

293
00:31:32.890 --> 00:31:33.500
Liel: Yeah.

294
00:31:33.760 --> 00:31:36.919
Liel: Let's do TypeScript or JavaScript.

295
00:31:37.470 --> 00:31:41.509
Gabor Szabo: Okay, so I can move here, but I just skip the first one.

296
00:31:42.090 --> 00:31:45.419
Liel: Yeah, package manager, let's just do 1PM.

297
00:31:45.950 --> 00:31:49.400
Liel: Because I'm not sure if you have anything.

298
00:31:49.760 --> 00:31:53.959
Liel: And for the UI template, it doesn't really matter, let's do vanilla.

299
00:31:54.100 --> 00:31:55.200
Liel: So it's…

300
00:31:55.500 --> 00:31:57.700
Gabor Szabo: Okay, so YouTube, I keep it.

301
00:31:57.850 --> 00:31:58.830
Gabor Szabo: Like this.

302
00:32:00.660 --> 00:32:06.090
Liel: Yeah, UI flavor… It can be whatever, the default is fine.

303
00:32:09.500 --> 00:32:13.389
Liel: Okay, and now we have a folder called ToriApp.

304
00:32:20.370 --> 00:32:20.850
Gabor Szabo: Okay.

305
00:32:20.850 --> 00:32:25.230
Liel: And, we need to do, like, npm install. It says, so,

306
00:32:27.770 --> 00:32:29.739
Gabor Szabo: Just wanted to see what's in here.

307
00:32:30.150 --> 00:32:36.440
Liel: Okay, so as you can see, that's great. We have the SRC, which is the front end, we have SRC Tory.

308
00:32:38.150 --> 00:32:39.009
Gabor Szabo: This one.

309
00:32:39.820 --> 00:32:40.410
Liel: Yeah.

310
00:32:41.860 --> 00:32:42.690
Gabor Szabo: Okay.

311
00:32:43.360 --> 00:32:45.050
Gabor Szabo: So, NPM?

312
00:32:45.760 --> 00:32:49.579
Liel: Yeah, npm installed, because we need to get all the dependencies.

313
00:32:57.500 --> 00:33:01.919
Gabor Szabo: Go ahead, ask a question, or whoever wants to ask a question, just type in the question.

314
00:33:09.050 --> 00:33:13.019
Gabor Szabo: If that's… that's why you… you raised your hand for… I don't know.

315
00:33:13.260 --> 00:33:19.970
Gabor Szabo: Anyway, if anyone has questions, just type them in the chat, we are both looking at it once in a while.

316
00:33:22.150 --> 00:33:23.200
Gabor Szabo: Okay.

317
00:33:25.740 --> 00:33:26.690
Gabor Szabo: Done.

318
00:33:27.390 --> 00:33:32.950
Liel: Okay, and now there is… it's NPM… Random.

319
00:33:33.400 --> 00:33:37.789
Gabor Szabo: Actually, before that, maybe I should start a Git repository here, right?

320
00:33:39.480 --> 00:33:40.580
Liel: Sure.

321
00:33:42.650 --> 00:33:46.000
Gabor Szabo: And git add… Everything?

322
00:33:47.550 --> 00:33:53.050
Liel: Yeah, it also includes a gitignore, so… We should be fine.

323
00:33:54.910 --> 00:33:55.600
Gabor Szabo: Yeah.

324
00:33:58.630 --> 00:33:59.670
Gabor Szabo: Okay.

325
00:34:01.290 --> 00:34:06.439
Gabor Szabo: So it has this VS Code folder. Why do I need the VS Code folder in the report?

326
00:34:06.440 --> 00:34:08.790
Liel: Probably don't, unless you use VS Code.

327
00:34:09.560 --> 00:34:12.310
Gabor Szabo: I mean, we might, we might use, but, okay.

328
00:34:12.679 --> 00:34:15.870
Gabor Szabo: So, git… let's just git add.

329
00:34:17.530 --> 00:34:18.530
Gabor Szabo: Okay.

330
00:34:23.280 --> 00:34:24.190
Gabor Szabo: Okay.

331
00:34:24.449 --> 00:34:29.920
Gabor Szabo: then I'm going to push it out, everything to a repository later on, okay? But for now… Okay.

332
00:34:30.100 --> 00:34:30.730
Gabor Szabo: We are not…

333
00:34:30.739 --> 00:34:33.609
Liel: Probably won't be changing that much, but, sure.

334
00:34:34.199 --> 00:34:34.989
Gabor Szabo: Okay.

335
00:34:36.260 --> 00:34:41.829
Liel: now it's npm run dev, so space between every word.

336
00:34:44.400 --> 00:34:48.100
Liel: Oh, that just runs the front end.

337
00:34:50.360 --> 00:34:51.250
Gabor Szabo: Okay.

338
00:34:52.239 --> 00:34:53.469
Gabor Szabo: Should I visit it?

339
00:34:54.530 --> 00:34:58.359
Liel: Sure, let's visit it, just to see what it looks like.

340
00:34:58.360 --> 00:34:59.989
Gabor Szabo: The one for…

341
00:35:08.970 --> 00:35:09.950
Gabor Szabo: Okay.

342
00:35:10.630 --> 00:35:15.410
Liel: Yeah, that's, like, a simple app. The grid won't work.

343
00:35:15.990 --> 00:35:21.689
Liel: But now let's return to the terminal, and and actually run the…

344
00:35:21.860 --> 00:35:27.129
Liel: Yeah, it won't work, and you will probably see an error in the console if you open it.

345
00:35:29.410 --> 00:35:31.219
Liel: Here, the browser console.

346
00:35:31.220 --> 00:35:32.410
Gabor Szabo: Yeah. Okay.

347
00:35:32.410 --> 00:35:46.670
Liel: cannot access because you don't have that. So, yeah, that's, like, a nice tip, too. You can check for that window.underscore underscore tori internals underscore underscore. If you have an app that you'd like to work

348
00:35:47.000 --> 00:35:50.260
Liel: Either in the browser or in Tori, so you can…

349
00:35:50.490 --> 00:35:53.619
Liel: Like, have the best of both worlds, and have called that…

350
00:35:56.880 --> 00:35:57.620
Gabor Szabo: Okay?

351
00:35:58.080 --> 00:36:01.320
Liel: And do that. Let's go back to the terminal, and…

352
00:36:02.290 --> 00:36:04.949
Liel: Yeah, let's scale the process.

353
00:36:05.600 --> 00:36:09.069
Gabor Szabo: Will it not restart, reload, restart, if I make a change?

354
00:36:09.070 --> 00:36:13.429
Liel: We will just, Ctrl-C it, because we need a different one, so…

355
00:36:13.840 --> 00:36:18.300
Liel: Instead of npm run dev, we need just the word Tory between run and dev.

356
00:36:24.130 --> 00:36:24.740
Gabor Szabo: This?

357
00:36:24.950 --> 00:36:25.580
Liel: Yeah.

358
00:36:25.890 --> 00:36:29.100
Liel: That should… Ron Tory.

359
00:36:32.250 --> 00:36:33.190
Liel: Okay, so…

360
00:36:33.190 --> 00:36:33.850
Gabor Szabo: Hmm?

361
00:36:33.850 --> 00:36:36.039
Liel: It built it, now it builds.

362
00:36:40.100 --> 00:36:41.040
Liel: the app.

363
00:36:47.050 --> 00:36:48.170
Gabor Szabo: It takes time.

364
00:36:51.380 --> 00:36:54.979
Liel: Oh, so apparently some packages are missing?

365
00:36:54.980 --> 00:36:55.910
Gabor Szabo: Oh, sorry.

366
00:36:56.260 --> 00:37:01.630
Gabor Szabo: Just wanted to see how much it takes, how long… how much…

367
00:37:03.450 --> 00:37:06.900
Gabor Szabo: Okay, whatever. Just wanted to look at my CPUs. Okay.

368
00:37:07.090 --> 00:37:07.670
Liel: Oh.

369
00:37:08.850 --> 00:37:11.920
Liel: Oh, okay, so on Ubuntu, it…

370
00:37:11.920 --> 00:37:12.730
Gabor Szabo: Yeah.

371
00:37:12.730 --> 00:37:14.000
Liel: How you're running it?

372
00:37:14.430 --> 00:37:15.550
Liel: Ubuntu?

373
00:37:15.550 --> 00:37:16.140
Gabor Szabo: Yeah.

374
00:37:16.520 --> 00:37:18.339
Liel: So please install this.

375
00:37:32.640 --> 00:37:36.209
Liel: Okay, that's great, and now let's try that again.

376
00:37:36.590 --> 00:37:39.070
Liel: Hopefully, it will build this time.

377
00:37:45.480 --> 00:37:47.499
Liel: Yeah, quite a bit of dependencies.

378
00:37:49.870 --> 00:37:50.840
Gabor Szabo: Yeah.

379
00:38:01.640 --> 00:38:03.499
Gabor Szabo: Okay, something is still missing.

380
00:38:03.500 --> 00:38:04.069
Liel: Another one.

381
00:38:04.070 --> 00:38:04.630
Gabor Szabo: grip.

382
00:38:06.330 --> 00:38:09.079
Gabor Szabo: GLIPC… GLIP 2.0.

383
00:38:10.690 --> 00:38:16.390
Liel: Yeah, it probably has… That…

384
00:38:16.700 --> 00:38:21.289
Liel: Wait, it has it in prerequisites… Linux.

385
00:38:25.360 --> 00:38:30.650
Liel: So, I'll post the entire install from the documentation.

386
00:38:32.060 --> 00:38:33.990
Liel: Feels like everything it needs.

387
00:38:35.120 --> 00:38:37.980
Liel: Most of which are probably existing, but

388
00:38:44.040 --> 00:38:48.159
Liel: Yeah, and in general, please don't, like, copy and paste

389
00:38:48.890 --> 00:38:51.330
Liel: Like that, but in this case…

390
00:38:51.330 --> 00:38:52.330
Gabor Szabo: I looked at it.

391
00:38:53.080 --> 00:38:57.049
Gabor Szabo: I mean, if, if… unless there are hidden characters there.

392
00:38:57.360 --> 00:39:02.020
Gabor Szabo: It looked, it looked right, I mean… It's just installing stuff.

393
00:39:05.550 --> 00:39:08.130
Liel: Yeah, I did install JLib, which is nice.

394
00:39:12.910 --> 00:39:15.920
Gabor Szabo: Can you link to where you copied this from?

395
00:39:16.990 --> 00:39:20.440
Gabor Szabo: Yeah, sure, can you insert the link as well? Share the link in the chat?

396
00:39:20.440 --> 00:39:25.129
Liel: It's from the official documentation, I just got… went to work, because it's…

397
00:39:25.700 --> 00:39:29.970
Gabor Szabo: But it's better if I then include that in the final notes.

398
00:39:30.410 --> 00:39:32.049
Gabor Szabo: The link there.

399
00:39:33.590 --> 00:39:37.939
Gabor Szabo: Okay, looks better. Oh, no, actually, that's just the information.

400
00:39:37.940 --> 00:39:48.000
Liel: about creating the app, what is the difference between the command cargo creator app? It's the same thing, just you are using cargo instead of NPM.

401
00:39:49.290 --> 00:39:52.549
Liel: So, you can absolutely use that as well, it's…

402
00:39:53.050 --> 00:39:56.409
Liel: It's essentially the same, whatever you are more comfortable with.

403
00:39:59.960 --> 00:40:02.859
Gabor Szabo: You're responding to this question in the chat, right?

404
00:40:02.860 --> 00:40:04.500
Liel: Yeah, stefano's question.

405
00:40:05.730 --> 00:40:06.350
Gabor Szabo: Yeah.

406
00:40:23.090 --> 00:40:24.260
Liel: Yeah, only.

407
00:40:25.050 --> 00:40:25.970
Liel: demand.

408
00:40:35.310 --> 00:40:41.610
Gabor Szabo: So I wonder what happened since the last time when we ran this. Did I replace my computer? No.

409
00:40:42.930 --> 00:40:44.690
Gabor Szabo: I mean, back then.

410
00:40:44.690 --> 00:40:47.519
Liel: Yeah, it has been a while, so I'm not sure.

411
00:40:48.710 --> 00:40:49.820
Gabor Szabo: Yeah, I don't remember.

412
00:40:49.820 --> 00:41:01.029
Liel: Yeah, now we can see this, and now please type something in the name, and once we go to the grid, it will say… it will actually run the command. Hello for you, you have been greeted.

413
00:41:01.340 --> 00:41:08.509
Liel: from Rust. Now, can you please leave that window, or that tab on the terminal open and open a new one?

414
00:41:08.690 --> 00:41:11.139
Liel: So we can, like, try to edit.

415
00:41:15.030 --> 00:41:16.299
Gabor Szabo: Yeah, you're here.

416
00:41:16.300 --> 00:41:19.469
Liel: Okay, so now, let's go to…

417
00:41:21.240 --> 00:41:23.959
Gabor Szabo: This is just… sorry, just checking this one, yeah?

418
00:41:24.210 --> 00:41:25.290
Liel: Yeah, that's okay.

419
00:41:25.650 --> 00:41:30.540
Liel: I think it's in SRC. Let's first change something in the front end.

420
00:41:35.490 --> 00:41:37.570
Liel: Yeah, let's go to main dot.

421
00:41:37.730 --> 00:41:39.780
Liel: TS, I guess?

422
00:41:40.750 --> 00:41:43.639
Gabor Szabo: Okay? Is it okay if I open it with Vim?

423
00:41:45.060 --> 00:41:46.090
Liel: Yeah, sure.

424
00:41:49.190 --> 00:41:57.540
Liel: Okay, so now we have… that, so you see we defined grid, and on DOMContentLoaded.

425
00:41:57.940 --> 00:41:59.489
Liel: It just does that.

426
00:42:04.530 --> 00:42:12.129
Liel: So… You know what? Let's actually close that file and change something in the HTML.

427
00:42:13.640 --> 00:42:15.330
Gabor Szabo: Okay, which is… which file?

428
00:42:16.840 --> 00:42:20.239
Liel: I think it's in SRC slash assets?

429
00:42:22.190 --> 00:42:25.020
Liel: Haven't done a vanilla one in a while.

430
00:42:26.430 --> 00:42:29.120
Liel: No. So, maybe it's in the root?

431
00:42:33.870 --> 00:42:34.680
Liel: Yeah.

432
00:42:34.680 --> 00:42:36.229
Gabor Szabo: There's an index extremely.

433
00:42:36.230 --> 00:42:37.950
Liel: edit index.html.

434
00:42:45.120 --> 00:42:47.340
Gabor Szabo: So this is going to be this window, right?

435
00:42:47.340 --> 00:42:57.370
Liel: I just checked the capstone… Yeah, so there is a hot reload, so yeah, you can change… Whatever.

436
00:43:02.890 --> 00:43:08.000
Gabor Szabo: Okay, and I didn't do anything reload, any reloading, it automatically reloaded, nice.

437
00:43:09.470 --> 00:43:21.989
Liel: Yeah, that's very nice. A really great developer experience. Now let's go and change something in the RAS code. So, let's close that, and it's in the SRC-tower.

438
00:43:30.410 --> 00:43:32.560
Gabor Szabo: Which file shall I open? The main orderly?

439
00:43:37.050 --> 00:43:42.399
Liel: Lib. Main is just like a thin wrapper. The code is actually in Lib RS.

440
00:43:51.040 --> 00:43:53.139
Gabor Szabo: Hmm. Why did it come up?

441
00:43:53.610 --> 00:43:54.970
Gabor Szabo: What happened to it?

442
00:43:59.570 --> 00:44:00.799
Gabor Szabo: It's suddenly showing down.

443
00:44:00.800 --> 00:44:02.630
Liel: We've done the squad.

444
00:44:05.100 --> 00:44:06.169
Gabor Szabo: Okay, what should I do?

445
00:44:10.440 --> 00:44:16.989
Liel: Let's change the message on line 4 to something. Greeted from Rasmans, let's say, yeah.

446
00:44:17.500 --> 00:44:21.340
Liel: And note that when he saved, it.

447
00:44:23.170 --> 00:44:23.820
Gabor Szabo: Group?

448
00:44:24.790 --> 00:44:25.380
Liel: Yeah.

449
00:44:25.540 --> 00:44:26.370
Liel: What?

450
00:44:26.910 --> 00:44:28.230
Liel: Let's try it.

451
00:44:30.790 --> 00:44:31.880
Gabor Szabo: Let's do this one.

452
00:44:31.880 --> 00:44:34.810
Liel: Okay, so you didn't save it yet. Note.

453
00:44:34.810 --> 00:44:37.610
Gabor Szabo: Oh, I think it's… oh, I didn't save it, sorry, yeah.

454
00:44:38.060 --> 00:44:38.780
Liel: Yeah, please.

455
00:44:41.220 --> 00:44:42.100
Gabor Szabo: Sorry.

456
00:44:43.140 --> 00:44:43.460
Liel: Yeah.

457
00:44:43.460 --> 00:44:44.339
Gabor Szabo: There is a little bit.

458
00:44:44.340 --> 00:44:45.300
Liel: But,

459
00:44:45.590 --> 00:44:56.230
Liel: Whenever you change the last code, it triggers a rebuild, and yeah, now it reopened the window, so it's a slightly less nice developer experience.

460
00:44:57.460 --> 00:44:58.070
Gabor Szabo: Yeah.

461
00:44:58.430 --> 00:45:00.290
Gabor Szabo: But it's here. Okay.

462
00:45:00.580 --> 00:45:01.529
Liel: Yeah, it's true.

463
00:45:02.010 --> 00:45:04.760
Liel: So, that's the demo.

464
00:45:05.760 --> 00:45:16.939
Liel: And yeah, with a few additional steps, and the… you can also do that for mobile, but we won't do that in here, because it's… it can be quite heavy, whereas either

465
00:45:18.620 --> 00:45:21.419
Liel: iOS we can do on Linux, and

466
00:45:22.170 --> 00:45:29.040
Liel: Android, it requires all the SDK stuff, which I don't want to waste time downloading.

467
00:45:31.370 --> 00:45:32.820
Liel: So, that's that.

468
00:45:33.250 --> 00:45:41.180
Liel: And I think that's, like, the basic demo, and that would be it. Thanks.

469
00:45:41.620 --> 00:45:43.839
Gabor Szabo: Okay. Shall I close everything?

470
00:45:44.840 --> 00:45:45.430
Liel: Yep.

471
00:45:53.850 --> 00:45:55.649
Gabor Szabo: Stop the sharing?

472
00:45:56.960 --> 00:45:57.620
Liel: No.

473
00:45:58.220 --> 00:45:58.910
Gabor Szabo: Okay, alright.

474
00:45:59.140 --> 00:46:00.230
Liel: Hooray.

475
00:46:00.530 --> 00:46:01.400
Liel: Yeah.

476
00:46:01.540 --> 00:46:08.080
Liel: I'll just share again, where is it? Portion of screen, and it stopped your very long.

477
00:46:08.080 --> 00:46:11.949
Gabor Szabo: You can look, you can… you can steal my sharing! Oh, I didn't know that.

478
00:46:13.190 --> 00:46:14.280
Gabor Szabo: Okay.

479
00:46:14.280 --> 00:46:14.930
Liel: Yeah.

480
00:46:15.120 --> 00:46:16.730
Liel: So that's the basics.

481
00:46:17.660 --> 00:46:26.539
Liel: And… As a product, the only way to communicate from the UI to the application core is via

482
00:46:27.980 --> 00:46:32.020
Liel: Or, it should be message passing.

483
00:46:32.260 --> 00:46:39.800
Liel: It isn't, like, actual… Actual, function call between the two.

484
00:46:40.320 --> 00:46:45.120
Liel: From the front-end perspective, all it does is a fetch call.

485
00:46:46.240 --> 00:46:53.930
Liel: like, the window.underscore underscore internals. What it does, you can also see it in the WorkPro, is, like, must fetch.

486
00:46:59.750 --> 00:47:04.899
Liel: So… Sorry, I did say it here, so it's done.

487
00:47:05.170 --> 00:47:15.049
Liel: Message pressing, and each message is also known as a command, as we say. Permissions allow us to filter what we can and cannot do.

488
00:47:15.350 --> 00:47:16.830
Liel: in each command.

489
00:47:19.330 --> 00:47:26.129
Liel: And we also can scope them. For example, if we have a file system plugin, and we don't want it to…

490
00:47:26.300 --> 00:47:29.630
Liel: Access anything beyond the home folder, we can do that.

491
00:47:30.330 --> 00:47:34.610
Liel: So, how it's done is we have, like, a TOML file.

492
00:47:36.690 --> 00:47:41.309
Liel: And, yeah. In here, let's say the identifier.

493
00:47:41.580 --> 00:47:44.109
Liel: is my identifier.

494
00:47:44.270 --> 00:47:59.460
Liel: The commands that are allowed for this particular plugin are read file, and it only allows home, and we can also say, like, we don't want home secret, or that could be your home slash dot SSH

495
00:47:59.970 --> 00:48:04.329
Liel: Folder, since you don't want the app to read your private key.

496
00:48:06.170 --> 00:48:18.999
Liel: And regarding the dollar home, I also found, like, unfortunately, I haven't found it in the documentation yet, but I found where it comes from.

497
00:48:19.380 --> 00:48:21.019
Liel: in the…

498
00:48:22.260 --> 00:48:35.050
Liel: code itself. So, yeah, it's in here, you have many, like, audio cache config, some of which are platform-specific, or some of which aren't available in Android, for example.

499
00:48:35.510 --> 00:48:36.620
Liel: Those ones?

500
00:48:40.980 --> 00:48:44.130
Liel: And in case it doesn't find…

501
00:48:44.840 --> 00:48:48.960
Liel: Then… it just doesn't substitute, so it…

502
00:48:49.250 --> 00:48:53.309
Liel: Just, let's say, Dollar Gabor Gabor, or Dollar Liel.

503
00:48:53.420 --> 00:48:57.969
Liel: It won't find it here, so it just will leave it at all real.

504
00:49:04.610 --> 00:49:09.170
Liel: That's that, and on top of permissions, we have capabilities.

505
00:49:10.340 --> 00:49:15.419
Liel: Which… Allow us to, like, set permissions?

506
00:49:16.180 --> 00:49:18.569
Liel: Per window label.

507
00:49:19.310 --> 00:49:21.870
Liel: So, label isn't the window writer?

508
00:49:25.460 --> 00:49:29.150
Liel: But… It's…

509
00:49:30.280 --> 00:49:38.920
Liel: something else you can tell, like, in this case, I don't think… or we just tell it to be specifically for main.

510
00:49:39.280 --> 00:49:41.680
Liel: We can also restrict it by platform.

511
00:49:43.550 --> 00:49:50.839
Liel: And so on. So at this point, you might ask, okay, so… but what does it protect against? Why do we need it?

512
00:49:51.290 --> 00:49:54.729
Liel: So, it depends on which permission capability, but…

513
00:49:55.450 --> 00:49:59.720
Liel: In case the front end is compromised, so let's say…

514
00:50:00.640 --> 00:50:03.720
Liel: let's say I, as an adversarial.

515
00:50:04.790 --> 00:50:07.790
Liel: Somehow managed to take over the front end?

516
00:50:10.070 --> 00:50:14.030
Liel: it allows to limit how much damage I can do.

517
00:50:16.250 --> 00:50:21.850
Liel: Or the file system is a great example, because if we read files.

518
00:50:22.060 --> 00:50:30.079
Liel: We might just want to make sure that the application cannot read anything it's not supposed to, even

519
00:50:30.500 --> 00:50:35.350
Liel: If, like, we control the code, I would still like to have that.

520
00:50:36.590 --> 00:50:40.230
Liel: Additional filter, in case someone takes over.

521
00:50:45.780 --> 00:50:53.729
Liel: Unfortunately, it doesn't protect us against code that's malicious inside the RAT.

522
00:50:55.420 --> 00:50:59.900
Liel: portion of our office, because that runs directly the core process.

523
00:51:08.240 --> 00:51:16.680
Liel: And also, it doesn't protect us from anything else, like, if we don't define the permissions correctly, or if we don't…

524
00:51:16.850 --> 00:51:19.769
Liel: like… Make sure to check them.

525
00:51:20.610 --> 00:51:26.080
Liel: then… Or we bypass their checks somewhere?

526
00:51:26.340 --> 00:51:29.850
Liel: then, tori want.

527
00:51:30.130 --> 00:51:31.739
Liel: won't protect us.

528
00:51:32.640 --> 00:51:38.760
Liel: The capabilities can be also defined in JSON, but permissions are… Strictly tunnel.

529
00:51:43.290 --> 00:51:57.950
Liel: There is also one more… Possibility for filtering commands before they reach the core process, And we'll…

530
00:51:58.790 --> 00:52:01.620
Liel: We'll take, like, a little look at that.

531
00:52:01.740 --> 00:52:08.669
Liel: So, what we've discussed… we've discussed so far is called the brownfield pattern for communication.

532
00:52:08.920 --> 00:52:18.530
Liel: And, yeah, it basically just forwards the… each command we send to the core, and then the core handles it.

533
00:52:19.680 --> 00:52:23.919
Liel: It still limits misuse, but we can do better if we want to.

534
00:52:24.920 --> 00:52:31.159
Liel: there is an isolation pattern, which is basically yet another JavaScript program we run.

535
00:52:32.360 --> 00:52:36.730
Liel: that… Does introduce some overhead.

536
00:52:38.010 --> 00:52:41.630
Liel: But it allows us to filter

537
00:52:42.790 --> 00:52:46.939
Liel: Before it even reaches the Rust core, if you wish to.

538
00:52:48.900 --> 00:52:57.019
Liel: So, it's basically another layer that… Sits between the… Webflow and the core.

539
00:53:06.840 --> 00:53:14.850
Liel: So, I think I talked about it a little bit before, but there are quite a lot of plugins that allow us to

540
00:53:15.000 --> 00:53:16.940
Liel: have… sound function.

541
00:53:24.780 --> 00:53:27.550
Gabor Szabo: Your voice disappeared? Yeah?

542
00:53:32.590 --> 00:53:35.790
Gabor Szabo: Okay… Something happened?

543
00:53:40.530 --> 00:53:41.680
Gabor Szabo: Clear?

544
00:53:47.450 --> 00:53:51.970
Gabor Szabo: Europe… Okay, you seem to be back now?

545
00:53:51.970 --> 00:53:57.200
Liel: Yeah, but I was mute, I was sound… Yeah.

546
00:53:57.420 --> 00:53:58.820
Liel: Oh, wow, thanks.

547
00:53:59.100 --> 00:54:01.780
Liel: Now… Now, do you hear me?

548
00:54:02.090 --> 00:54:03.529
Gabor Szabo: Yeah, I can hear you now.

549
00:54:03.910 --> 00:54:05.860
Liel: Okay, that's great.

550
00:54:06.630 --> 00:54:07.859
Liel: So, yeah.

551
00:54:08.020 --> 00:54:11.239
Liel: Plugins provide some functionality.

552
00:54:11.770 --> 00:54:13.030
Liel: Out of the box.

553
00:54:13.140 --> 00:54:16.200
Liel: So you don't need to, like, write it yourself.

554
00:54:17.100 --> 00:54:20.289
Liel: Some of them can be used from Rust as well.

555
00:54:20.400 --> 00:54:25.090
Liel: And some examples are for logging, and some deep links.

556
00:54:26.020 --> 00:54:34.210
Liel: So, DeepLinks allows us to set the app as a default handler for some protocol. So, let's say,

557
00:54:35.380 --> 00:54:37.330
Liel: Code Mavens.

558
00:54:37.990 --> 00:54:45.400
Liel: Or, yeah, call Mavens.com slash. If we want that, we can make sure that

559
00:54:46.280 --> 00:54:48.639
Liel: It opens those links in our app.

560
00:54:50.340 --> 00:54:53.240
Liel: Oh, and yeah.

561
00:54:56.320 --> 00:54:59.769
Liel: Sorry, there was a little question in the chat, I'll answer that shortly.

562
00:55:01.550 --> 00:55:09.439
Liel: And, yeah, so that's what DeepLink is for. Of course, the operating system just,

563
00:55:09.980 --> 00:55:15.319
Liel: Or it asks if we are okay with opening these links in our app.

564
00:55:17.110 --> 00:55:25.789
Liel: And there is the dialog plugin, and there's a lot more. So there is a list of all official plugins, and there's awesome directory for other links.

565
00:55:26.100 --> 00:55:27.180
Liel: we can use.

566
00:55:35.310 --> 00:55:45.610
Liel: We can also warp an existing project, so… For the second demo, What we'll do… is…

567
00:55:46.380 --> 00:55:50.870
Liel: I have a little app which I have done the front end for.

568
00:55:51.020 --> 00:55:53.569
Liel: And I'll extend it as a Tory app.

569
00:56:00.330 --> 00:56:03.129
Liel: For distribution and packaging.

570
00:56:03.380 --> 00:56:08.159
Liel: That's, like, the only thing before the demo.

571
00:56:09.660 --> 00:56:16.390
Liel: I'd like to point out that it does support generating packages, but you can do, like.

572
00:56:17.040 --> 00:56:20.590
Liel: You know, generate for different platforms.

573
00:56:22.710 --> 00:56:27.599
Liel: So, for example, if I'm on a Mac, I can generate for a Mac.

574
00:56:28.940 --> 00:56:35.249
Liel: And that's it. Or maybe if I spawn a virtual machine, then I can

575
00:56:35.630 --> 00:56:39.349
Liel: do, like, Linux, and potentially Windows.

576
00:56:39.560 --> 00:56:45.639
Liel: But I need, like, A native runner to do for each platform.

577
00:56:46.070 --> 00:56:52.509
Liel: So you can get around that with CI. There is, like, an official Tory tab auction, which does that.

578
00:56:53.780 --> 00:57:03.740
Liel: And again, there is the documentation for following AVIs. I think there is one experimental use case that allows

579
00:57:04.680 --> 00:57:07.860
Liel: from, I think, Mac to Linux?

580
00:57:10.330 --> 00:57:14.210
Liel: But, you know, that's not stable, at least.

581
00:57:14.780 --> 00:57:15.890
Liel: right now.

582
00:57:19.010 --> 00:57:19.940
Liel: Okay.

583
00:57:20.540 --> 00:57:24.149
Liel: And… now we have the demo!

584
00:57:26.230 --> 00:57:33.140
Liel: So, let me enlarge the… Screen a little bit…

585
00:57:33.790 --> 00:57:37.960
Liel: And, yeah, let's do it like that, and… Like that.

586
00:57:39.660 --> 00:57:46.610
Liel: So, let me first open… Z, which is my editor first.

587
00:57:49.190 --> 00:57:53.539
Liel: I've prepared a little app, in this case it will be a little clipboard manager.

588
00:57:55.790 --> 00:58:02.410
Liel: So, let's see how it looks. It will look a little bit differently, because…

589
00:58:02.600 --> 00:58:10.089
Liel: what happens is that I want it to be a little transparent, and it's not possible in the web browser, but in Tori it is.

590
00:58:10.560 --> 00:58:12.879
Liel: So what we'll do is npm run them.

591
00:58:14.940 --> 00:58:17.170
Liel: And once we go to that…

592
00:58:17.760 --> 00:58:31.279
Liel: We see that we have, like, two objects. These are, like, mock data. I will replace that with data we'll receive from an event. And once I click on something, I'd like it to be copied.

593
00:58:36.570 --> 00:58:38.660
Liel: Okay, so that's that.

594
00:58:39.660 --> 00:58:42.660
Liel: I'll leave their web browser just in case.

595
00:58:43.970 --> 00:58:47.430
Liel: I will… Poppy.

596
00:58:48.560 --> 00:58:49.850
Liel: this folder.

597
00:58:50.500 --> 00:58:52.670
Liel: into another…

598
00:58:57.550 --> 00:58:58.350
Liel: Peace.

599
00:58:58.720 --> 00:59:02.390
Liel: In my products folder, with my colleague.

600
00:59:07.220 --> 00:59:08.819
Liel: Let's copy that.

601
00:59:09.400 --> 00:59:13.730
Liel: And let's… Move to that folder.

602
00:59:18.750 --> 00:59:19.560
Liel: Yeah.

603
00:59:19.880 --> 00:59:23.450
Liel: That's that, and… Sorry?

604
00:59:23.570 --> 00:59:28.699
Liel: Trust. Trust and continue, and now… We have that folder.

605
00:59:29.060 --> 00:59:35.350
Liel: And here… So, one caveat I did notice is that sometimes the node modules

606
00:59:35.980 --> 00:59:41.210
Liel: isn't working right, so… let's do our… mode. Module?

607
00:59:42.070 --> 00:59:45.279
Liel: and NPMI to install.

608
00:59:45.780 --> 00:59:46.940
Liel: Everything, yeah.

609
00:59:47.850 --> 00:59:51.110
Liel: So it's 5173, we need that.

610
01:00:04.430 --> 01:00:11.410
Liel: For… yeah. Okay, so we did that, which is great. Now, I think it's cargo authority in it.

611
01:00:13.320 --> 01:00:16.140
Liel: And the app name is Clipman, that's right.

612
01:00:16.690 --> 01:00:19.180
Liel: Let's do a capital letter.

613
01:00:19.660 --> 01:00:26.300
Liel: web assets, it's correctly in this. And the fourth… 5173.

614
01:00:26.630 --> 01:00:27.959
Liel: That's for him.

615
01:00:29.310 --> 01:00:31.769
Liel: NPM run dev, npm run build…

616
01:00:32.210 --> 01:00:36.370
Liel: Everything is right, now we can see we have SRC total.

617
01:00:36.880 --> 01:00:43.070
Liel: which has the Torikov JSON, And the Libaris, as we saw earlier.

618
01:00:44.020 --> 01:00:48.420
Liel: Just, like, it doesn't register a command in here right now.

619
01:00:52.540 --> 01:00:55.390
Liel: So now I can run to Dev.

620
01:01:00.870 --> 01:01:03.579
Liel: Cargo, then, in this case?

621
01:01:11.080 --> 01:01:11.820
Liel: Yeah.

622
01:01:12.830 --> 01:01:15.250
Liel: Now, it's running.

623
01:01:17.070 --> 01:01:18.380
Liel: everything here…

624
01:01:23.920 --> 01:01:25.760
Liel: It will take a little while.

625
01:01:26.890 --> 01:01:36.179
Liel: And while it does that, let's add some settings so we can… Have it transparent.

626
01:01:37.480 --> 01:01:41.579
Liel: Sure, there is another setting specifically for Mac.

627
01:01:42.160 --> 01:01:44.910
Liel: Mac OS Private API, true.

628
01:01:45.030 --> 01:01:50.680
Liel: Note that, in this case, you can't upload it to the App Store, because… FO.

629
01:01:51.020 --> 01:01:52.640
Liel: Kinda hates devs.

630
01:01:52.780 --> 01:01:57.740
Liel: I guess… Sorry, I had to grab the tissue.

631
01:02:05.650 --> 01:02:10.010
Liel: Yeah, that build, and now you can see it is transparent.

632
01:02:10.460 --> 01:02:12.010
Liel: Which isn't nice.

633
01:02:16.270 --> 01:02:20.399
Liel: Yeah, I thought it would look nicer, but

634
01:02:20.960 --> 01:02:23.379
Liel: Yeah, that's okay for the demo.

635
01:02:27.890 --> 01:02:30.520
Liel: We can always tweak it later.

636
01:02:32.880 --> 01:02:42.309
Liel: Now… For an actual clipboard to function, we need a library that accesses the clipboard.

637
01:02:42.700 --> 01:02:45.209
Liel: So, there is one called Our Board.

638
01:02:45.920 --> 01:02:47.819
Liel: Which is nice.

639
01:02:49.270 --> 01:02:59.700
Liel: But, also wrapped in… Or Cargo Tari ad, yeah, in a plug called Clipboard Manager.

640
01:03:00.390 --> 01:03:13.550
Liel: it uses our board internally, and I'd just like to show you how easy it is to install any plugin. And if you check out the Libra S, you see that it already

641
01:03:13.760 --> 01:03:17.089
Liel: edit that. So, it's really nice.

642
01:03:18.030 --> 01:03:20.950
Liel: Let's now do a cargo. Dev?

643
01:03:22.300 --> 01:03:27.389
Liel: So we can have Vlad in place.

644
01:03:31.830 --> 01:03:34.899
Liel: Yeah, by the way, this is a live demo, so…

645
01:03:35.580 --> 01:03:43.639
Liel: Hopefully, everything goes right, but it can go wrong, and hopefully, in this case, you could learn from that too.

646
01:03:45.880 --> 01:03:56.090
Liel: Okay, so inside the setup, Debug assertions, that's okay, that's… That just allows us to… Long?

647
01:03:58.690 --> 01:04:02.000
Liel: And in here, Elm… app.

648
01:04:02.490 --> 01:04:03.340
Liel: Don't.

649
01:04:06.680 --> 01:04:13.499
Liel: handle… that will be the app handle, and what we need is storage, yeah.

650
01:04:13.740 --> 01:04:16.740
Liel: A sync runtime, which is okay.

651
01:04:18.180 --> 01:04:19.190
Liel: Spawn?

652
01:04:21.710 --> 01:04:23.130
Liel: And…

653
01:04:28.240 --> 01:04:35.790
Liel: Let's do… no, let's do spawn blocking, because we might block the executor. Unfortunately, the…

654
01:04:36.080 --> 01:04:41.880
Liel: The way the people library works is that we listen for an event, and it might block.

655
01:04:42.180 --> 01:04:50.480
Liel: So… I want to… With that…

656
01:05:01.990 --> 01:05:04.139
Liel: Spawn blocking…

657
01:05:13.370 --> 01:05:16.480
Liel: Maybe just that. Yeah, broadly just that.

658
01:05:17.100 --> 01:05:20.649
Liel: So, in this case, we want… To run in a loop.

659
01:05:21.880 --> 01:05:28.820
Liel: And we need the app handle, so let's… handle?

660
01:05:29.300 --> 01:05:32.920
Liel: app.handle, and we can also clone it.

661
01:05:33.120 --> 01:05:42.920
Liel: Internally, it uses an arc, an atomic reference counter, so that's relatively cheap. It's not free, but it should be okay.

662
01:05:45.440 --> 01:05:49.690
Liel: app. Handle.clip.

663
01:05:50.390 --> 01:05:51.370
Liel: Bird?

664
01:05:52.330 --> 01:05:53.290
Liel: dot.

665
01:05:53.520 --> 01:05:55.130
Liel: Read text.

666
01:05:59.820 --> 01:06:02.520
Liel: So, this returns a result.

667
01:06:06.030 --> 01:06:08.290
Liel: Let's match on that.

668
01:06:11.560 --> 01:06:12.780
Liel: Okay.

669
01:06:14.080 --> 01:06:16.069
Liel: Text.

670
01:06:16.920 --> 01:06:17.969
Liel: Do that.

671
01:06:18.170 --> 01:06:19.270
Liel: Error…

672
01:06:24.540 --> 01:06:30.979
Liel: Let's just do, or… Sorry. Login. Login?

673
01:06:39.290 --> 01:06:40.599
Liel: Love it done.

674
01:06:42.660 --> 01:06:43.530
Liel: Sure.

675
01:06:49.150 --> 01:06:51.620
Liel: And this is probably tracing.

676
01:06:55.040 --> 01:07:00.110
Liel: So, let's do… Who is not.

677
01:07:01.920 --> 01:07:03.309
Liel: Pick text.

678
01:07:03.580 --> 01:07:12.639
Liel: Or, you know what? I'm not sure that's a good idea, because that can potentially just flood our laws with

679
01:07:13.230 --> 01:07:17.350
Liel: That, so let's… Put that in a comment for now.

680
01:07:17.500 --> 01:07:20.130
Liel: And in here, let's do app.

681
01:07:20.650 --> 01:07:21.680
Liel: Candle?

682
01:07:23.340 --> 01:07:24.070
Liel: Oh.

683
01:07:24.380 --> 01:07:26.590
Liel: That needs to go to the side.

684
01:07:26.930 --> 01:07:28.510
Liel: Handle. Dot.

685
01:07:29.500 --> 01:07:30.550
Liel: Commit.

686
01:07:32.120 --> 01:07:33.689
Liel: And… yeah.

687
01:07:34.520 --> 01:07:35.480
Liel: Text.

688
01:07:37.750 --> 01:07:38.860
Liel: profit?

689
01:07:39.530 --> 01:07:42.629
Liel: And the payload, we are just our text.

690
01:07:49.740 --> 01:07:51.350
Liel: Okay, apparently.

691
01:07:52.030 --> 01:07:54.550
Liel: It also leads to a result.

692
01:07:54.960 --> 01:07:58.229
Liel: But I can ignore that for the sake of the demo.

693
01:08:00.320 --> 01:08:05.870
Liel: And yeah, it's quite annoying that… It…

694
01:08:06.080 --> 01:08:10.090
Liel: Restartes every time, but, yeah.

695
01:08:11.350 --> 01:08:14.340
Liel: At least there is some kind of utter loading.

696
01:08:14.570 --> 01:08:18.429
Liel: Hopefully, they make that even better in the future.

697
01:08:19.100 --> 01:08:21.340
Liel: So, in rapp.

698
01:08:21.830 --> 01:08:24.330
Liel: View what we need. Next.

699
01:08:24.810 --> 01:08:26.400
Liel: Is we need to.

700
01:08:28.410 --> 01:08:30.040
Liel: Remove this.

701
01:08:33.740 --> 01:08:41.460
Liel: Yeah, let's just add any in here, because… Oh.

702
01:08:41.760 --> 01:08:43.929
Liel: It can't add that.

703
01:08:49.330 --> 01:08:50.830
Liel: Or… yeah.

704
01:08:51.700 --> 01:08:55.569
Liel: Let's see what… It does.

705
01:08:58.930 --> 01:09:00.120
Liel: Where is it?

706
01:09:01.779 --> 01:09:06.400
Liel: Oh, it's just empty, so it doesn't show anything right now.

707
01:09:06.930 --> 01:09:08.479
Liel: So I missed it.

708
01:09:09.609 --> 01:09:18.880
Liel: Let's do… where is Tory? I'm sorry, I have to consult the documentation, because I don't remember the exact syntax for that.

709
01:09:20.600 --> 01:09:23.280
Liel: No, it's nothing learned, it's developed.

710
01:09:28.060 --> 01:09:29.950
Liel: It is process clean.

711
01:09:35.580 --> 01:09:39.090
Liel: Because I need to find how the event

712
01:09:43.979 --> 01:09:44.770
Liel: Oop.

713
01:09:45.950 --> 01:09:47.180
Liel: That's that.

714
01:09:47.319 --> 01:09:53.380
Liel: And also, in the… In the presentation.

715
01:09:53.819 --> 01:09:55.510
Liel: Sorry. That.

716
01:09:59.400 --> 01:10:03.440
Liel: That, and yeah, so we can import the…

717
01:10:03.860 --> 01:10:09.460
Liel: No, not in Vogue, but probably the… Listen.

718
01:10:09.700 --> 01:10:10.430
Liel: Fine.

719
01:10:11.480 --> 01:10:13.900
Liel: Yeah, global events, and yeah.

720
01:10:14.390 --> 01:10:18.349
Liel: Okay, so we need to import that, and we need to use that.

721
01:10:18.630 --> 01:10:21.900
Liel: So… listen…

722
01:10:34.610 --> 01:10:36.540
Liel: story apps.

723
01:10:36.660 --> 01:10:38.119
Liel: slash API.

724
01:10:41.870 --> 01:10:43.730
Liel: That's that.

725
01:10:45.180 --> 01:10:53.800
Liel: Yeah, it complains because I didn't listen to an event. Listen to… How did we call it?

726
01:10:53.950 --> 01:10:55.480
Liel: Text copied.

727
01:11:00.220 --> 01:11:04.579
Liel: And the second argument is the

728
01:11:08.340 --> 01:11:11.410
Liel: event. So, event.

729
01:11:16.480 --> 01:11:18.029
Liel: Let's do that.

730
01:11:25.520 --> 01:11:31.459
Liel: And, yeah, we should probably… Import it, in case it doesn't…

731
01:11:44.640 --> 01:11:50.149
Liel: Let's import it like that, so it doesn't clash with anything.

732
01:11:51.340 --> 01:11:54.319
Liel: And in this case, we just have a string, so…

733
01:11:54.580 --> 01:11:57.839
Liel: In any case, we have text copied.

734
01:11:59.660 --> 01:12:05.100
Liel: What we can do is items.push, Okay.

735
01:12:05.300 --> 01:12:07.090
Liel: items.value.

736
01:12:09.270 --> 01:12:10.779
Liel: Alright, let's do it.

737
01:12:15.310 --> 01:12:18.469
Liel: Yeah, I'm just not sure.

738
01:12:18.960 --> 01:12:24.360
Liel: it will work with views reactivity, but let's try it.

739
01:12:28.550 --> 01:12:29.870
Liel: ID.

740
01:12:32.810 --> 01:12:36.909
Liel: random UUID, we can just use that.

741
01:12:38.980 --> 01:12:39.780
Liel: Time.

742
01:12:40.140 --> 01:12:45.739
Liel: just a student for text. We have some support for images, but…

743
01:12:47.260 --> 01:12:55.609
Liel: It's not implemented yet. Let's see if we have time for that. And content will just be event.payload.

744
01:12:56.110 --> 01:12:58.380
Liel: Which is correctly string.

745
01:12:59.610 --> 01:13:01.420
Liel: And we have that.

746
01:13:01.990 --> 01:13:05.359
Liel: Now, let's see if that actually works.

747
01:13:06.810 --> 01:13:10.450
Liel: But yeah, one thing I don't like is that…

748
01:13:10.820 --> 01:13:14.860
Liel: We don't have any sleep, so what we can do?

749
01:13:18.160 --> 01:13:19.549
Liel: He asleep?

750
01:13:22.560 --> 01:13:24.530
Liel: Here, we can do sleep.

751
01:13:25.040 --> 01:13:33.470
Liel: Like that, because… Oz… We are in a blocking thread, duration. Done.

752
01:13:33.800 --> 01:13:35.440
Liel: Error 4.

753
01:13:37.570 --> 01:13:38.930
Liel: Mail is?

754
01:13:39.220 --> 01:13:41.600
Liel: Let's do, like, 2?

755
01:13:46.090 --> 01:13:47.980
Liel: mismatched types.

756
01:13:49.000 --> 01:13:51.739
Liel: Okay, so you're 64, so…

757
01:13:59.440 --> 01:14:05.420
Liel: Okay, now… Oh, sorry, I just wanted to copy something.

758
01:14:07.610 --> 01:14:10.470
Liel: And, apparently, it doesn't work.

759
01:14:13.120 --> 01:14:16.589
Liel: Import binding name is not found.

760
01:14:18.040 --> 01:14:19.060
Liel: Okay.

761
01:14:20.240 --> 01:14:21.360
Liel: Oh.

762
01:14:22.760 --> 01:14:26.849
Liel: So we should do something like that, import.

763
01:14:27.020 --> 01:14:27.810
Liel: Type?

764
01:14:29.320 --> 01:14:30.130
Liel: That.

765
01:14:38.130 --> 01:14:40.530
Liel: Okay. That didn't work.

766
01:14:41.430 --> 01:14:45.579
Liel: Which I suspect is related to that.

767
01:14:51.130 --> 01:14:55.960
Liel: Back to the activity thing I mentioned earlier, so let's do that.

768
01:14:56.590 --> 01:14:58.699
Liel: I know it's a rust step.

769
01:15:06.580 --> 01:15:15.020
Liel: It's the worst thing, but… Unfortunately, this can be complex to debug.

770
01:15:16.620 --> 01:15:18.210
Liel: Let's do that together.

771
01:15:19.450 --> 01:15:26.500
Liel: In the case of that… I'd like to have… Some kind of error.

772
01:15:26.790 --> 01:15:35.619
Liel: or some kind of that. So, what I have here is… That, or let's do info.

773
01:15:38.130 --> 01:15:38.950
Liel: Fantastic.

774
01:15:41.220 --> 01:15:43.710
Liel: Just to see if that works.

775
01:15:47.700 --> 01:15:49.990
Liel: Yeah, it runs.

776
01:15:50.370 --> 01:15:56.340
Liel: That's that, yeah. As you can see, it does spam me quite often.

777
01:15:58.590 --> 01:15:59.869
Liel: Let's do that.

778
01:16:00.870 --> 01:16:03.810
Liel: Successfully grab the text.

779
01:16:11.000 --> 01:16:13.380
Liel: Okay, so he does that as well.

780
01:16:18.650 --> 01:16:19.969
Liel: Remove that.

781
01:16:22.590 --> 01:16:25.759
Liel: Oh, maybe I just need to eliminate it.

782
01:16:29.450 --> 01:16:30.750
Liel: Oh, let's see.

783
01:16:35.320 --> 01:16:40.520
Liel: Okay, let's… dot meet…

784
01:16:45.210 --> 01:16:49.390
Liel: It's actually bad, that result.

785
01:17:03.950 --> 01:17:06.030
Liel: So, it does succeed.

786
01:17:31.750 --> 01:17:37.890
Liel: Oh, so the ID is a problem, so let's have… Last idea.

787
01:17:47.680 --> 01:17:51.190
Liel: And here… Rusty.

788
01:17:53.430 --> 01:17:55.650
Liel: And do that.

789
01:18:04.160 --> 01:18:07.170
Liel: So, it's converted to a string.

790
01:18:10.520 --> 01:18:12.699
Liel: But even that didn't work.

791
01:18:14.160 --> 01:18:17.800
Liel: So let's… That's… oh.

792
01:18:24.810 --> 01:18:26.680
Liel: That's worth…

793
01:18:41.620 --> 01:18:43.050
Liel: That's unfortunate.

794
01:18:43.810 --> 01:18:46.500
Liel: Let's see, listen…

795
01:18:52.480 --> 01:18:52.885
Liel: What?

796
01:18:58.400 --> 01:19:05.170
Liel: Okay, so… Technically, it should work, but I'm not sure why it doesn't.

797
01:19:07.630 --> 01:19:12.660
Liel: Let's, first of all… Do… sleep.

798
01:19:13.100 --> 01:19:14.250
Liel: Seconds.

799
01:19:15.330 --> 01:19:16.780
Liel: Maybe 10 seconds?

800
01:19:32.710 --> 01:19:33.540
Liel: Oh.

801
01:19:43.410 --> 01:19:50.860
Liel: Yeah, so as you can see now, I can see the text copy fired, so maybe I was just flooding it too much.

802
01:19:56.370 --> 01:19:58.649
Liel: So, if you can see, I will just copy

803
01:20:00.790 --> 01:20:03.530
Liel: That, the Congo book, for example.

804
01:20:05.940 --> 01:20:08.450
Liel: Just so you can see it's not…

805
01:20:12.360 --> 01:20:14.959
Liel: Yeah, you can see it's the cargo book.

806
01:20:16.370 --> 01:20:24.100
Liel: And if I look at the dev tools, Let's check the upstate.

807
01:20:24.410 --> 01:20:28.880
Liel: Yeah. We now probably have some reactivity issue.

808
01:20:29.100 --> 01:20:30.739
Liel: in that,

809
01:20:57.730 --> 01:21:05.759
Liel: So, let's see… And default console lob items.

810
01:21:05.980 --> 01:21:10.549
Liel: Liel? Yes, so… We can see it.

811
01:21:11.490 --> 01:21:14.459
Liel: Let's remove that, and inspect elementary.

812
01:21:21.340 --> 01:21:23.350
Liel: Okay, so it's a proxy.

813
01:21:23.490 --> 01:21:26.289
Liel: So, technically, it should.

814
01:21:33.970 --> 01:21:35.030
Liel: Show it?

815
01:21:37.450 --> 01:21:41.479
Liel: And also, let's make sure we aren't copying the same thing.

816
01:21:41.960 --> 01:21:48.400
Liel: So, what we can do… is just… if… Items.

817
01:21:49.080 --> 01:21:53.450
Liel: dot add, or items.value.ad equals 1.

818
01:21:53.750 --> 01:21:55.990
Liel: dot content.

819
01:21:57.090 --> 01:22:01.309
Liel: is equal to event payload, just for an example.

820
01:22:08.160 --> 01:22:09.779
Liel: Yeah, yeah.

821
01:22:12.450 --> 01:22:14.919
Liel: Let's add that so it doesn't complain.

822
01:22:15.170 --> 01:22:17.100
Liel: If it's undefined…

823
01:22:22.600 --> 01:22:25.440
Liel: Okay, so now we have that.

824
01:22:25.940 --> 01:22:33.549
Liel: And inside app.view, Item of items…

825
01:22:36.160 --> 01:22:39.400
Liel: Type is text, which is correct.

826
01:22:52.220 --> 01:22:54.449
Liel: Okay, so, yeah.

827
01:23:02.990 --> 01:23:05.230
Liel: It doesn't like the async.

828
01:23:08.590 --> 01:23:12.700
Liel: So how can I do that? Let's do…

829
01:23:28.100 --> 01:23:37.770
Liel: How can I do that? Let me just, like, copy the file, duplicate, and let's call it… Bye.

830
01:23:38.320 --> 01:23:41.169
Liel: Or, no, but it isn't a great name.

831
01:23:41.830 --> 01:23:42.590
Liel: Hmm.

832
01:23:49.420 --> 01:23:56.159
Liel: In here… Yeah, the import can go there, and I can remove the styling.

833
01:23:56.560 --> 01:24:04.370
Liel: But in app… What I'll need to do is, first of all, remove all this setup from here.

834
01:24:05.220 --> 01:24:09.040
Liel: Substance… And, up, and up.

835
01:24:14.230 --> 01:24:18.140
Liel: See, we're… Is that in here?

836
01:24:25.980 --> 01:24:33.810
Liel: Okay, so if I clear all filters… Maybe it needs some reload?

837
01:24:41.580 --> 01:24:42.990
Liel: Okay.

838
01:24:47.460 --> 01:24:53.190
Liel: That's just… Add a suspense.

839
01:24:53.290 --> 01:24:55.220
Liel: Again, this is not a view.

840
01:25:05.030 --> 01:25:08.210
Liel: So, yeah, let's try that a little bit.

841
01:25:25.950 --> 01:25:32.140
Liel: Okay, async setup. That's right… Let me add…

842
01:25:33.220 --> 01:25:38.080
Liel: One item to that. So, we'll do… Post.

843
01:25:38.490 --> 01:25:41.050
Liel: One ending here.

844
01:25:42.140 --> 01:25:44.300
Liel: Next Indeed.

845
01:25:44.740 --> 01:25:47.879
Liel: Is just zero content.

846
01:25:50.150 --> 01:25:53.510
Liel: Let's just add that, so we have something…

847
01:25:59.330 --> 01:26:00.330
Liel: Yeah.

848
01:26:00.650 --> 01:26:07.469
Liel: Okay, let's also remove the transparency, because that… that looks nicer.

849
01:26:07.750 --> 01:26:12.060
Liel: But… Can make it harder to debug.

850
01:26:17.730 --> 01:26:19.410
Liel: We have that…

851
01:26:25.110 --> 01:26:26.729
Liel: And it still complains.

852
01:26:37.470 --> 01:26:39.810
Liel: Oh. I.

853
01:26:40.670 --> 01:26:43.300
Liel: I made a really stupid mistake.

854
01:26:51.630 --> 01:26:54.970
Liel: I want that in the inner component.

855
01:26:55.360 --> 01:27:01.250
Liel: And I want to, like, remove All of the rest…

856
01:27:03.500 --> 01:27:09.850
Liel: Yeah, here in the root component, first of all, I don't want All this setup.

857
01:27:10.750 --> 01:27:17.099
Liel: And in the template… All I want is to have a suspense.

858
01:27:19.500 --> 01:27:21.239
Liel: And up, you know.

859
01:27:28.390 --> 01:27:29.870
Liel: Where is our app?

860
01:27:30.100 --> 01:27:31.010
Liel: Here.

861
01:27:32.860 --> 01:27:38.769
Liel: Oh, fail to resolve component, or I just need in the…

862
01:27:42.570 --> 01:27:48.309
Liel: Airport… Look at that.

863
01:27:53.580 --> 01:27:54.810
Liel: Okay.

864
01:27:57.930 --> 01:28:05.200
Liel: And now, if I, let's say, copy VIT, You see, I have it.

865
01:28:05.950 --> 01:28:10.640
Liel: Yeah, sorry it took a little bit of time, but hopefully you got something from it.

866
01:28:11.760 --> 01:28:13.190
Liel: And yeah, the…

867
01:28:13.190 --> 01:28:13.930
Gabor Szabo: Nice.

868
01:28:16.740 --> 01:28:21.129
Liel: The view DevTools, I… yeah, I just removed that.

869
01:28:21.700 --> 01:28:23.369
Liel: We have that.

870
01:28:25.070 --> 01:28:30.409
Liel: And let's do one command, because we actually want to use that.

871
01:28:31.080 --> 01:28:37.300
Liel: So… what we need to do first. Oh, okay, time.

872
01:28:37.440 --> 01:28:38.660
Liel: ESTMS?

873
01:28:38.770 --> 01:28:40.679
Liel: Just so he doesn't complain.

874
01:28:41.740 --> 01:28:45.820
Liel: And what we can do is first register a command.

875
01:28:47.700 --> 01:28:50.870
Liel: So, in here… It can do.

876
01:28:51.340 --> 01:28:52.290
Liel: Commando?

877
01:28:53.670 --> 01:28:54.390
Liel: Wow.

878
01:28:58.120 --> 01:28:58.790
Liel: Sweet.

879
01:29:03.430 --> 01:29:06.349
Liel: And in here, let's do text string.

880
01:29:10.090 --> 01:29:16.349
Liel: Result… Well, let's not do a result in here. Let's just do it vanilla.

881
01:29:17.240 --> 01:29:27.369
Liel: And, we also need the app handle to get the clipboard, so what we can do is…

882
01:29:31.400 --> 01:29:37.529
Liel: And just… We just need to add that.

883
01:29:38.670 --> 01:29:44.640
Liel: In here… Importory app handle, that's great.

884
01:29:47.120 --> 01:29:50.300
Liel: And why is it defined?

885
01:29:50.410 --> 01:29:52.540
Liel: Multiple times…

886
01:29:57.560 --> 01:30:01.830
Liel: Oh, maybe I just don't need the… We pop.

887
01:30:02.300 --> 01:30:05.570
Liel: Yeah, I don't need the popcorn.

888
01:30:06.770 --> 01:30:10.989
Liel: So, next… Yeah, it reopened.

889
01:30:14.100 --> 01:30:18.500
Liel: and handle. Click. Bird.

890
01:30:19.210 --> 01:30:20.260
Liel: Right.

891
01:30:20.660 --> 01:30:21.680
Liel: Text?

892
01:30:21.880 --> 01:30:22.780
Liel: Next.

893
01:30:23.280 --> 01:30:27.390
Liel: So… for… this. I don't want this to be…

894
01:30:29.430 --> 01:30:33.540
Liel: much longer, so I'll just support text right now.

895
01:30:35.060 --> 01:30:38.960
Liel: So, okay, one last, or… Yeah.

896
01:30:39.730 --> 01:30:46.449
Liel: Let's ignore the warning here, that I don't… Really care about, for now.

897
01:30:50.460 --> 01:30:55.350
Liel: So, what I need to do next is actually call the command.

898
01:30:55.790 --> 01:30:59.380
Liel: So, first of all, I need to, like, register it.

899
01:31:02.400 --> 01:31:03.930
Liel: I think…

900
01:31:09.180 --> 01:31:12.229
Liel: Yeah, I actually need to edit in the setup.

901
01:31:13.010 --> 01:31:15.350
Liel: Let me quickly reference.

902
01:31:15.830 --> 01:31:16.570
Liel: Where?

903
01:31:17.220 --> 01:31:18.220
Liel: that.

904
01:31:18.440 --> 01:31:19.580
Liel: Should go.

905
01:31:25.490 --> 01:31:32.070
Liel: Commands, okay, custom command… And yeah, we need to add the invoke handler.

906
01:31:32.590 --> 01:31:33.620
Liel: Okay.

907
01:31:34.170 --> 01:31:38.620
Liel: So, in here.invokeHandler, generate.

908
01:31:39.050 --> 01:31:40.760
Liel: Headdler!

909
01:31:41.520 --> 01:31:44.380
Liel: And copy will be done.

910
01:31:52.400 --> 01:31:53.749
Liel: Now we can.

911
01:31:55.330 --> 01:31:56.460
Liel: Do that.

912
01:31:57.670 --> 01:32:10.610
Liel: And… In our app… So here, we can have… we have clicked… And event is… Just…

913
01:32:10.920 --> 01:32:15.360
Liel: item.ai, so we need to find the item.

914
01:32:27.310 --> 01:32:30.510
Liel: If we don't find it, I don't really care.

915
01:32:37.280 --> 01:32:44.130
Liel: Now, we need… keyboard in Vogue.

916
01:32:46.740 --> 01:32:50.809
Liel: Which will allow us to… Invoke the command.

917
01:32:51.730 --> 01:33:03.310
Liel: Now let's… I think it needs to be a sync as well, because it returns a promise whatsoever, or… yeah, whatsoever isn't the right word, but you get the point.

918
01:33:05.570 --> 01:33:14.030
Liel: Let's… Do that, so invoke… And… How did I call it?

919
01:33:14.930 --> 01:33:16.519
Liel: Copy the clipboard.

920
01:33:20.250 --> 01:33:30.299
Liel: I think it's dashes instead of underscores, and… The argument is item.

921
01:33:37.220 --> 01:33:42.790
Liel: So it should be string… Let's just use the screen. Again, don't…

922
01:33:51.630 --> 01:33:59.880
Liel: Yeah, okay, so copycat label. We have that. Now I copied dependencies, so we'll have that as well in a few seconds.

923
01:34:01.320 --> 01:34:02.350
Liel: Yes.

924
01:34:02.900 --> 01:34:16.169
Liel: And now, let's check if that works. So, the way we do that is, let's say I open a new terminal. Now, the thing I have is dependencies, which is okay. And now, if I click it.

925
01:34:16.950 --> 01:34:18.369
Liel: I did not work.

926
01:34:18.520 --> 01:34:20.669
Liel: Okay, so another…

927
01:34:24.560 --> 01:34:30.130
Liel: Command, copy to clipboard, not found. Maybe it is with under… scores?

928
01:34:37.550 --> 01:34:39.090
Liel: Let's check now.

929
01:34:39.330 --> 01:34:45.250
Liel: Now we have dependencies, which is okay, but I want something else. Let's say the name of the folder.

930
01:34:51.850 --> 01:34:52.770
Liel: Yes.

931
01:34:53.380 --> 01:34:59.640
Liel: Now, I have that in Clickboard. Once I type dependencies, that still doesn't work.

932
01:35:00.820 --> 01:35:02.540
Liel: Okay, what's next?

933
01:35:03.030 --> 01:35:08.940
Liel: Invalid arguments… oh, so it requires me to have it.

934
01:35:10.390 --> 01:35:11.670
Liel: Like that.

935
01:35:12.240 --> 01:35:13.260
Liel: Text.

936
01:35:19.520 --> 01:35:24.930
Liel: I think, let me just reference my own presentation once again.

937
01:35:31.080 --> 01:35:35.119
Liel: I don't have it in here, but I have it here.

938
01:35:35.470 --> 01:35:41.160
Liel: Invoke command… Passing ordinance. That's what they need.

939
01:35:41.650 --> 01:35:43.669
Liel: Yeah, exactly, Isaia.

940
01:35:44.940 --> 01:35:47.120
Liel: Try to do. Text.

941
01:35:47.590 --> 01:35:51.820
Liel: Not that, but item.content.

942
01:35:54.260 --> 01:35:57.789
Liel: And let's ignore item.

943
01:36:04.600 --> 01:36:05.320
Liel: Yeah.

944
01:36:05.420 --> 01:36:07.980
Liel: Let's ignore anything that isn't a string.

945
01:36:10.520 --> 01:36:16.089
Liel: Now, let's see… yeah, I populated that, and let's copy dependencies as well.

946
01:36:21.460 --> 01:36:22.480
Liel: Great.

947
01:36:22.880 --> 01:36:25.609
Liel: Now we have dependencies, once I do that…

948
01:36:26.400 --> 01:36:30.340
Liel: And we have a working little clipboard manager.

949
01:36:31.050 --> 01:36:38.709
Liel: Of course, it doesn't actually have, like, that smart of a… brain or state.

950
01:36:39.100 --> 01:36:45.470
Liel: But… At least it works, and I think that's enough for a demo right now.

951
01:36:48.390 --> 01:36:50.070
Liel: So let's close that.

952
01:36:52.830 --> 01:36:59.260
Liel: Would you, if you'd like, I could also upload this demo afterwards to Git?

953
01:37:01.010 --> 01:37:05.269
Gabor Szabo: Yeah, I think it has probably more value than what we did in the first demo.

954
01:37:07.670 --> 01:37:08.320
Liel: Yeah.

955
01:37:10.940 --> 01:37:19.300
Liel: So, we were here… That's… do that.

956
01:37:20.230 --> 01:37:27.460
Liel: So, you'll have some additional links here, documentation, Oscari, their GitHub, and their Discord.

957
01:37:27.690 --> 01:37:35.059
Liel: There are some real nice people here, so in case you have any questions about Tori.

958
01:37:35.290 --> 01:37:39.880
Liel: They're probably… much more skilled than I am.

959
01:37:42.050 --> 01:37:44.840
Liel: So, feel free to use that.

960
01:37:45.840 --> 01:37:47.630
Liel: And… yeah.

961
01:37:47.810 --> 01:37:59.159
Liel: Thanks, that would be it for my contact info. You have my email, you have my links in here to GitHub and LinkedIn as well. And, yeah, I'll export the…

962
01:38:00.310 --> 01:38:05.499
Liel: the presentation slides to PDF, so people can put that into…

963
01:38:07.330 --> 01:38:08.210
Gabor Szabo: Excellent.

964
01:38:08.990 --> 01:38:10.219
Gabor Szabo: Excellent. Okay.

965
01:38:10.630 --> 01:38:12.609
Gabor Szabo: Any questions? Any of you?

966
01:38:12.610 --> 01:38:13.240
Liel: Yeah.

967
01:38:16.410 --> 01:38:19.420
Gabor Szabo: Okay, I think people got really tired already.

968
01:38:20.000 --> 01:38:20.860
Gabor Szabo: This was a…

969
01:38:20.860 --> 01:38:22.470
Liel: Yeah, that makes sense.

970
01:38:22.470 --> 01:38:24.300
Gabor Szabo: Have you, have you watched 10 kids?

971
01:38:24.300 --> 01:38:26.250
Liel: Thank you for, listening.

972
01:38:26.770 --> 01:38:36.290
Gabor Szabo: Yeah, I would like to thank you for giving this presentation. It was, deep, and,

973
01:38:37.570 --> 01:38:42.849
Gabor Szabo: I guess it seems that there are no more questions from the audience, so…

974
01:38:43.110 --> 01:38:45.430
Gabor Szabo: We can finish this, right?

975
01:38:46.850 --> 01:38:48.300
Gabor Szabo: What are you saying?

976
01:38:48.610 --> 01:38:54.999
Liel: Yeah, yeah, we can stop the recording, and I'll stay for a little while in case someone wants to.

977
01:38:55.410 --> 01:38:59.790
Gabor Szabo: Yeah, so again, thank you very much for, for doing this, for the.

978
01:38:59.790 --> 01:39:00.549
Liel: Always a pleasure.

979
01:39:00.950 --> 01:39:14.369
Gabor Szabo: Yeah, thank you. For the people watching it, please like the video, and follow the channel, and check out below the video, you will find links to… to the various things, that, probably the slides, and,

980
01:39:14.370 --> 01:39:30.680
Gabor Szabo: to their LinkedIn profile of Liel, and you can contact him as well. So thank you, and thank you very much for all those people who were here in the audience. It's always good to have audience, instead of just…

981
01:39:30.790 --> 01:39:33.059
Gabor Szabo: Speaking to a video.

982
01:39:33.520 --> 01:39:44.000
Gabor Szabo: So, thank you very much, and you can stay around, and we can have an open mic session after I stop the video recording. So, thank you, and bye-bye.



