---
title: Creating a hexadecimal editor in Rust
timestamp: 2026-11-01T14:30:01
author:
published: true
show_related: true
description:
tags:
---


## Description

In this talk, I'll tell a story involving the need of binary editing, how I tried to improve existing tools, and the outcome and challenges of my decision to build my own hex editor in Rust while learning the language. And no, I didn't ask a LLM to write it, for a variety of reasons I'll also cover in the presentation.

Next, I'll present the features my hex editor has and how it helps me (and hopefully others) to achieve my goals while working with security research on a daily basis.

## Bio

[Fernando Mercês](https://github.com/merces/) is a security researcher that spent more than 10 years analyzing malware and investigating cyberattacks and APTs. At [Binarly](https://binarly.io), he works with vulnerability detection in POSIX binaries as his primary focus, and broader ELF ecosystem support on the side. In his free time, he's a volunteer at [Mente Binária](https://mentebinaria.org.br/), the NPO that he founded with free educational programs in computing and security.

## Length

50 min

## Notes

* [x86 Software Reverse-Engineering, Cracking, and Counter-Measures](https://www.amazon.com/x86-Software-Reverse-Engineering-Cracking-Counter-Measures/dp/1394199880/)



{% youtube id="kqmQ5UXVsDM" file="2026-10-29-creating-a-hexadecimal-editor-in-rust-by-fernando-merces.mp4" %}


## Transcript

1
00:00:02.620 --> 00:00:19.800
Gabor Szabo: Hello, and welcome to the Code Maven session. If you are watching this video, then that's the Code Maven channel. If you are here at the presentation, then you probably came for a Rust event. So we also call it Rust Maven as a subsection of the Code Maven.

2
00:00:19.810 --> 00:00:31.939
Gabor Szabo: Fernando, I'm really happy that you accepted the invitation and agreed to give this presentation, and I'm really happy for all the other people who joined us. There are a few people here.

3
00:00:32.130 --> 00:00:40.669
Gabor Szabo: As I mentioned earlier, in our mingling session, you are welcome to ask questions in the… in the…

4
00:00:41.190 --> 00:00:45.340
Gabor Szabo: In the chat, and hopefully you will get an answer, or maybe not.

5
00:00:46.030 --> 00:01:03.540
Gabor Szabo: And if you're watching this on YouTube, then please like the video, and follow the channel, and remember that below the video you will find a link to the future events, and also to notes about this… notes and links about this very presentation.

6
00:01:03.640 --> 00:01:13.249
Gabor Szabo: And I think that's what I needed to say, and now it's your turn, Fernando. Again, thank you very much, and the stage is yours.

7
00:01:15.470 --> 00:01:18.930
Fernando Mercês: Oh, thanks very much. So, yeah, thanks for having me.

8
00:01:20.860 --> 00:01:23.309
Fernando Mercês: I am… currently, I have a…

9
00:01:26.510 --> 00:01:33.360
Fernando Mercês: Weird. But, yeah, if you don't understand anything, something, please,

10
00:01:33.480 --> 00:01:36.200
Fernando Mercês: Ask a question in the chat, I'll be monitoring that.

11
00:01:36.460 --> 00:01:41.700
Fernando Mercês: I will share, now, a terminal window…

12
00:01:42.130 --> 00:01:45.249
Fernando Mercês: So we can start the presentation.

13
00:01:46.840 --> 00:01:50.310
Fernando Mercês: You probably can see these lights.

14
00:01:50.520 --> 00:01:52.559
Gabor Szabo: Yeah, I'm see… I can see them.

15
00:01:53.280 --> 00:01:54.010
Fernando Mercês: Cool.

16
00:01:54.410 --> 00:02:00.320
Fernando Mercês: All right, yeah, so let's… let's start, as we talked a little bit earlier.

17
00:02:01.900 --> 00:02:09.389
Fernando Mercês: Well, it's nice to be here, and talk to everyone in a very different group.

18
00:02:09.880 --> 00:02:14.440
Fernando Mercês: And with different background, which is kind of… which is very nice.

19
00:02:15.580 --> 00:02:25.350
Fernando Mercês: So yeah, creating a hexadecimal editor in REST. This is about my experience. It is, of course, not the only way of doing this.

20
00:02:25.450 --> 00:02:39.219
Fernando Mercês: But it's basically a sharing of a journey that I decided to start, and then I'm going to talk about the reasons, the challenges, and the current state of this project.

21
00:02:39.890 --> 00:02:44.749
Fernando Mercês: So, first, for those who are not familiar with the topic.

22
00:02:44.940 --> 00:02:53.459
Fernando Mercês: And a hex editor is something very, very basic, actually. You can use that for a variety.

23
00:02:53.770 --> 00:02:55.790
Fernando Mercês: Of, of goals.

24
00:02:56.130 --> 00:03:03.310
Fernando Mercês: But it's a very well-known tool on the field of reverse engine… software reverse engineering, and maybe hardware reverse…

32
00:04:03.020 --> 00:04:04.020
Fernando Mercês: Okay.

33
00:04:04.540 --> 00:04:07.659
Fernando Mercês: Yeah, so I stopped at this slide, I was saying that,

34
00:04:07.850 --> 00:04:25.860
Fernando Mercês: a hacks editor is something that you can use for a variety of goals. It's a well-known tool on the software reverse engineering field, hacking in general, security, but also programming, especially, like, if I can say that, like, old-school programming.

35
00:04:26.450 --> 00:04:44.250
Fernando Mercês: And it's very basic, right? Provides you with a view of the bytes, of the content of the file, or even a disk or a partition. There are hex editors out there that can open partitions or disks or other types of devices as well, and see… and give you,

36
00:04:44.470 --> 00:04:54.610
Fernando Mercês: a way to look at… at the data, which is there, in… in… if I can say that, in raw mood. So you see the data, and that's it.

37
00:04:55.120 --> 00:05:04.549
Fernando Mercês: And then, of course, having them… having this data being shown in, in hex… hexadecimal is… is, useful.

38
00:05:05.050 --> 00:05:22.450
Fernando Mercês: But you could… you could use any other numeric-based, numeric-based as well. So yeah, for file format analysis, this is something that I use a lot, like, but you can also use on reverse engineering itself, so, to make, for example, small patches to software.

39
00:05:22.680 --> 00:05:24.630
Fernando Mercês: To either proprietary or…

40
00:05:24.810 --> 00:05:32.760
Fernando Mercês: free software, as long as it's compiled, and then you need to do something quick, or validate what kind of data.

41
00:05:33.370 --> 00:05:36.830
Fernando Mercês: Ended up in a compiled binary, for example.

42
00:05:37.610 --> 00:05:42.900
Fernando Mercês: For game hacks, This is a big thing. There are other,

43
00:05:43.200 --> 00:06:02.160
Fernando Mercês: There are more advanced software for those kind of things as well, like debuggers, disassemblers, other types of software, full frameworks, but the hex editor is usually the first one that you use, because you need something quick, and something that would not interpret any kind…

44
00:06:02.300 --> 00:06:10.070
Fernando Mercês: That will not do any kind of interpretation over the data. And then you can see the data, how it is.

45
00:06:11.950 --> 00:06:16.420
Fernando Mercês: Alright, but then, of course, there are…

46
00:06:16.950 --> 00:06:21.460
Fernando Mercês: probably hundreds, maybe thousands of hex editors out there.

47
00:06:22.100 --> 00:06:26.729
Fernando Mercês: Here, I have a list of some… hex editors, I…

48
00:06:26.940 --> 00:06:32.730
Fernando Mercês: use, and I was using them, like, way more a few years ago.

49
00:06:33.300 --> 00:06:41.130
Fernando Mercês: And it's a totally, opinionated list of, regarding the speed and features. This is something that makes sense

50
00:06:41.380 --> 00:06:48.109
Fernando Mercês: Probably only for me, the way I assess all this software. They are all great, in general.

51
00:06:48.750 --> 00:06:49.910
Fernando Mercês: But…

52
00:06:50.150 --> 00:07:00.390
Fernando Mercês: I… I needed something different, right? Because, you know, the… for example, if you… the second one there on the list, Hill, is… is the best, in my opinion.

53
00:07:01.110 --> 00:07:03.640
Fernando Mercês: But it's commercial, and it's Windows only.

54
00:07:03.870 --> 00:07:15.149
Fernando Mercês: Then we have another one, let's say the one, the first one, 010. It's nice, it's full of features, but it's also commercial, so you have to pay for it.

55
00:07:15.320 --> 00:07:21.849
Fernando Mercês: And the speed is… it's not as fast as heel, for example. So, I was trying to compare all these hacks.

56
00:07:21.850 --> 00:07:30.130
Gabor Szabo: Fernando, sorry to bother you, but is there something maybe on the screen? There seems to be a black rectangle in the top.

57
00:07:30.770 --> 00:07:31.520
Fernando Mercês: Hmm.

58
00:07:31.520 --> 00:07:36.569
Gabor Szabo: Yeah, okay, now we can see the full… I can see the full slide, probably…

59
00:07:37.300 --> 00:07:38.220
Fernando Mercês: Okay, something else.

60
00:07:38.220 --> 00:07:39.299
Gabor Szabo: How are you dicey.

61
00:07:39.600 --> 00:07:40.200
Gabor Szabo: Thanks.

62
00:07:40.200 --> 00:07:43.170
Fernando Mercês: Oh, alright, okay, cool, thanks for telling me.

63
00:07:44.130 --> 00:07:48.920
Fernando Mercês: Yeah, so that's a comparison that I did myself, you know, it's not a…

64
00:07:49.080 --> 00:07:52.409
Fernando Mercês: Again, it's based on my opinion.

65
00:07:52.660 --> 00:08:08.290
Fernando Mercês: Especially the column about speed and features, you know, the features I need, so I gave it stars, depending on the features I need, and also on speed, depending on the speed that I require, because I truly believe that, you know, software,

66
00:08:08.890 --> 00:08:10.290
Fernando Mercês: It has to be fast.

67
00:08:10.410 --> 00:08:13.489
Fernando Mercês: If it can be fast, it should, because…

68
00:08:13.640 --> 00:08:22.890
Fernando Mercês: We are running, like, CPUs with, we have CPUs with multiple cores, lots of gigabytes of run, and all this stuff.

69
00:08:23.140 --> 00:08:32.540
Fernando Mercês: And if you go back to the old days, let's say in the 90s, we had way less processing power, memory.

70
00:08:32.760 --> 00:08:37.680
Fernando Mercês: RAM, and all these, less powerful hardware.

71
00:08:37.850 --> 00:08:43.330
Fernando Mercês: But we had software that was, you know, opening and being operated quickly.

72
00:08:43.570 --> 00:08:50.730
Fernando Mercês: So… Yeah, so this is something that I give value to software that runs fast.

73
00:08:51.250 --> 00:08:59.439
Fernando Mercês: Yeah, so then I needed something different from all of them. You know, they… again, they are all great projects.

74
00:08:59.750 --> 00:09:06.270
Fernando Mercês: But I wanted to have something that I could, use Within the terminal.

75
00:09:06.700 --> 00:09:15.630
Fernando Mercês: So, with a text user interface, for example, And… I wanted to… Not pay for it.

76
00:09:16.200 --> 00:09:21.679
Fernando Mercês: And I want it to be extensible, so I needed a couple features that were not

77
00:09:22.000 --> 00:09:26.829
Fernando Mercês: Covered by a single of any of those projects.

78
00:09:27.750 --> 00:09:35.679
Fernando Mercês: So, there are other hexagitors out there. This is, of course, not, final lease, there are…

79
00:09:35.830 --> 00:09:41.809
Fernando Mercês: way more projects than this. Some of them were abandoned, they were really great, but they're all,

80
00:09:42.120 --> 00:09:44.219
Fernando Mercês: They're not being updated anymore.

81
00:09:44.450 --> 00:09:50.639
Fernando Mercês: Some are a bit, exotic, and you don't find, like, many users,

82
00:09:50.920 --> 00:10:00.919
Fernando Mercês: or many publications about them. I've tested them all when I… before starting this, because I was really looking for some hex heditors that would actually

83
00:10:01.040 --> 00:10:03.659
Fernando Mercês: Meet my requirements.

84
00:10:04.710 --> 00:10:14.630
Fernando Mercês: Some are actually frameworks. They have, like, hexagitors embedded, so as part of, A bigger set of features.

85
00:10:15.350 --> 00:10:26.589
Fernando Mercês: They also work really great. Radar is one example. Commercial software like IDE and Binary Ninja, they also have hexagitors as features.

86
00:10:26.800 --> 00:10:31.009
Fernando Mercês: But again, None of them actually,

87
00:10:31.330 --> 00:10:38.019
Fernando Mercês: I wanted something very specific, and you'll see in the next slides why.

88
00:10:39.080 --> 00:10:48.319
Fernando Mercês: Yeah, so one thing that I also did was I tried to compare all the hexagitors, you know, those awesome lists that,

89
00:10:49.220 --> 00:10:50.959
Fernando Mercês: that you find on GitHub.

90
00:10:51.880 --> 00:11:07.110
Fernando Mercês: And I created one for hex editors as well, just to compare them. So, the idea would be to help people when they want to decide which hex editor to use. And you can go over this list, try all of them, try some of them.

91
00:11:07.440 --> 00:11:12.279
Fernando Mercês: Pick the one you like the most, and then start using this.

92
00:11:13.800 --> 00:11:22.260
Fernando Mercês: Alright, so, if I needed those features, and I have my own, let's say, requirements for speed,

93
00:11:22.370 --> 00:11:35.750
Fernando Mercês: And for, being a multi-platform hexagitor, because I wanted to use that in different operating systems as well. My laptop is from one brand, my PC is from another brand, so I wanted to have this,

94
00:11:36.490 --> 00:11:47.949
Fernando Mercês: good platform, and I wanted to do this from the terminal. So, what could I do? First thing, I tried to improve the existing tools.

95
00:11:48.300 --> 00:11:53.790
Fernando Mercês: The first one was Hugh, again, it's, it's commercial.

96
00:11:53.950 --> 00:11:58.170
Fernando Mercês: But it has a nice plugin interface that you can write plugins for.

97
00:11:58.540 --> 00:12:03.830
Fernando Mercês: So, whenever I needed a feature, I would try to write a plugin.

98
00:12:03.960 --> 00:12:11.150
Fernando Mercês: using its API, its own proprietary way of SDK, and…

99
00:12:11.540 --> 00:12:14.080
Fernando Mercês: And then have the feature implemented.

100
00:12:15.050 --> 00:12:19.860
Fernando Mercês: But eventually, because it's Windows only, and it's also commercial.

101
00:12:19.890 --> 00:12:36.899
Fernando Mercês: I was like, okay, maybe that's not the best approach. So these plugins, I only wrote two, I had more to come, but I decided to move on and try to find something else that would meet more the requirements I needed.

102
00:12:37.340 --> 00:12:42.950
Fernando Mercês: So I found this nice one called Hexpatch, it's open source.

103
00:12:42.950 --> 00:12:46.349
Gabor Szabo: We are seeing the second slide, is that right?

104
00:12:47.600 --> 00:12:50.499
Fernando Mercês: Should be that one. Try to improve? Yeah.

105
00:12:50.500 --> 00:12:52.699
Gabor Szabo: Cyber improve, now we see that one, okay.

106
00:12:53.350 --> 00:12:56.430
Fernando Mercês: Okay, I will try to, you know.

107
00:12:56.880 --> 00:12:59.920
Fernando Mercês: yeah, fix this. I don't know how, but…

108
00:13:00.140 --> 00:13:02.629
Fernando Mercês: Yeah, maybe I can ask you when you…

109
00:13:02.790 --> 00:13:04.820
Fernando Mercês: when I change slides, if you see the next.

110
00:13:04.820 --> 00:13:09.189
Gabor Szabo: Okay, yeah, yeah, good, good idea, because we saw the slide number 2.

111
00:13:09.910 --> 00:13:14.289
Fernando Mercês: But if that makes sense?

112
00:13:14.290 --> 00:13:16.860
Gabor Szabo: I think that you number the slides.

113
00:13:16.860 --> 00:13:19.810
Fernando Mercês: Yeah, yeah, that's, slide number 6. All right.

114
00:13:19.810 --> 00:13:20.840
Gabor Szabo: Yes,

115
00:13:21.880 --> 00:13:23.360
Fernando Mercês: Yeah, so,

116
00:13:24.010 --> 00:13:31.680
Fernando Mercês: This, just, just to make sure that we saw everything. So, the third slide, I showed this, this table, comparison table.

117
00:13:31.930 --> 00:13:33.810
Fernando Mercês: Then I talked about a new affair.

118
00:13:33.810 --> 00:13:35.049
Gabor Szabo: We sold that, yeah.

119
00:13:35.270 --> 00:13:36.350
Fernando Mercês: Abandoned.

120
00:13:36.500 --> 00:13:39.890
Fernando Mercês: What to do, and then here, slide 6.

121
00:13:40.640 --> 00:13:41.750
Gabor Szabo: Yes, okay.

122
00:13:41.920 --> 00:13:42.690
Gabor Szabo: Thanks.

123
00:13:43.490 --> 00:13:49.129
Fernando Mercês: Cool, thank you. So, yeah, so I tried to create these plugins for HEO, and they worked.

124
00:13:49.310 --> 00:14:00.760
Fernando Mercês: It's okay, they work fine. But then, again, they are… here is Windows only, and it's commercial, so I decided to find something else, and found this hex patch.

125
00:14:00.770 --> 00:14:09.920
Fernando Mercês: Which is nice as well. It… it has a text user interface, which I needed. It's multi-platform, it's open source.

126
00:14:10.050 --> 00:14:14.509
Fernando Mercês: I really thought that I had found exactly what I needed.

127
00:14:15.160 --> 00:14:20.880
Fernando Mercês: But I had some requests. I had some, features I wanted to…

128
00:14:21.090 --> 00:14:25.070
Fernando Mercês: I wanted it to have. And indeed, I've opened these issues.

129
00:14:25.180 --> 00:14:34.269
Fernando Mercês: And… Well… They were well received, but of course, it's an open source project, so people probably,

130
00:14:34.380 --> 00:14:51.280
Fernando Mercês: spend, like, their free time doing this, so they cannot… they could not implement the features I wanted, at the time I wanted, you know, for free, of course. So, they… they kindly asked me to implement the features myself. But the problem is that this editor

131
00:14:51.540 --> 00:14:57.400
Fernando Mercês: is written in Rust. And to me, at the time this happened, Rust was…

132
00:14:57.610 --> 00:15:00.580
Fernando Mercês: you know, I didn't know anything about Rust.

133
00:15:00.710 --> 00:15:07.630
Fernando Mercês: I barely knew the name of the language, so I could not implement this myself, the features I wanted.

134
00:15:07.740 --> 00:15:10.970
Fernando Mercês: And I, again, kind of hit a wall.

135
00:15:12.200 --> 00:15:17.100
Fernando Mercês: So, what I did, at the same time, I…

136
00:15:17.490 --> 00:15:22.430
Fernando Mercês: I'm in slide 7 now. At the same time, I…

137
00:15:23.240 --> 00:15:31.409
Fernando Mercês: I joined a new company, right, called Binaryly, and I was told before joining the company that,

138
00:15:31.510 --> 00:15:36.610
Fernando Mercês: The whole ecosystem was, built using Rust.

139
00:15:37.150 --> 00:15:50.100
Fernando Mercês: And then I said, well, great, I know nothing about this language, and I will probably have to use the tools, and maybe eventually send a patch, at least understand the basics of the codes.

140
00:15:50.720 --> 00:15:58.700
Fernando Mercês: So, I… my background is in C programming, but I'm not, I'm a security researcher, so I'm not,

141
00:15:59.300 --> 00:16:13.820
Fernando Mercês: professional developer, if I can say that. So, the software I would write is, like, a little 100 lines to 200 lines, and I did… I never had to follow, like, best practices in programming.

142
00:16:13.920 --> 00:16:29.939
Fernando Mercês: If it worked on my machine, that was good enough for me, you know? And I know that's not the reality of most developers out there, they have to deal with very complex stuff, and different environments, and all this stuff. It was not my case. So, that's all to say that I was not a good programmer.

143
00:16:30.300 --> 00:16:32.470
Fernando Mercês: I was not an experienced programmer.

144
00:16:32.570 --> 00:16:45.509
Fernando Mercês: And… but I had to. I had to understand Rust, I had to write it, I had to learn it. So, I found myself in this, situation that I needed to learn a new language, which is, in that case, is Rust.

145
00:16:45.890 --> 00:16:51.999
Fernando Mercês: And I wanted to have my own, to have a hex editor the way I want it.

146
00:16:52.510 --> 00:16:55.979
Fernando Mercês: So what I did, I started planning.

147
00:16:56.120 --> 00:17:08.900
Fernando Mercês: I tried to find, you know, libraries, maybe a different language could also be something like this, but then I gave up… I quickly gave up about the language and chose Rust, because I needed that for work.

148
00:17:09.470 --> 00:17:16.339
Fernando Mercês: And I needed to… to create this software, this hex heditor, that would be fast.

149
00:17:16.740 --> 00:17:29.760
Fernando Mercês: It should be cross-platform, it should be a… should have a text user interface, so I could use it within the terminal. It could also… it should also be fast to operate. So, basically using,

150
00:17:29.940 --> 00:17:48.109
Fernando Mercês: keyboard shortcuts, because then it's faster. If you're already typing on the terminal, and so you don't have to use your hands, move your hand until you find the mouse, and then go back to the terminal, to the keyboard. So I wanted to operate it from the keyboard, exclusively.

151
00:17:49.540 --> 00:17:55.169
Fernando Mercês: I took a hill as an inspiration, because, again, I find it, it's very nice.

152
00:17:55.400 --> 00:18:00.269
Fernando Mercês: But it's Windows only and commercial, but it's still… it's full of features, and

153
00:18:00.440 --> 00:18:03.649
Fernando Mercês: It's very fast, and it's an impressive piece of work.

154
00:18:04.300 --> 00:18:08.330
Fernando Mercês: And I chose, Rust, right? So,

155
00:18:09.420 --> 00:18:16.740
Fernando Mercês: Let me see the challenges, yeah. I chose, Rust plus Ratatouille, which is,

156
00:18:17.580 --> 00:18:26.309
Fernando Mercês: Text user interface, library. That's what Hexpatch uses. I looked at Hexpatch source codes.

157
00:18:26.410 --> 00:18:31.720
Fernando Mercês: And see what they were using, and decided to use the same thing.

158
00:18:33.900 --> 00:18:45.810
Fernando Mercês: And, yes, I wanted to learn and exercise my brain as well, especially in the era of AI. I know I could just have asked the AI to create that for me.

159
00:18:46.060 --> 00:18:59.850
Fernando Mercês: But then I felt like, well, I have to understand this myself, and it will also be a good exercise for my brain, because I heard that Russ has a different paradigm than

160
00:18:59.960 --> 00:19:11.639
Fernando Mercês: previous or other existing programming languages. And I was like, well, cool, let me see if I can understand that. I'm still trying, by the way. It's not the easiest thing in the world.

161
00:19:11.970 --> 00:19:21.860
Fernando Mercês: But it's cool, and I had some success while learning this. But then comes the challenges. The first one is the Rust language itself.

162
00:19:22.310 --> 00:19:29.170
Fernando Mercês: I found that, official book, like, quite big, to be honest.

163
00:19:29.710 --> 00:19:42.529
Fernando Mercês: the ecosystem is huge, especially compared to C. In C, for example, we can probably enumerate all the keywords. You know, it's very, very simple language.

164
00:19:42.670 --> 00:19:47.019
Fernando Mercês: But in Rust, things are different, and there are many ways of doing stuff, so…

165
00:19:47.230 --> 00:19:49.360
Fernando Mercês: Yeah, it was challenging for me.

166
00:19:49.770 --> 00:19:58.919
Fernando Mercês: The other thing is the… the graphics programming, so the idea of having a main loop, like in a game, or…

167
00:19:59.130 --> 00:20:13.639
Fernando Mercês: rendering, I of course, struggled in the beginning, because I was changing something, and then the render loop would run again, would iterate again, and I would lose my changes and all this stuff, or get… or being blocked.

168
00:20:13.790 --> 00:20:25.549
Fernando Mercês: And I stopped rendering the next frames because my if or my while loop had blocked something. So, yeah, that to me was, was also challenging.

169
00:20:26.380 --> 00:20:32.239
Fernando Mercês: But I started that. I started that on, on July 2025.

170
00:20:33.220 --> 00:20:41.830
Fernando Mercês: And… I'm going to show you here the concepts and the features that I wanted to have.

171
00:20:41.830 --> 00:20:45.219
Gabor Szabo: The lines are back, sorry. The black box is back.

172
00:20:47.310 --> 00:20:48.330
Fernando Mercês: Okay.

173
00:20:50.370 --> 00:20:53.279
Gabor Szabo: What is that? Is that some, part of,

174
00:20:53.740 --> 00:20:58.689
Fernando Mercês: That's the term, you know, I mean, in my… I see… I see the black… okay, it's rendered now, right?

175
00:20:58.890 --> 00:21:01.110
Gabor Szabo: Yeah, now I can see this slide.

176
00:21:01.910 --> 00:21:03.030
Fernando Mercês: Yeah, it's a very…

177
00:21:03.030 --> 00:21:03.820
Gabor Szabo: Number 9.

178
00:21:04.940 --> 00:21:05.830
Fernando Mercês: Okay.

179
00:21:07.110 --> 00:21:10.240
Gabor Szabo: Maybe Zoom has problems with the sharing, yeah.

180
00:21:11.390 --> 00:21:18.520
Fernando Mercês: Yeah, okay, but now I can see also the screen you guys see, so if I see the black boxes there, I'll try to fix that.

181
00:21:19.860 --> 00:21:20.650
Gabor Szabo: Okay.

182
00:21:21.440 --> 00:21:28.340
Fernando Mercês: So these are the concepts of the editor I created. I will not go, like, step…

183
00:21:28.490 --> 00:21:35.360
Fernando Mercês: Like, one by one, go through them one by one, because… They…

184
00:21:35.490 --> 00:21:42.560
Fernando Mercês: I've been working on this for more than one year now. It's, of course, it's a personal project.

185
00:21:42.830 --> 00:21:48.889
Fernando Mercês: that I also use for work, sometimes, because it's a hexagator in the end of the day, so I can use that for…

186
00:21:50.790 --> 00:21:53.550
Fernando Mercês: anything that I want. But,

187
00:21:53.870 --> 00:21:58.669
Fernando Mercês: But I only spent, like, a few hours, I don't know, a month on this.

188
00:21:59.250 --> 00:22:06.529
Fernando Mercês: Yeah, and I did this, everything from scratch. I see the black rectangle again, okay.

189
00:22:07.740 --> 00:22:12.190
Fernando Mercês: So if you look at the codes, you will see,

190
00:22:12.370 --> 00:22:16.149
Fernando Mercês: That's a code from, you know, a beginner in Rust.

191
00:22:16.320 --> 00:22:20.799
Fernando Mercês: There are lots of things that have been changing over the… over time.

192
00:22:21.280 --> 00:22:22.470
Fernando Mercês: And…

193
00:22:23.140 --> 00:22:36.399
Fernando Mercês: whenever I discover, you know, a more… a smarter way of doing something, I go there, and then… it's my own playground as well. Especially because, as I told you, I needed to learn that for… to learn Rust.

194
00:22:36.800 --> 00:22:40.009
Fernando Mercês: In order to work at the company that I work.

195
00:22:40.660 --> 00:22:44.120
Fernando Mercês: And I wanted to have my own playground so I would not break.

196
00:22:44.250 --> 00:22:49.310
Fernando Mercês: the company's software. I could do anything on my own software.

197
00:22:49.600 --> 00:22:57.579
Fernando Mercês: Alright, so let me… this is… these are the concepts and the features, but I'm going to show you, like, a demo.

198
00:22:58.190 --> 00:23:06.120
Fernando Mercês: So we can, understand more, and then I go back to these concepts, and we'll be…

199
00:23:06.320 --> 00:23:09.220
Fernando Mercês: They will be more easily, understood.

200
00:23:10.190 --> 00:23:16.400
Fernando Mercês: So first, I have here… Yeah.

201
00:23:16.740 --> 00:23:18.059
Fernando Mercês: I have a game.

202
00:23:19.710 --> 00:23:28.059
Fernando Mercês: it's a gas game, you know. The same one… well, not the same, but similar to what we have in the official Rust book, like, you have to guess the number.

203
00:23:29.230 --> 00:23:37.350
Fernando Mercês: And I'm using Z6 here to open the binary that was generated by the previous, compilation.

204
00:23:38.050 --> 00:23:40.839
Fernando Mercês: So that's the main window of the…

205
00:23:41.810 --> 00:23:45.689
Fernando Mercês: of Z6, of the hex editor I've been talking about.

206
00:23:46.310 --> 00:23:55.170
Fernando Mercês: This version is… That's version… Yeah, 090, this is,

207
00:23:55.290 --> 00:23:58.810
Fernando Mercês: the one on GitHub, but not released yet.

208
00:23:59.410 --> 00:24:17.939
Fernando Mercês: But anyway, I tried to copy… I mean, I call that VIN-inspired, but actually, I kind of copied the way VIN does things. It's inspired in multiple different projects. VIN is one of them, it's heavily inspired by VIN.

209
00:24:18.300 --> 00:24:23.059
Fernando Mercês: So you can see the bar over here is the bar that VIN has.

210
00:24:23.950 --> 00:24:31.250
Fernando Mercês: the… the current, VIN call it, mode, right? The mode that,

211
00:24:31.590 --> 00:24:33.719
Fernando Mercês: The app is, it's normal.

212
00:24:33.990 --> 00:24:36.190
Fernando Mercês: Then I st- I also…

213
00:24:36.440 --> 00:24:40.100
Fernando Mercês: added some other stuff from other Hexagittals, for example, heel.

214
00:24:40.280 --> 00:24:47.820
Fernando Mercês: shows if you are editing an ELF binary or a PE binary, a Windows executable.

215
00:24:48.150 --> 00:24:56.959
Fernando Mercês: Straight away in your first screen, so I added this here, and here is the offset, like, the position of the current cursor.

216
00:24:57.220 --> 00:25:00.870
Fernando Mercês: And the percentage of the file that you are.

217
00:25:02.100 --> 00:25:11.339
Fernando Mercês: So, hopefully, yeah, the black boxes. I know that they keep appearing and going away. Sorry, that's not my… that's not part of ZZ6, right?

218
00:25:13.450 --> 00:25:22.649
Fernando Mercês: Yeah, and then you can do pretty much, you can navigate here just like you do with VIM. So, use the H, J,

219
00:25:22.790 --> 00:25:33.020
Fernando Mercês: K, L, also zero or dollar sign to go to the end. I had to adapt from VIN. Like, for example, in VIN, if you press the dollar sign.

220
00:25:34.850 --> 00:25:41.299
Fernando Mercês: it will go to the end of the line. Here, we don't have the concept of lines, because it's a stream of data.

221
00:25:41.480 --> 00:25:45.390
Fernando Mercês: It's very common to… to show

222
00:25:45.650 --> 00:26:04.959
Fernando Mercês: 16 bytes per line, but you can change that. So it goes to the end of the line, although this concept is actually more virtual in a binary file. But yeah, if you press home, or AND, or zero, or Dollar, and of course, all these comments are documented in the reading file.

223
00:26:05.560 --> 00:26:10.049
Fernando Mercês: Then you… you are navigating through Through the file.

224
00:26:10.230 --> 00:26:11.010
Fernando Mercês: Right.

225
00:26:11.150 --> 00:26:29.629
Fernando Mercês: And as you would expect… so, why did I choose VIN to inspire the keys? Because otherwise, I would have to come up with a totally new set of keys. So, whoever would be downloading ZCC and try to use that would have to check the help file.

226
00:26:29.970 --> 00:26:49.309
Fernando Mercês: see what the key shortcuts are. You still have to do this for some very specific stuff, because, you know, it's not VIN, so it's different. But, at least the basic navigation, or… I don't know, if you press a slash key, you would expect to have a search feature. It's the same.

227
00:26:49.410 --> 00:26:54.459
Fernando Mercês: So you can search here, like, Ginu, for example, searching for a string.

228
00:26:54.780 --> 00:26:58.109
Fernando Mercês: And pressing enter, so it finds the first one.

229
00:26:59.340 --> 00:27:00.840
Fernando Mercês: Right?

230
00:27:01.330 --> 00:27:07.970
Fernando Mercês: And then you can press N to keep finding, like, the next match, or Shift-N to go back.

231
00:27:08.170 --> 00:27:13.919
Fernando Mercês: the previous match, all… I mean, not all, but, the most common

232
00:27:14.110 --> 00:27:18.480
Fernando Mercês: stuff for, that you would do in Veeam also work.

233
00:27:18.790 --> 00:27:21.300
Fernando Mercês: Also works here.

234
00:27:22.650 --> 00:27:24.210
Fernando Mercês: Alright, so…

235
00:27:25.360 --> 00:27:32.059
Fernando Mercês: To show you some of the features, I'll go back to the pressing G to go to the beginning of the file.

236
00:27:32.510 --> 00:27:36.330
Fernando Mercês: And… Let me quit.

237
00:27:37.210 --> 00:27:44.240
Fernando Mercês: the… the program, so we can… just like VIN, like, column, and then… Q for quits.

238
00:27:44.700 --> 00:27:46.110
Fernando Mercês: And we are out.

239
00:27:46.270 --> 00:27:53.440
Fernando Mercês: We'll do some editing of this file, because it's an X… it's a hex editor, so…

240
00:27:55.270 --> 00:28:04.969
Fernando Mercês: This file here, this program here, I've… of course, I created that myself, like, just to simulate something, some obstacles that we would have to…

241
00:28:05.450 --> 00:28:10.190
Fernando Mercês: To, overcome, if you want to run it, or if you want to do whatever.

242
00:28:10.350 --> 00:28:17.039
Fernando Mercês: we want to, with this. So, in this case, my, system is, is, Ubuntu.

243
00:28:17.240 --> 00:28:21.849
Fernando Mercês: And I've emulated something that I've… that I…

244
00:28:22.270 --> 00:28:25.999
Fernando Mercês: I've experienced myself, like, many years ago, when I downloaded as

245
00:28:26.180 --> 00:28:28.419
Fernando Mercês: I think it was a Mortal Kombat game.

246
00:28:28.590 --> 00:28:32.439
Fernando Mercês: for Windows Vista, and

247
00:28:32.760 --> 00:28:38.100
Fernando Mercês: And the game would not run, because it said, well, we can… we don't… we do not support Windows Vista.

248
00:28:38.340 --> 00:28:40.410
Fernando Mercês: Because the game was released before.

249
00:28:40.620 --> 00:28:47.670
Fernando Mercês: And, actually, the error message would say something like, we don't support this version of Windows, it was generic.

250
00:28:48.030 --> 00:28:53.249
Fernando Mercês: And I remember I used one hex editor, I don't remember which one.

251
00:28:54.030 --> 00:29:03.120
Fernando Mercês: to kind of bypass this, to make the program to ignore that, just to see if that game would work on Windows Vista, and it did. So…

252
00:29:03.430 --> 00:29:11.730
Fernando Mercês: This is one thing that you could do, for example. Of course, this is… this is already applying the concept of using a hex editor to reverse engineering.

253
00:29:12.010 --> 00:29:16.360
Fernando Mercês: But, yeah, it's… It's a SU Basic.

254
00:29:16.970 --> 00:29:27.500
Fernando Mercês: So yeah, one thing that we could do here, for example, is look for that string. We could use the search feature that I just showed you, by using the… pressing the slash key.

255
00:29:28.030 --> 00:29:33.629
Fernando Mercês: But we can also, pressing S, for example, this is something that VIN doesn't,

256
00:29:33.920 --> 00:29:45.390
Fernando Mercês: doesn't have, because it doesn't need it, actually, right? Everything, in theory, when you are editing using Veeam, mostly, almost everything is text anyway. But here,

257
00:29:45.890 --> 00:29:46.880
Fernando Mercês: You can…

258
00:29:47.060 --> 00:29:57.969
Fernando Mercês: navigate through a list of strings that ZCC finds for you. This is also… this was inspired in, Hill. Heel does this, which is very nice.

259
00:29:58.110 --> 00:30:05.740
Fernando Mercês: And, you can… I actually set the minimum of strings… sorry, the minimum length.

260
00:30:05.930 --> 00:30:07.579
Fernando Mercês: Of the string you wanted.

261
00:30:07.870 --> 00:30:15.220
Fernando Mercês: to filter. You can also press slash within here, and then filter that using a regular expression.

262
00:30:15.490 --> 00:30:18.919
Fernando Mercês: In that case, I think it was Debian, right?

263
00:30:20.490 --> 00:30:34.859
Fernando Mercês: then it would find, in this case, we found our string within the binary here. It's in this position, our file offset, if I can say that, but 20A0. If you press enter, it follows there.

264
00:30:35.100 --> 00:30:46.310
Fernando Mercês: So… We found that string, we found also another one called Debian here. We can, select this, and…

265
00:30:46.450 --> 00:30:47.500
Fernando Mercês: make it…

266
00:30:48.060 --> 00:30:55.930
Fernando Mercês: pick up an interesting caller, so we can highlight that every time that we see it, which is good.

267
00:30:56.450 --> 00:31:00.339
Fernando Mercês: We can also comment, for example, here, if I press…

268
00:31:00.560 --> 00:31:08.699
Fernando Mercês: the same column key, I say, well, this is, I don't know, an OS stream.

269
00:31:09.720 --> 00:31:18.799
Fernando Mercês: and press enter. Now I've added a comment at this offset, so every time I… the cursor is over this, this comment appears.

270
00:31:19.110 --> 00:31:24.909
Fernando Mercês: The idea here is, let's say you are doing something very complex, and you are finding

271
00:31:25.260 --> 00:31:32.740
Fernando Mercês: different stuff that you find interesting, and you have to annotate that. So, you use those comments to annotate your stuff.

272
00:31:33.120 --> 00:31:35.420
Fernando Mercês: When you quit, this is 6.

273
00:31:36.400 --> 00:31:39.229
Fernando Mercês: So yeah, WQ, for example.

274
00:31:41.310 --> 00:31:52.410
Fernando Mercês: It creates, a file that has, what you did, what you saved, so, for example, the blocks.

275
00:31:52.560 --> 00:31:58.719
Fernando Mercês: And also the comments. So the idea is, when you load it again, the same file.

276
00:31:58.970 --> 00:32:04.619
Fernando Mercês: It's going to load this, tone… DOML file,

277
00:32:05.020 --> 00:32:13.389
Fernando Mercês: together. So the things will be there. So Alt-N for names, you see the strings, the comments that you made, and then

278
00:32:13.800 --> 00:32:18.040
Fernando Mercês: You go there, it's, it still has the color that you put, and the…

279
00:32:18.690 --> 00:32:31.679
Fernando Mercês: and the comments that you made. So yeah, so now we are going to change that, because you see, there is another string here, like, etc, which is usually one way of, identifying the…

280
00:32:32.160 --> 00:32:35.139
Fernando Mercês: the operating system, right?

281
00:32:35.970 --> 00:32:42.409
Fernando Mercês: And so, like, I could assume that it's reading this, and maybe comparing with Debian.

282
00:32:42.680 --> 00:32:45.499
Fernando Mercês: And if I look at my…

283
00:32:46.360 --> 00:32:49.100
Fernando Mercês: that is an issue. It says Ubuntu.

284
00:32:49.290 --> 00:32:52.230
Fernando Mercês: So, I'm going to change that here.

285
00:32:52.500 --> 00:32:57.710
Fernando Mercês: So, pressing R, we enter into the replace mode.

286
00:32:58.000 --> 00:33:07.989
Fernando Mercês: I could change the bytes, like 50-50 for a capital U, and all this stuff, but I can also press Tab, and then just type.

287
00:33:09.670 --> 00:33:11.300
Fernando Mercês: Ubuntu here.

288
00:33:12.990 --> 00:33:24.689
Fernando Mercês: And then, now, the buffer is changed, but it's not yet written to file. That's the same concept that VIN uses, right? You changed the buffer, but you didn't write to the file yet.

289
00:33:25.510 --> 00:33:29.690
Fernando Mercês: if I do quit, Write and quit.

290
00:33:30.830 --> 00:33:33.130
Fernando Mercês: Now, in theory.

291
00:33:34.070 --> 00:33:42.180
Fernando Mercês: yeah, now the game, let's say, runs, because now it's checking against Ubuntu and, sorry, against Debian.

292
00:33:42.520 --> 00:33:47.469
Fernando Mercês: And, against Ubuntu, because we changed that, right? And now it's running.

293
00:33:47.800 --> 00:33:51.210
Fernando Mercês: And then, well… That's a regular guess.

294
00:33:51.430 --> 00:33:58.739
Fernando Mercês: the number game, so we can try 50… it says too low, so maybe…

295
00:33:59.630 --> 00:34:05.429
Fernando Mercês: 75, too high, and that's it, so 67…

296
00:34:05.740 --> 00:34:09.569
Fernando Mercês: Too low, game over. So the idea here was, like.

297
00:34:10.130 --> 00:34:15.690
Fernando Mercês: It's an easy game, but it's actually with only 3 tries, or 3 lives.

298
00:34:16.060 --> 00:34:20.750
Fernando Mercês: It's a bit hard to win, actually, right? You have to be very lucky.

299
00:34:20.980 --> 00:34:27.679
Fernando Mercês: So… in order to show, like, what ZCC does,

300
00:34:28.139 --> 00:34:32.059
Fernando Mercês: We will try to hack this, like, to change this number of lives.

301
00:34:32.590 --> 00:34:34.379
Fernando Mercês: So we are looking for…

302
00:34:35.389 --> 00:34:39.700
Fernando Mercês: In this6, in this binary, is the number of,

303
00:34:40.100 --> 00:34:42.839
Gabor Szabo: Sorry, I have a… I have a question here.

304
00:34:42.949 --> 00:34:50.440
Gabor Szabo: Sure. So I think maybe I just missed it, and I have really no idea how these things work.

305
00:34:50.980 --> 00:34:57.969
Gabor Szabo: You changed one set of letters with the exact same amount of letters, right?

306
00:34:58.320 --> 00:34:59.040
Fernando Mercês: Yeah.

307
00:34:59.460 --> 00:35:03.629
Gabor Szabo: What if you need to add more… if the string is longer?

308
00:35:04.500 --> 00:35:05.319
Fernando Mercês: Yeah, yeah.

309
00:35:06.190 --> 00:35:06.840
Gabor Szabo: Or shorter.

310
00:35:07.850 --> 00:35:27.749
Fernando Mercês: Yeah, if this string is longer, well, there are ways of doing this, but we'll have to go deeper into reverse engineering, because we'll have to change the address of that string to a different address. Let's say, usually, what we do in this case, we find where that string is referenced.

311
00:35:28.470 --> 00:35:35.240
Fernando Mercês: Which means… That address, the address here would be…

312
00:35:35.440 --> 00:35:45.779
Fernando Mercês: 2092. So, somehow, the code references this string by address, so it reaches this. And then we change that reference

313
00:35:46.060 --> 00:35:47.699
Fernando Mercês: To a different number.

314
00:35:48.070 --> 00:35:49.710
Fernando Mercês: Let's say a number here.

315
00:35:50.040 --> 00:35:51.300
Fernando Mercês: So in the beginning of this…

316
00:35:51.300 --> 00:36:03.529
Gabor Szabo: The beginning will be the same for that thing, but the end… but the length is not, and every other thing will have changed the address, everything which comes later, right?

317
00:36:04.370 --> 00:36:08.080
Gabor Szabo: I mean, if we add a new, longer string.

318
00:36:08.460 --> 00:36:12.159
Gabor Szabo: The address of that string remains the same.

319
00:36:12.160 --> 00:36:13.580
Fernando Mercês: Yes, yes.

320
00:36:13.580 --> 00:36:22.649
Gabor Szabo: It's just the length is changing, and what also changes is the address of every other thing that comes after this.

321
00:36:24.630 --> 00:36:28.410
Fernando Mercês: Oh, not really, because… not really, because in the…

322
00:36:28.640 --> 00:36:36.039
Fernando Mercês: in the… in the code itself, which I didn't show here, because there is no disassembly capability in Z6 yet.

323
00:36:36.250 --> 00:36:42.430
Fernando Mercês: But the code will reference this string by its address. So, if you change just the address.

324
00:36:42.700 --> 00:36:44.940
Fernando Mercês: that the code references. Let's say…

325
00:36:45.160 --> 00:36:49.090
Fernando Mercês: you change the address to this one, to 1F90.

326
00:36:49.270 --> 00:36:53.319
Fernando Mercês: You can just write your string here, let's say arc Linux.

327
00:36:54.770 --> 00:37:03.589
Fernando Mercês: Right? And then, you'd leave this one as Debian, There's not any… to… to… To touch it.

328
00:37:03.710 --> 00:37:06.700
Fernando Mercês: You just leave this one as is, and then…

329
00:37:06.880 --> 00:37:14.760
Fernando Mercês: At the reference, at the code reference that… that reads from here, you would just patch the code to read from here.

330
00:37:15.080 --> 00:37:19.019
Gabor Szabo: Oh, and you're using some space in the code which is not used?

331
00:37:19.350 --> 00:37:20.730
Fernando Mercês: Exactly, exactly.

332
00:37:20.730 --> 00:37:21.170
Gabor Szabo: Okay.

333
00:37:21.170 --> 00:37:22.670
Fernando Mercês: Yes. Yes.

334
00:37:24.070 --> 00:37:28.760
Gabor Szabo: But is it common that you have, like, these sections of empty…

335
00:37:28.760 --> 00:37:29.620
Fernando Mercês: It is.

336
00:37:29.750 --> 00:37:35.880
Fernando Mercês: It is, yes, because, see lots of zeros, because, in many sections,

337
00:37:36.400 --> 00:37:40.020
Fernando Mercês: It has to be, like, aligned to a certain number of bytes.

338
00:37:40.130 --> 00:37:44.490
Fernando Mercês: So it doesn't matter, you know, how much data you have.

339
00:37:44.770 --> 00:37:53.840
Fernando Mercês: Usually the compiler adds some padding to… to make it properly aligned, so you get new,

340
00:37:54.220 --> 00:37:59.219
Fernando Mercês: Addresses, empty addresses that are not used, and then you can write your stuff.

341
00:37:59.810 --> 00:38:08.880
Gabor Szabo: Interesting. Okay, sorry, it was a totally, I think, sidetrack of how to use hacking, or I don't know what to call it.

342
00:38:09.820 --> 00:38:12.759
Fernando Mercês: Yeah, yeah, yeah, no, it's okay, it's a good question.

343
00:38:13.330 --> 00:38:19.419
Fernando Mercês: Because in that case, Debian and Ubuntu, indeed, they have the same length. Yeah.

344
00:38:20.010 --> 00:38:26.749
Fernando Mercês: Alright, so… Yeah, so the game is, is,

345
00:38:26.900 --> 00:38:30.250
Fernando Mercês: It's difficult, right, with only this, tree,

346
00:38:31.670 --> 00:38:39.540
Fernando Mercês: 3 tries. So, we are looking for this number 3, which the game starts with 3.

347
00:38:39.930 --> 00:38:43.770
Fernando Mercês: giving you by… by giving you three lives. Of course.

348
00:38:43.900 --> 00:38:52.229
Fernando Mercês: This is, again, now it's a number, it's not a string. So we cannot just look for, let's say, 3.

349
00:38:52.630 --> 00:38:53.410
Fernando Mercês: Here.

350
00:38:53.710 --> 00:39:00.880
Fernando Mercês: Because the… the… the text… The character tree is actually 33 in,

351
00:39:01.140 --> 00:39:06.500
Fernando Mercês: in hexadecimal, following the ASCII table, and

352
00:39:07.250 --> 00:39:17.739
Fernando Mercês: In that case, lives is a variable, it's an integer, so it has to be incremented or decremented, actually. So it's not,

353
00:39:17.870 --> 00:39:22.770
Fernando Mercês: A character, so it's literally the byte 03.

354
00:39:23.580 --> 00:39:32.500
Fernando Mercês: In theory, if you are talking about a 32-bit or 64-bit program, That's, that's an INT32.

355
00:39:32.610 --> 00:39:38.460
Fernando Mercês: Or, unsigned int 32, right? 32 bits. So 4 bytes.

356
00:39:38.960 --> 00:39:46.350
Fernando Mercês: We could… And then comes the other feature of, of… of ZCC.

357
00:39:47.020 --> 00:39:55.929
Fernando Mercês: We are here at the search… in the search mode. If you press tab, you go to the search slash hex. Here, you can type the bytes.

358
00:39:56.510 --> 00:40:08.279
Fernando Mercês: So, in that case, we are looking for a 03, right? You can add spaces here if you want, just to make it clearer. And the reason I'm typing, like, a

359
00:40:09.080 --> 00:40:13.280
Fernando Mercês: Reversed is actually because of the indiness of,

360
00:40:13.580 --> 00:40:21.489
Fernando Mercês: of this processor, right? I'm using AMD, and also Intel does this, so it stores the integers,

361
00:40:21.820 --> 00:40:31.750
Fernando Mercês: from the… the first byte is here, then the second here, the third one, and finally the fourth one. So this is 0003.

362
00:40:32.210 --> 00:40:35.579
Fernando Mercês: So this is the sequence I'm looking for. I'm looking for a…

363
00:40:36.290 --> 00:40:43.550
Fernando Mercês: Integer variable within the binary that has the value… it's initialized with the value of 3.

364
00:40:44.550 --> 00:40:46.550
Fernando Mercês: The problem with this approach.

365
00:40:46.710 --> 00:40:56.259
Fernando Mercês: is that you find this stuff everywhere. For example, here I'm still at the very beginning of the file, and I found this sequence, one of them.

366
00:40:56.680 --> 00:41:08.299
Fernando Mercês: But this is very likely part of the header. It's not the code yet. It's not the… it's not data yet. So, we could keep searching, let's say by pressing N,

367
00:41:08.490 --> 00:41:17.080
Fernando Mercês: And we would find a lot, lots of, zero trees. We can actually use AltH to highlight that.

368
00:41:17.900 --> 00:41:26.409
Fernando Mercês: And then all 03s will be highlighted in the quotes, and you see all of them… I mean, it can be any of them.

369
00:41:26.410 --> 00:41:38.779
Fernando Mercês: Basically, we don't know. So it's very weird, I mean, it's a very ineffective approach to try randomly to find a zero tree and try to change that to 4, for example.

370
00:41:39.570 --> 00:41:44.610
Fernando Mercês: But there is a better way of doing this. Let me just disable this highlight with Alt-H.

371
00:41:45.680 --> 00:41:51.890
Fernando Mercês: And that's because of the concepts of, of the views, right?

372
00:41:52.060 --> 00:41:56.040
Fernando Mercês: So, views in ZCC are ways to look at data.

373
00:41:56.340 --> 00:42:01.269
Fernando Mercês: The first one, you know, of course, the one you are going to spend, the rest of your time…

374
00:42:01.270 --> 00:42:02.370
Gabor Szabo: Yeah, okay.

375
00:42:02.680 --> 00:42:04.120
Fernando Mercês: Yeah, I saw that sometime.

376
00:42:06.100 --> 00:42:08.819
Fernando Mercês: is the hex one, the hex view, of course.

377
00:42:08.980 --> 00:42:11.030
Fernando Mercês: But there's also the text view.

378
00:42:11.680 --> 00:42:19.380
Fernando Mercês: If you press Tab, you see the file as a text file, as VIN would show, for example, accepting that the…

379
00:42:21.050 --> 00:42:23.590
Fernando Mercês: The known printable characters.

380
00:42:23.950 --> 00:42:30.719
Fernando Mercês: Are rendered as blank spaces, just to make… make them look nicer.

381
00:42:31.510 --> 00:42:33.609
Gabor Szabo: You're getting to the matrix trail.

382
00:42:35.480 --> 00:42:36.820
Gabor Szabo: You're approaching.

383
00:42:37.640 --> 00:42:40.289
Fernando Mercês: And if we press, tab again.

384
00:42:41.850 --> 00:42:47.180
Fernando Mercês: We are using, another view. I know the black box is there, I'm going… okay.

385
00:42:47.980 --> 00:42:52.680
Fernando Mercês: When I move the window, it goes away, and then it comes back, and then it goes away, alright.

386
00:42:52.860 --> 00:43:00.250
Fernando Mercês: That view here for head for executable files, it actually parses

387
00:43:00.370 --> 00:43:10.049
Fernando Mercês: understands the formats of executable files. For now, just only for P and ELF files, so Windows and Linux, Unix.

388
00:43:10.300 --> 00:43:12.120
Fernando Mercês: Files in general.

389
00:43:12.860 --> 00:43:19.090
Fernando Mercês: And it gives the meaning of the stuff. So, for example, this 2 here.

390
00:43:19.720 --> 00:43:31.649
Fernando Mercês: You know, that class 2 means that this binary is an L64, it's a 64-bit binary. This 2 here is actually this byte number 2.

391
00:43:31.980 --> 00:43:39.490
Fernando Mercês: The only difference from this view, this way of looking at data, is that here, there's no interpretation, there's nothing, you are the one responsible

392
00:43:39.640 --> 00:43:43.560
Fernando Mercês: You know, if we want to understand why this… Number 2 is here.

393
00:43:43.850 --> 00:43:50.590
Fernando Mercês: That's on you. You have to read the ELF file specification And, and…

394
00:43:51.040 --> 00:43:55.909
Fernando Mercês: And then, figure out why this 02… what this 02 means.

395
00:43:56.580 --> 00:44:00.059
Fernando Mercês: But if you use that view.

396
00:44:00.870 --> 00:44:02.719
Fernando Mercês: It gives you the meaning for you.

397
00:44:04.010 --> 00:44:09.870
Fernando Mercês: This is only useful if you are, you know, reversing some stuff, analyzing some compiled program.

398
00:44:10.160 --> 00:44:13.280
Fernando Mercês: It's very specific to reverse engineering tasks.

399
00:44:13.640 --> 00:44:18.800
Fernando Mercês: But, it makes things way easier, because, for example.

400
00:44:19.640 --> 00:44:22.950
Fernando Mercês: If you know a little bit about compilers.

401
00:44:24.060 --> 00:44:28.620
Fernando Mercês: You might guess that, these sections

402
00:44:29.170 --> 00:44:32.070
Fernando Mercês: Are basically different ways of,

403
00:44:32.850 --> 00:44:36.050
Fernando Mercês: It's how the data is spread

404
00:44:36.340 --> 00:44:41.810
Fernando Mercês: over the binary. And, you have sections with, usually codes.

405
00:44:41.940 --> 00:44:59.760
Fernando Mercês: It's, for example, that one, text section, where the code will be, data, got, and then there are specifics, things that are specific to this format, but still, this is how the data is organized within this binary. Not in the Z6, the binary that we are editing.

406
00:45:00.740 --> 00:45:05.110
Fernando Mercês: And in that case, if I go here to section Text, for example.

407
00:45:05.240 --> 00:45:10.110
Fernando Mercês: I see that the offsets, the fire offset, is 11, 80.

408
00:45:10.480 --> 00:45:11.980
Fernando Mercês: If I press enter here.

409
00:45:12.200 --> 00:45:17.789
Fernando Mercês: Then I go there. This is the code. Of course, you still see a lot… only bytes.

410
00:45:18.030 --> 00:45:22.559
Fernando Mercês: Because… ZZ6 does not have an embedded disassembler yet.

411
00:45:22.710 --> 00:45:30.080
Fernando Mercês: But, but this is the code of the program. Now you know that the code starts here. You can even make a comment, like, this is .txt.

412
00:45:30.530 --> 00:45:34.349
Fernando Mercês: Now, from anywhere, If you press Alt-N,

413
00:45:34.470 --> 00:45:43.689
Fernando Mercês: you can go… you have your OS string, which we changed to Ubuntu, And in 1180, we have…

414
00:45:43.810 --> 00:45:46.220
Fernando Mercês: The beginning of the text section.

415
00:45:46.460 --> 00:45:49.350
Fernando Mercês: That's the code, where the code starts.

416
00:45:50.350 --> 00:45:56.650
Fernando Mercês: Alright, in normal conditions. We are looking for the variable again.

417
00:45:57.010 --> 00:46:03.289
Fernando Mercês: So, this is data, right? Because it has to be decremented every time you try

418
00:46:03.510 --> 00:46:09.259
Fernando Mercês: A wrong… you… you provided with a wrong guess in the game.

419
00:46:09.500 --> 00:46:19.010
Fernando Mercês: it decrements its value, so its data. So instead of looking randomly, for… 03 bytes.

420
00:46:19.970 --> 00:46:24.689
Fernando Mercês: over the binary, you can just press enter here, go to the data section.

421
00:46:24.930 --> 00:46:27.979
Fernando Mercês: And start looking for your zero tree now.

422
00:46:28.600 --> 00:46:33.049
Fernando Mercês: And if you look at the size of the data section, it's just only 14 bytes.

423
00:46:33.170 --> 00:46:36.989
Fernando Mercês: Of course, this is a very small game that I wrote myself.

424
00:46:37.230 --> 00:46:43.490
Fernando Mercês: in a… Real-world game, this would be harder, but the concepts are the same.

425
00:46:45.280 --> 00:46:57.449
Fernando Mercês: So this data section has 14 bytes, right? Actually, this is in hex. If you press, yeah, that's… that's 14 in hex, actually, it's not, it's not decimal.

426
00:46:58.630 --> 00:47:01.130
Fernando Mercês: So it's basically this.

427
00:47:01.800 --> 00:47:08.150
Fernando Mercês: Right? All these bytes here is the data section. We have these bytes here that we don't know what they are using for.

428
00:47:08.430 --> 00:47:11.540
Fernando Mercês: And we have our tree in here.

429
00:47:12.010 --> 00:47:17.589
Fernando Mercês: So, if I'm correct here, That's the value that initialized that variable.

430
00:47:18.200 --> 00:47:21.429
Fernando Mercês: So if I press, like, CTRL-A, for example.

431
00:47:21.770 --> 00:47:25.430
Fernando Mercês: This is for… also from VIN. Ctrl-A increments a number.

432
00:47:25.550 --> 00:47:28.070
Fernando Mercês: Ctrl-X decrements that.

433
00:47:28.220 --> 00:47:30.729
Fernando Mercês: It also works here. Let's say 4.

434
00:47:31.670 --> 00:47:33.980
Fernando Mercês: And now I'm going to quit.

435
00:47:34.270 --> 00:47:39.500
Fernando Mercês: Save the file, write to the file, My change, and then quit.

436
00:47:41.820 --> 00:47:43.420
Fernando Mercês: And then run the game.

437
00:47:45.050 --> 00:47:46.400
Fernando Mercês: Then I have 4.

438
00:47:46.840 --> 00:47:51.279
Fernando Mercês: lives. And of course, this will be way more interesting

439
00:47:51.790 --> 00:47:54.770
Fernando Mercês: Oh, you know, I should have added a comment in there.

440
00:47:54.960 --> 00:48:01.069
Fernando Mercês: So, I'll follow… I'll do the same stuff, go to the data.

441
00:48:03.440 --> 00:48:11.410
Fernando Mercês: I can say that this is data, And here is, number… of lives.

442
00:48:12.420 --> 00:48:14.130
Fernando Mercês: Because this way.

443
00:48:17.690 --> 00:48:19.949
Fernando Mercês: Oops, what I did. Okay.

444
00:48:20.430 --> 00:48:25.469
Fernando Mercês: This way, if I… He opened that, and pressed Alt-N.

445
00:48:25.810 --> 00:48:28.760
Fernando Mercês: I can go straight to where I was.

446
00:48:29.150 --> 00:48:32.510
Fernando Mercês: And that's what I wanted, you know, I wanted to be fast.

447
00:48:32.670 --> 00:48:44.519
Fernando Mercês: I'm only using the keyboard here, not touching the mouse, and it's quite fast, you know. When you are typing something, you are, you know, programming, doing some stuff here, you can just,

448
00:48:45.160 --> 00:48:48.590
Fernando Mercês: do this, Alt-N, and I'm there.

449
00:48:48.760 --> 00:48:53.869
Fernando Mercês: So it's very… you can get to the point that you want very… real quick.

450
00:48:55.270 --> 00:49:00.630
Fernando Mercês: And in that case, we can make it more interesting, like a… maybe FF?

451
00:49:01.240 --> 00:49:07.689
Fernando Mercês: And if you forget, you know, the number. There's also a calculator within it, which is,

452
00:49:07.950 --> 00:49:13.330
Fernando Mercês: inspired by, Hill. Hill also has a calculator on it.

453
00:49:13.720 --> 00:49:19.540
Fernando Mercês: That you can use, to… Yeah, to…

454
00:49:20.690 --> 00:49:28.969
Fernando Mercês: verify the numbers, you know, to do whatever. Let's say you want 200 lives, so it's C8, alright? So I can just put here.

455
00:49:29.470 --> 00:49:33.410
Fernando Mercês: R, 200 replace mode, C8.

456
00:49:33.680 --> 00:49:34.630
Fernando Mercês: Enter.

457
00:49:35.060 --> 00:49:36.220
Fernando Mercês: And we are done.

458
00:49:36.960 --> 00:49:41.079
Fernando Mercês: And now… If this is correct… oops.

459
00:49:41.310 --> 00:49:43.369
Fernando Mercês: We have 200 lives.

460
00:49:43.600 --> 00:49:46.590
Fernando Mercês: Yeah. Now they think… now it's much better, right?

461
00:49:46.780 --> 00:49:53.650
Fernando Mercês: It's too low, alright, so 75, too low, right, 80… True.

462
00:49:54.890 --> 00:49:55.990
Fernando Mercês: 90.

463
00:50:00.550 --> 00:50:01.700
Fernando Mercês: Too hard.

464
00:50:04.140 --> 00:50:15.949
Fernando Mercês: 83? Yeah, you got it. Now we cheated, of course, because we changed the number of lives, but still. So that's,

465
00:50:16.390 --> 00:50:22.070
Fernando Mercês: This was, the idea was to show you the… a practical way of,

466
00:50:22.490 --> 00:50:30.779
Fernando Mercês: how to use a hex editor. That's not particular to DZ6, you can do this with any kinds of editor, except the…

467
00:50:31.180 --> 00:50:40.649
Fernando Mercês: the header parsing that shows you where the section code and data sections are, this is not very common to find in other X editors.

468
00:50:41.030 --> 00:50:48.460
Fernando Mercês: I did that because I used that in Hel. Hel does this, which is nice. But other ones? Not many. Well…

469
00:50:48.580 --> 00:50:56.470
Fernando Mercês: Actually, a few of them do this, actually. They have, like, plugins or structures that you can, that you can add this, indeed.

470
00:50:56.710 --> 00:51:08.979
Fernando Mercês: But yeah, I didn't go through all the features, but, just through some of them. But the idea is that I keep, you know, in my… when I have a free weekend, I get a book on, on VIN.

471
00:51:09.230 --> 00:51:14.359
Fernando Mercês: And start reading about its features, and if it looks interesting, I…

472
00:51:14.690 --> 00:51:19.830
Fernando Mercês: try to port that to Z6. Also, in my daily work, when I'm,

473
00:51:20.170 --> 00:51:26.760
Fernando Mercês: Doing, like, reverse engineering binaries and looking for stuff within binaries, vulnerabilities, whatever.

474
00:51:27.060 --> 00:51:44.409
Fernando Mercês: If I feel that a certain feature would help me, and that doesn't exist, then I write it down, because when I have time, I'll try to do this in Z6. There is some very interesting stuff, if you look at the README, but this would take, like.

475
00:51:44.780 --> 00:51:49.069
Fernando Mercês: Too many, too many… too long to show you here.

476
00:51:50.010 --> 00:51:53.799
Fernando Mercês: But yeah, so just to… and…

477
00:51:54.060 --> 00:51:57.330
Fernando Mercês: Close to… to the end,

478
00:51:58.630 --> 00:52:07.630
Fernando Mercês: The code structure, it's basically, you know, something that I came up myself. I'm pretty sure it could be better. If you look at the code itself.

479
00:52:08.120 --> 00:52:14.710
Fernando Mercês: Well, it's, it's very, let's say…

480
00:52:16.110 --> 00:52:27.800
Fernando Mercês: domestic code, right? But it works. Luckily, at first, I was very concerned that I would do everything that I need within a loop.

481
00:52:28.140 --> 00:52:47.929
Fernando Mercês: And, you know, you learn from programming books that you should not… you should avoid branching within a loop, for example, to avoid, for performancy reasons. But, impressively, yeah, Rust is fast enough, and things really work the way, it should.

482
00:52:49.900 --> 00:52:53.100
Fernando Mercês: I have a few,

483
00:52:53.410 --> 00:53:02.299
Fernando Mercês: comments from people out there, they made me very happy, especially because I was not expecting them. It's like a personal stuff.

484
00:53:02.680 --> 00:53:05.310
Fernando Mercês: But, some people are using this.

485
00:53:05.420 --> 00:53:06.680
Fernando Mercês: And

486
00:53:07.200 --> 00:53:24.129
Fernando Mercês: we've… we get some issues on GitHub, people saying it's really cool, people saying… Also, people say, how… how do I operate on this, you know, how do I quit? Because they are not used to Vim, so I realized that if you are new to VIN,

487
00:53:24.410 --> 00:53:27.589
Fernando Mercês: Then ZCC can be a bit…

488
00:53:27.980 --> 00:53:37.770
Fernando Mercês: hard to start with, but the advantage is that if you learn the basics of Z6, you also learn the basics of Veeam, so it's also good.

489
00:53:40.430 --> 00:53:57.270
Fernando Mercês: In publications, I was happy to see the second one was a publication by someone. It's a Brazilian guy, I won't lie, because he knows about DZ6, and he works for this company, WatchGuard, and he…

490
00:53:57.270 --> 00:54:00.949
Fernando Mercês: Analyzed some malware, and published a blog post on this.

491
00:54:00.980 --> 00:54:07.389
Fernando Mercês: And one of the tools that he used is ZZ6 in there. I saw it when I read the article.

492
00:54:07.510 --> 00:54:11.870
Fernando Mercês: And the first one is an article of myself, so it's the company I work for.

493
00:54:12.040 --> 00:54:18.939
Fernando Mercês: And then, of course, it's a bit biased, because I used ZZ6 because, you know, I created that, but

494
00:54:19.530 --> 00:54:26.470
Fernando Mercês: It's still okay, I can say it's being used, out there, at least by

495
00:54:27.040 --> 00:54:34.449
Fernando Mercês: Whatever, 2 people. Maybe 3 after this talk, I don't know. So yeah, it's an open source project, it's okay.

496
00:54:36.070 --> 00:54:38.550
Fernando Mercês: There are challenges as well.

497
00:54:39.380 --> 00:54:46.380
Fernando Mercês: many, many challenges. If you look at the issues on GitHub, I've basically opened the issues for everything that I think

498
00:54:46.580 --> 00:54:48.660
Fernando Mercês: That should be fixed.

499
00:54:48.990 --> 00:54:51.420
Fernando Mercês: And I have no idea how, on how.

500
00:54:51.560 --> 00:54:53.829
Fernando Mercês: I know, at this time,

501
00:54:54.240 --> 00:54:56.330
Fernando Mercês: In this era, I could just have

502
00:54:56.720 --> 00:54:59.830
Fernando Mercês: an LLM to fix everything for me.

503
00:55:00.120 --> 00:55:02.800
Fernando Mercês: But I've put so much passion into this.

504
00:55:03.130 --> 00:55:08.720
Fernando Mercês: So much from my brain, you know, from my, so many hours awake.

505
00:55:08.900 --> 00:55:14.350
Fernando Mercês: So many conversations with my wife, because of who's doing this.

506
00:55:14.720 --> 00:55:22.860
Fernando Mercês: that, at this point, I will not do this, so, this, will continue to be, human-written

507
00:55:22.980 --> 00:55:26.969
Fernando Mercês: thing, because, yeah, it's the way I…

508
00:55:27.220 --> 00:55:34.070
Fernando Mercês: It's my hobby, as well. So, if I have AI doing my hobby for me, like, doing my art for me.

509
00:55:34.880 --> 00:55:40.309
Fernando Mercês: what am I going to do? So, so, no, that's the… so that's the idea. The issues are there.

510
00:55:40.590 --> 00:55:42.050
Fernando Mercês: If you wanna take a look.

511
00:55:42.310 --> 00:55:47.410
Fernando Mercês: For example, just please don't use AI to fix them, I could do this, but it's,

512
00:55:47.680 --> 00:55:52.279
Fernando Mercês: We are not doing this for this reason.

513
00:55:52.400 --> 00:56:03.609
Fernando Mercês: Yeah, so multiple threads. One thing that I tried to do with Rust, because I heard this, right, you can go thread safely, fearless multi-threading with Rust.

514
00:56:04.030 --> 00:56:12.510
Fernando Mercês: I could not do this with the current, Way I designed Z6.

515
00:56:12.630 --> 00:56:18.580
Fernando Mercês: But it's my… my mistake, it should be doable, but it's something to do.

516
00:56:18.860 --> 00:56:32.889
Fernando Mercês: The use of traits, again, I came from C, and here is, you know, some criticism. Not a bad one, but every time I would ask someone what a trait is, people would tell me, oh, it's like interface in C++.

517
00:56:33.720 --> 00:56:43.670
Fernando Mercês: And I was like, great, I don't know what an interface is either, so it doesn't help to me, you know. For C programmers, this stuff usually are,

518
00:56:44.570 --> 00:56:46.989
Fernando Mercês: It's not obvious.

519
00:56:47.040 --> 00:57:04.839
Fernando Mercês: So for me, also functional programming, all these nice features from Rust, they are great, but I spent a lot of time, you know, trying to understand them better, and I still do, to be honest. But it's great, so I'm not complaining about the language, it's just that coming from a C background can be challenging.

520
00:57:06.670 --> 00:57:10.740
Fernando Mercês: Alright, let's, lead us to the end.

521
00:57:10.890 --> 00:57:15.540
Fernando Mercês: of the talk, there is… That's the website.

522
00:57:15.980 --> 00:57:18.390
Fernando Mercês: This is 6.dev.br.

523
00:57:19.160 --> 00:57:24.219
Fernando Mercês: the source code on GitHub, And then there's…

524
00:57:24.220 --> 00:57:29.489
Gabor Szabo: see it now. We're seeing 9, number 9 here, 14, okay, now.

525
00:57:30.860 --> 00:57:33.119
Fernando Mercês: Yeah, I was talking about these challenges here.

526
00:57:33.120 --> 00:57:34.250
Gabor Szabo: Yeah, okay.

527
00:57:35.200 --> 00:57:37.450
Fernando Mercês: And now… thanks, yes.

528
00:57:38.380 --> 00:57:50.429
Fernando Mercês: So the website of ZCC, it's very simple, and then the source code, which has the README with all the commands, which you'll find more useful if you want to try that.

529
00:57:51.570 --> 00:57:57.490
Fernando Mercês: And then there's me on two social networks, different social networks.

530
00:57:59.150 --> 00:58:02.900
Fernando Mercês: And that's… that's it. I guess.

531
00:58:03.400 --> 00:58:06.360
Fernando Mercês: These were, like, 58 minutes?

532
00:58:06.360 --> 00:58:11.730
Gabor Szabo: Thank you very much. There is a question, comment slash question, in the chat, if you can look at it.

533
00:58:12.380 --> 00:58:13.139
Gabor Szabo: Of course, sir.

534
00:58:15.280 --> 00:58:17.640
Fernando Mercês: Oh, I… I agree.

535
00:58:19.140 --> 00:58:22.030
Gabor Szabo: I can read it out if you would like. Should I read it out?

536
00:58:22.460 --> 00:58:23.529
Fernando Mercês: Yes, please, please.

537
00:58:23.530 --> 00:58:34.170
Gabor Szabo: I use this type of things using Cheat Engine to increase money in the video games, so that I can purchase assets from the video games.

538
00:58:34.590 --> 00:58:46.240
Gabor Szabo: However, I have question regarding how Arts Linux will be pointed to the slash ETC slash issue slash something,

539
00:58:47.000 --> 00:58:48.610
Gabor Szabo: Here I got confused.

540
00:58:49.240 --> 00:58:49.720
Gabor Szabo: Mmm…

541
00:58:50.200 --> 00:58:51.359
Fernando Mercês: Oh, okay.

542
00:58:52.750 --> 00:58:54.830
Fernando Mercês: Okay,

543
00:58:56.040 --> 00:59:01.969
Fernando Mercês: I think you… this will be more… this will be clearer if I show you the source code of,

544
00:59:02.790 --> 00:59:05.810
Fernando Mercês: Of the… actually, maybe there is assembly.

545
00:59:06.870 --> 00:59:09.660
Fernando Mercês: Oh… that file.

546
00:59:12.630 --> 00:59:14.640
Fernando Mercês: It should be in Maine.

547
00:59:25.130 --> 00:59:30.450
Fernando Mercês: Yeah, so there is a… Where is this?

548
00:59:35.110 --> 00:59:40.619
Gabor Szabo: And in the meantime, just everyone who is here, if you have any questions, feel free to ask them

549
00:59:40.730 --> 00:59:44.750
Gabor Szabo: And now, in the chat, and then we'll get to them.

550
00:59:48.550 --> 00:59:52.340
Fernando Mercês: Alright, yeah.

551
00:59:53.030 --> 00:59:57.699
Fernando Mercês: So here… At that point in the codes.

552
00:59:58.160 --> 01:00:04.970
Fernando Mercês: a string comparison will… will happen, right? So this is where the code compares the…

553
01:00:05.220 --> 01:00:08.510
Fernando Mercês: Debian string, which is embedded in the binary.

554
01:00:08.990 --> 01:00:14.580
Fernando Mercês: with… Whatever comes from it is an issue.

555
01:00:16.480 --> 01:00:21.130
Fernando Mercês: So, the… and then Gabor asked.

556
01:00:21.290 --> 01:00:26.419
Fernando Mercês: What if I needed to… what if I'm running, let's say, something else, like Arc Linux?

557
01:00:26.550 --> 01:00:33.109
Fernando Mercês: So that will… that will be in a TSA issue. That will come from… from… from… from this.

558
01:00:33.230 --> 01:00:36.360
Fernando Mercês: From the… from a previous read, which is here.

559
01:00:37.950 --> 01:00:41.589
Fernando Mercês: And then it's going to be compared with Debian.

560
01:00:42.300 --> 01:00:50.049
Fernando Mercês: So, the idea was to compare this with something else. That's why I… I told him we could find,

561
01:00:50.420 --> 01:00:54.999
Fernando Mercês: a home, zeroed section, you know, a few…

562
01:00:55.310 --> 01:00:59.820
Fernando Mercês: Bytes that are set to zero, and they're not used by the code.

563
01:00:59.970 --> 01:01:05.260
Fernando Mercês: write down your string there, so let's say Arc Linux, right? And…

564
01:01:05.870 --> 01:01:10.210
Fernando Mercês: Here, when we load that string, which comes from here.

565
01:01:10.570 --> 01:01:13.309
Fernando Mercês: That math here, that's,

566
01:01:13.470 --> 01:01:16.989
Fernando Mercês: calculation here. We end up loading

567
01:01:17.260 --> 01:01:21.509
Fernando Mercês: into this register, the address of the string, of the Debian string.

568
01:01:21.850 --> 01:01:23.940
Fernando Mercês: And then we'd have to change this.

569
01:01:24.870 --> 01:01:33.969
Fernando Mercês: So that, this address would be the address of the new string that we wrote within the binary, in that set of zeros.

570
01:01:35.120 --> 01:01:39.410
Fernando Mercês: You cannot do this, I mean, you can do this only changing the bytes.

571
01:01:39.730 --> 01:01:48.809
Fernando Mercês: But, you'd have to understand, you know, what each byte does, for example, like this. Those bytes here, these bytes are there. You can use Z6 for that.

572
01:01:49.100 --> 01:01:55.790
Fernando Mercês: But it's way easier if you use a disassembler in this case, because it's a more advanced patch.

573
01:01:56.000 --> 01:01:59.119
Fernando Mercês: Or even a decompiler that you can do this.

574
01:01:59.260 --> 01:02:00.510
Fernando Mercês: Very easily.

575
01:02:01.180 --> 01:02:09.959
Fernando Mercês: But yeah, maybe in the future, I can add a disassembly as well, and then we will have a screen such as this, that we can just change the address

576
01:02:10.110 --> 01:02:14.850
Fernando Mercês: Of the variable that's been loaded here, and it's… and then we get it.

577
01:02:15.280 --> 01:02:16.050
Fernando Mercês: Yep.

578
01:02:19.310 --> 01:02:21.350
Fernando Mercês: There is an additional question, right?

579
01:02:21.350 --> 01:02:30.909
Gabor Szabo: Yeah, yeah, it continues, so there's an additional question, I have, which is, if you have to protect binary injections, especially cracking the software.

580
01:02:31.090 --> 01:02:34.930
Gabor Szabo: How we can protect our system from being

581
01:02:35.560 --> 01:02:38.999
Gabor Szabo: Okay, get crack the internal structure.

582
01:02:39.730 --> 01:02:54.579
Gabor Szabo: Specifically, when it comes to license or API keys from using… we protect the user access limitation. Okay, does cryptography help here, or cryptography also can be decoded using some methodologies?

583
01:02:55.350 --> 01:02:58.059
Gabor Szabo: Okay, I think the idea is how can…

584
01:02:58.500 --> 01:03:04.069
Gabor Szabo: how can we protect ourselves from such things? Basically, I think that that's the question, right?

585
01:03:04.680 --> 01:03:05.080
Fernando Mercês: Sure.

586
01:03:05.080 --> 01:03:08.210
Gabor Szabo: If someone wants to… I don't know.

587
01:03:09.010 --> 01:03:09.520
Gabor Szabo: Yeah.

588
01:03:09.520 --> 01:03:10.240
Fernando Mercês: Yep.

589
01:03:10.380 --> 01:03:14.189
Fernando Mercês: Yeah, in general, I mean, everything that you… that you put

590
01:03:14.560 --> 01:03:18.120
Fernando Mercês: That it's within your binary,

591
01:03:18.710 --> 01:03:27.170
Fernando Mercês: someone can see, right? You can encrypt, let's say, for example, you can find a… You, you…

592
01:03:28.090 --> 01:03:31.809
Fernando Mercês: Let's think that this, let's suppose this data is encrypted here.

593
01:03:31.870 --> 01:03:37.319
Fernando Mercês: But then the code would have to decrypt it somehow, so the key would have to be stored here.

594
01:03:37.350 --> 01:03:55.170
Fernando Mercês: For example, or maybe an internet connection should be done, and then get the key, and then using to decrypt, but the query will pass through memory anyhow. So, software protection is actually, is hard. There are, even…

595
01:03:55.220 --> 01:03:56.810
Fernando Mercês: commercial solutions.

596
01:03:57.280 --> 01:04:01.989
Fernando Mercês: for this, they are called protectors, in general, software protectors.

597
01:04:02.230 --> 01:04:09.039
Fernando Mercês: That they do their best, especially on the game world, the gaming world, they use it a lot.

598
01:04:09.080 --> 01:04:26.460
Fernando Mercês: to avoid cheating. So they… they load drivers during the game, they… they look for any kinds of modifications, they calculate hashes of all these bytes, for example. So if you change anything, this hash will be different.

599
01:04:26.680 --> 01:04:30.110
Fernando Mercês: But everything, or mostly everything, runs

600
01:04:30.710 --> 01:04:39.280
Fernando Mercês: on the local machine of whoever is using the software. So, there's still room for being,

601
01:04:39.730 --> 01:04:56.350
Fernando Mercês: overcame, right, for being cracked, so there's still room. I'm not aware of any software protection that I can say that is 100% effective. I'm aware of some that people would call, very difficult, for example.

602
01:04:57.400 --> 01:05:04.700
Fernando Mercês: There's one called Temida, there's another protector called VM Protect. They do very advanced stuff.

603
01:05:04.900 --> 01:05:08.040
Fernando Mercês: Maybe Enigma is another one.

604
01:05:08.340 --> 01:05:12.309
Fernando Mercês: And they work on both Linux and Windows, most of them.

605
01:05:13.110 --> 01:05:21.660
Fernando Mercês: the… The anti-cheat, I think it's called Inuvo, the company that have a good anti-cheat engine as well.

606
01:05:23.180 --> 01:05:31.369
Fernando Mercês: I know that they… they are hard, but depending on the way… also, they are complex, so depending on the way someone,

607
01:05:32.020 --> 01:05:33.180
Fernando Mercês: uses them.

608
01:05:33.450 --> 01:05:35.509
Fernando Mercês: They can still be defeated.

609
01:05:35.810 --> 01:05:43.910
Fernando Mercês: Yeah. So, yeah, it's a fascinating field, to be honest, because you have to, to, to deliver

610
01:05:44.010 --> 01:05:48.880
Fernando Mercês: Your secrets to your user, And… but…

611
01:05:49.350 --> 01:06:00.739
Fernando Mercês: you do not want your user to look into it. It's kind of, right? It's very hard. But it's doable, yeah, somehow.

612
01:06:02.890 --> 01:06:11.969
Gabor Szabo: Yeah, thank you very much. I actually would say that I would be really, really interested if at a later point you could

613
01:06:12.050 --> 01:06:29.040
Gabor Szabo: create a presentation, or give a presentation, which is a lot more into hacking various of these softwares. So you showed some very interesting example, but I don't know if you'll have time or the interest to do some deeper

614
01:06:29.220 --> 01:06:32.270
Gabor Szabo: examples.

615
01:06:32.710 --> 01:06:40.800
Gabor Szabo: Maybe if, the software is not, not,

616
01:06:41.380 --> 01:06:43.479
Gabor Szabo: What is it called, DZ6, right?

617
01:06:44.010 --> 01:06:46.279
Fernando Mercês: This is 6, yeah. It's delay…

618
01:06:46.280 --> 01:06:48.930
Gabor Szabo: And I say, why is it called ZCC? Did you mention that?

619
01:06:49.380 --> 01:06:51.759
Fernando Mercês: I didn't, that's a good question.

620
01:06:51.910 --> 01:06:59.969
Fernando Mercês: It's, because in Portuguese, when you say 16, 16 in Portuguese, you know, Z6, it reads the same.

621
01:07:00.560 --> 01:07:00.900
Gabor Szabo: Honestly.

622
01:07:00.900 --> 01:07:04.400
Fernando Mercês: Yeah, it's basically a business, yeah, so…

623
01:07:04.400 --> 01:07:04.810
Gabor Szabo: Okay.

624
01:07:04.810 --> 01:07:10.480
Fernando Mercês: It's a homage to the… Hexadecimal base, number base, yeah.

625
01:07:10.480 --> 01:07:19.480
Gabor Szabo: Interesting. So anyway, so if TZ6 is not good enough for that hacking, it might be a good opportunity for you to

626
01:07:19.650 --> 01:07:21.570
Gabor Szabo: To add those features.

627
01:07:21.620 --> 01:07:22.989
Fernando Mercês: So if those things.

628
01:07:22.990 --> 01:07:31.859
Gabor Szabo: So it could be a, okay, I'm preparing this presentation, and for that, I need the Z6 to be able to do this and this and this, for example, what you just showed.

629
01:07:31.860 --> 01:07:46.140
Gabor Szabo: And then it could be a driving force for you as well, but it would be definitely interesting to me, and probably for many other people, how to… how to do this. And I think I forgot the other thing that I wanted to say.

630
01:07:46.140 --> 01:07:51.180
Gabor Szabo: Yeah, that's the main thing, I think, what I wanted to say.

631
01:07:51.330 --> 01:07:53.189
Gabor Szabo: That it would be really interesting.

632
01:07:54.050 --> 01:08:03.749
Fernando Mercês: Yeah, yeah. Yeah, yeah, that's on the plan, you know, to do the disassembly, and maybe in the future, if I know how to code one, even a decompiler.

633
01:08:04.050 --> 01:08:09.620
Fernando Mercês: In the future. These are areas of study of mine, so…

634
01:08:10.590 --> 01:08:14.790
Fernando Mercês: I… I end up… I will probably add more stuff here.

635
01:08:16.890 --> 01:08:24.349
Gabor Szabo: Yeah, by the way, I remember the other thing, sorry. So you were talking about, ways of,

636
01:08:24.500 --> 01:08:40.339
Gabor Szabo: protecting the software, and you say that it's… basically, it's impossible, but you can do it very… to make it very difficult. And at the end, it's a lot, the question about how much effort the…

637
01:08:40.680 --> 01:08:47.730
Gabor Szabo: person who is going to attack the software is ready to invest. What is the,

638
01:08:48.170 --> 01:08:55.069
Gabor Szabo: Value they might get from, from, breaking, cracking your software.

639
01:08:55.069 --> 01:09:12.889
Gabor Szabo: Which reminded me of, I don't know, probably you remember the good old time when people were creating websites, and they really wanted to… and then people said, okay, but I can see how you created the code, because I just can view source.

640
01:09:13.170 --> 01:09:13.770
Gabor Szabo: And then…

641
01:09:13.779 --> 01:09:14.099
Fernando Mercês: Yeah.

642
01:09:14.100 --> 01:09:19.429
Gabor Szabo: Various websites appeared that if you clicked Use Source, it was empty.

643
01:09:20.700 --> 01:09:36.459
Gabor Szabo: And people were starting to say, look, they managed to do it, and it's empty, you can't see the source. And then you just had to tell them, okay, scroll down a little bit, and you will see the source code. They just ended, I don't know, 500 empty rows at the top.

644
01:09:37.170 --> 01:09:37.630
Fernando Mercês: Yeah.

645
01:09:37.630 --> 01:09:46.589
Gabor Szabo: HTML disregarded, or the browser disregarded, and… but those people who didn't put in the… that minimal effort of understanding this.

646
01:09:46.729 --> 01:09:51.899
Gabor Szabo: And scrolling down, For them, it was… Invisible.

647
01:09:52.850 --> 01:09:53.830
Fernando Mercês: Yeah, yeah, yeah.

648
01:09:54.020 --> 01:09:59.119
Gabor Szabo: It's just pointing out, how this… okay, how much effort you need.

649
01:10:00.330 --> 01:10:10.309
Fernando Mercês: Yes, exactly. And that effort actually has been lowered now, right, with agents. Yeah. You can ask an agent to do this, and with the right tools set up.

650
01:10:10.570 --> 01:10:14.459
Fernando Mercês: It will likely go to some results.

651
01:10:14.700 --> 01:10:17.030
Fernando Mercês: Yeah, but, but…

652
01:10:17.200 --> 01:10:23.840
Fernando Mercês: But you still have to understand the basics, right? Even to know whether it's possible or not. Or maybe not, maybe you can just ask an agent, but still…

653
01:10:24.200 --> 01:10:29.979
Fernando Mercês: I mean, yeah, it's difficult. It's, at the same time, it's a fascinating area, something that I like a lot.

654
01:10:30.400 --> 01:10:35.540
Fernando Mercês: I've been using hex heditors for more than 20 years, I know, like, really old ones.

655
01:10:35.870 --> 01:10:42.420
Fernando Mercês: And I don't know, I just like those kind of tools. But I also understand that,

656
01:10:43.350 --> 01:10:46.179
Fernando Mercês: An agent could do all this stuff right now.

657
01:10:46.350 --> 01:10:47.750
Fernando Mercês: So, yeah.

658
01:10:48.420 --> 01:10:53.055
Gabor Szabo: Yeah, maybe. Yeah…

659
01:10:54.340 --> 01:11:09.459
Gabor Szabo: Yeah, we got a comment, the AI agents refuse to do such illegal code writings. I am not sure that all of the… and I don't know, maybe they do, but I'm not sure that the… I mean, it's not illegal to do this for your own software.

660
01:11:09.860 --> 01:11:10.270
Fernando Mercês: Oh, no!

661
01:11:10.270 --> 01:11:13.419
Gabor Szabo: So… Not at all. It's,

662
01:11:14.200 --> 01:11:23.549
Gabor Szabo: it's all kind of… it's an open question, and I'm pretty sure that there are tools that won't abide by whatever rules others reside, so…

663
01:11:24.110 --> 01:11:31.189
Fernando Mercês: Yeah, because there's stuff, there's stuff you can actually patch, like, for example, unmaintained software, when it's commercial, when it's proprietary.

664
01:11:31.490 --> 01:11:39.810
Fernando Mercês: Let's say it's, it has some configuration path, hard-coded. It's in the code.

665
01:11:39.950 --> 01:11:46.669
Fernando Mercês: And it's compiled, right? And you have no way to change that. By using such tools, like with reverse engineering in general.

666
01:11:46.900 --> 01:12:04.529
Fernando Mercês: You can go there and then change. You can fix a bug if it's unmaintained, if maybe the maintainer is not around anymore for some reason. It's an old software for a variety of reasons. You can also use this to understand the proprietary software, how proprietary software works.

667
01:12:05.360 --> 01:12:10.810
Fernando Mercês: And then you create your own, which is compatible, somehow.

668
01:12:11.220 --> 01:12:14.730
Fernando Mercês: Or to check the data, let's say you generate a PDF.

669
01:12:15.200 --> 01:12:29.300
Fernando Mercês: And then you use a hex editor to look at this PDF and to see what kind of metadata is being carried over together with the file. Maybe it has your name as well, maybe it has the time you created, maybe that's not what you want.

670
01:12:29.300 --> 01:12:31.160
Gabor Szabo: Maybe you'll information.

671
01:12:31.930 --> 01:12:41.650
Fernando Mercês: Yeah, I mean, yeah, it's good to check, you know. Look to have a way to look at the file at the…

672
01:12:42.430 --> 01:12:45.949
Fernando Mercês: at the raw level, if I can say that, the lowest level.

673
01:12:46.350 --> 01:12:46.990
Gabor Szabo: Yeah.

674
01:12:47.090 --> 01:12:58.599
Gabor Szabo: Very interesting, very interesting. Thank you very much, it seems that we don't have many other questions, or other questions. So.

675
01:12:59.200 --> 01:13:17.280
Gabor Szabo: Thank you for giving this presentation. I'm going to stop the video recording in a second. I'm just telling those people who are watching the video to like it and follow the channel, and check the links below the video where you can find… get to the notes of this slide and a couple of links.

676
01:13:17.390 --> 01:13:32.880
Gabor Szabo: And for the future videos, and if you are here, and you would like to, in the session, and would like to keep chatting about other stuff, or this, feel free to stay around after the video,

677
01:13:32.880 --> 01:13:41.649
Gabor Szabo: After I stop the video, I will allow everyone to speak, and that's it. Thank you very much. Goodbye.

678
01:13:41.960 --> 01:13:43.080
Fernando Mercês: Thanks, everyone!

