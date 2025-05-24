---
title: Rust at Work a conversation with Ran Reichman Co-Founder and CEO of Flarion
timestamp: 2025-05-15T07:00:01
author: szabgab
published: true
show_related: true
description:
---

<a href="https://rustacean-station.org/episode/ran-reichman/">Listen to the recording</a>.

* Guest: [Ran Reichman](https://www.linkedin.com/in/ran-reichman-740163b7/) Co-Founder & CEO of [Flarion](https://www.flarion.io/)
* Host: [Gabor Szabo](https://szabgab.com/)
* Language: English
* Location: Zoom

In this conversation we'll discuss the use of Rust at Flarion.

Ran is co-founder and CEO of Flarion, a company building high-performance data processing systems using Rust.

Prior to Flarion, Ran built high-scale data processing systems in cybersecurity, autonomous vehicles, and cloud infrastructure.

## Transcript

1
00:00:02.840 --> 00:00:25.500
Gabor Szabo: Hello and welcome to the Code-maven live meeting. And if you're listening to the podcast then it's supposed to be the rustation station. If everything goes well, then, this conversation is going to be uploaded there as well. My name is Gabor Sabo. I usually help companies with rust

2
00:00:25.800 --> 00:00:45.769
Gabor Szabo: and python, and in this series of of meetings and conversations I try to discuss with guests how the the title is rost at work, so I'm trying to understand how people use rust at work, and

3
00:00:45.980 --> 00:00:51.359
Gabor Szabo: what are the considerations? What are the advantages? Maybe disadvantages? And so on.

4
00:00:51.610 --> 00:00:59.340
Gabor Szabo: And this time the 1st guest is Ron Reichman from Florian, Hi Ron.

5
00:00:59.720 --> 00:01:01.610
Ran Reichman: Hello, yeah. It's great to be here.

6
00:01:02.710 --> 00:01:06.595
Gabor Szabo: Thank you for accepting the invitation, and

7
00:01:07.530 --> 00:01:17.960
Gabor Szabo: let's get started. I think the 1st thing should be is that you you introduce yourself. We actually never met. So it's gonna be a 1st for me as well.

8
00:01:18.450 --> 00:01:25.170
Ran Reichman: Okay, excellent. Yeah. So thanks again for for having me here and excited for this chat.

9
00:01:26.269 --> 00:01:46.930
Ran Reichman: I'm Ron Reichman, a co-founder, and CEO with clarion. And what we do is at Clarion is we're a data processing company building solutions to accelerate and make data processing much more efficient, especially at large scale of the big data which means usually like

10
00:01:47.928 --> 00:01:53.750
Ran Reichman: terabytes, tens of terabytes, hundreds of terabytes. But we support a wider range of workloads.

11
00:01:53.930 --> 00:02:01.579
Ran Reichman: And yeah, we're definitely a rust shop and lots to talk about in terms of why rust, how we use rust, etc.

12
00:02:03.090 --> 00:02:14.739
Gabor Szabo: Can. Can you tell a little bit about your about your background? And this is a startup company, right? How long, how long? How long is the company? And what did you before? And how did you get there? That's that's sort of the.

13
00:02:16.080 --> 00:02:31.519
Ran Reichman: Yeah, it's a great question. So I'm born and raised in Israel and in the army. I was in the program which has recently become more famous, thanks to the whiz founders that are also graduates spent about 8 years

14
00:02:31.730 --> 00:02:37.570
Ran Reichman: in various intelligence units, low level development stuff like that.

15
00:02:38.384 --> 00:02:51.085
Ran Reichman: Then went on to work on a on a news startup briefly, and then spent about 4 and a half years in the in the United States and working on

16
00:02:51.900 --> 00:02:56.640
Ran Reichman: on autonomous vehicles and on cloud infrastructure.

17
00:02:57.424 --> 00:03:04.729
Ran Reichman: So I had a chance to work at Roblox, which has a data at a very significant scale, and before that, at embark trucks.

18
00:03:04.950 --> 00:03:09.610
Ran Reichman: and for the past 2 and a half, 2 years or so I've been working on what is now clarion.

19
00:03:11.050 --> 00:03:11.715
Gabor Szabo: Okay,

20
00:03:12.540 --> 00:03:18.980
Gabor Szabo: So it's flaring is is 2 years. And and when did you start using rust? Or when did you get

21
00:03:19.140 --> 00:03:26.519
Gabor Szabo: to start? Yeah with rust. Was it in Florian, or or already you started to use rust earlier.

22
00:03:27.520 --> 00:03:39.599
Ran Reichman: So I'm my. My background is C, plus, plus. I have a ton of ton of experience with the C plus plus. And when I started building Clarion. I knew that I was that I was interested in in using rust.

23
00:03:40.009 --> 00:03:46.669
Ran Reichman: Because primarily, I mean, there's there are several reasons that we can go into, but that that's when I started going.

24
00:03:48.270 --> 00:04:06.279
Gabor Szabo: Yeah, I'm definitely interested in in in. Why did you start? Why did you think that is good for for this company? And this is a startup company. So one of the things that I hear sometimes that it's not maybe not such a good language for startups.

25
00:04:09.420 --> 00:04:32.499
Ran Reichman: Yeah. So I I would disagree. I think I think it's a good language for startups. I'd say 3 main reasons that we're using spark that we're using rust here at the flare on the 1st is we're connecting to existing data infrastructure for our customers, and our customers have a very high expectations in terms of reliability.

26
00:04:33.603 --> 00:04:36.929
Ran Reichman: So it's a

27
00:04:37.596 --> 00:04:46.779
Ran Reichman: it's a it's a quite a significant advantage. The fact that we don't really have to worry about the memory, memory, corruption, and everything that that entails.

28
00:04:47.090 --> 00:04:49.459
Ran Reichman: so that that that was one thing

29
00:04:50.017 --> 00:05:06.560
Ran Reichman: the second is, there's quite a lot of interesting infrastructure today in the rust world. There's also a lot of great stuff in C, plus plus. But things like polars, like data fusion. These are open source projects that we were happy to plug into and integrate with.

30
00:05:06.760 --> 00:05:31.820
Ran Reichman: And the 3, rd which is an interesting one is that we kind of like the people who are into rust. So what we do at flare on is kind of it requires a lot of out of the box, thinking it requires trying out new systems, new ideas, and oftentimes the kind of people the kinds of people that are interested in the working in rust are also the kinds of people that we want to have at Larry on. Those were the 3 main reasons.

31
00:05:33.550 --> 00:05:34.460
Gabor Szabo: Okay.

32
00:05:34.520 --> 00:05:55.609
Gabor Szabo: Before I go, continue with this. Actually, I would like to ask something about this. I would like to mention that, unlike other podcasts or conversations, or whatever we have here, where there's only 2 people speaking here, we have a few other guests who are watching us and listening to us.

33
00:05:55.610 --> 00:06:13.769
Gabor Szabo: and I would like, and though they can't speak, but they can definitely ask questions in the chat. So I would like to invite everyone who is present that if you have any questions, just type in the question, and I'm going to take care, and we are going to. I'm going to ask those questions here as well.

34
00:06:13.790 --> 00:06:20.239
Gabor Szabo: So here already we have the the 1st questions from the from the audience is, Have you considered other languages.

35
00:06:21.470 --> 00:06:26.619
Ran Reichman: Yeah. So my, my, I definitely considered other languages. The default for me was to use the C plus plus

36
00:06:28.390 --> 00:06:42.050
Ran Reichman: I have a lot of good things to say about C, plus plus. I'm definitely not a hater of the language. I think so so much over the existing infrastructure. Of the of the software world was built on on top of C, plus plus

37
00:06:42.532 --> 00:06:49.617
Ran Reichman: but yeah, I think the 3 the 3 reasons that I mentioned or pushed us towards Ross.

38
00:06:50.730 --> 00:06:52.339
Ran Reichman: but yeah, and.

39
00:06:52.340 --> 00:07:08.300
Gabor Szabo: Okay, I'm actually especially interested in the 3rd one that you mentioned of the the type of people. I think that was the the 3rd one. So can you describe what? How would you describe this this type, or how do they? How do they differ from? I don't know

40
00:07:08.480 --> 00:07:09.360
Gabor Szabo: others.

41
00:07:10.050 --> 00:07:20.130
Ran Reichman: Yeah, so any generalization is wrong. Right? So there there are. It's it's hard to categorize people for sure. But what we've seen

42
00:07:20.240 --> 00:07:43.149
Ran Reichman: is that when people are curious about rust they're usually just curious people. They hear about a new technology, and they want to understand it better. And they want to try it out, and they wanna explore it. And we're a company fundamentally what we do like if if we zoom out is we look at it as like receiving high level instructions

43
00:07:43.753 --> 00:07:47.960
Ran Reichman: from our customers like this is what I wanna achieve with my data.

44
00:07:48.280 --> 00:07:54.179
Ran Reichman: And then under the hood, we, we might use different engines, different technologies, different techniques.

45
00:07:54.741 --> 00:08:09.439
Ran Reichman: So we're we're we have a very broad stack in terms of things that we need to understand and support. So we really want people who are curious about new technology and want to adopt a different and interesting technologies. Yeah.

46
00:08:10.460 --> 00:08:18.469
Gabor Szabo: Okay. So there's another question here which is actually jumping a little bit ahead, asking about Flurry and and get on, Github, which

47
00:08:18.960 --> 00:08:26.850
Gabor Szabo: it was sort of planned to be that I asked later on whether any of any part of Florian is open source.

48
00:08:27.330 --> 00:08:30.509
Gabor Szabo: and and if if he has done, where is it?

49
00:08:30.760 --> 00:08:31.890
Gabor Szabo: Hey? Welcome.

50
00:08:31.890 --> 00:08:32.789
Ran Reichman: Yeah, so

51
00:08:33.679 --> 00:08:51.449
Ran Reichman: excellent question. We're huge fans of open source. And we also build on top of open source. We don't currently have a public repository. We do, do a lot of contributions particularly to projects like a data fusion and arrow

52
00:08:52.087 --> 00:08:59.859
Ran Reichman: so that that's mostly our involvement in open source that like, we were heavy users of existing technologies. And we're able to push them to the limit

53
00:09:00.060 --> 00:09:13.719
Ran Reichman: and find interesting bugs, edge cases, etc. And yeah, and unpack them, unpack them, debug them and issue pull requests. So we have. We have quite a bit of those in the in the open source. Ecosystem.

54
00:09:13.720 --> 00:09:42.949
Gabor Szabo: If you're already started to go in this direction, then I have a few more questions, because I mean you mentioned 2 major projects that you're rely on major open source projects. But I guess you have tens, or maybe hundreds of of rust crates that use. And if you think about all the hierarchy that maybe thousands. And I'm really interested in. How do you deal with all the dependencies, the

55
00:09:43.550 --> 00:09:46.289
Gabor Szabo: all the problems that come with the dependencies.

56
00:09:47.620 --> 00:10:00.109
Ran Reichman: Yeah. So I don't think there's a good solution to this, like one of the challenges with rust, which isn't necessarily unique to us. But it's very you can feel it very strongly when you're building a a big rust project is

57
00:10:00.892 --> 00:10:21.569
Ran Reichman: the number of crates that grows quite rapidly you start with like 50. And then you're at 150. It's only 500. So yeah, definitely, lot of lots of dependencies. We invest in the Ci infrastructure so that the build is as fast as possible. We we're constantly maintaining

58
00:10:21.720 --> 00:10:24.008
Ran Reichman: the like the whole

59
00:10:25.040 --> 00:10:31.749
Ran Reichman: build set up in order for it to be as quick as possible. Sometimes. That also means using like powerful servers

60
00:10:32.212 --> 00:10:36.077
Ran Reichman: but I don't think there's a silver bullet here in terms of

61
00:10:36.812 --> 00:11:01.139
Gabor Szabo: What? Yeah. What I'm interested in is, how do you select crates? How do you make sure that they are? How how much can you invest in, or how much do you invest in verifying that they are secure enough that they performant enough? And how do you deal with with bugs, or any issues in these these crates.

62
00:11:02.440 --> 00:11:11.110
Ran Reichman: Yeah. So if if we encounter a a, we're quite picky so if if something is small enough, then we won't take a crate, but actually implement it.

63
00:11:11.574 --> 00:11:17.419
Ran Reichman: But in terms of bugs and reliability. So that's the that's the thing that I think we we invest more than anything else

64
00:11:17.740 --> 00:11:19.719
Ran Reichman: in terms of just like

65
00:11:20.614 --> 00:11:28.730
Ran Reichman: we really have tons and tons of tests to make sure that reliability is that is really as close as possible to 100%

66
00:11:30.390 --> 00:11:36.083
Ran Reichman: And that also is the case. When we introduce new crates, like, we're also we're we're we have

67
00:11:36.540 --> 00:11:56.890
Ran Reichman: a very expensive suite of testing for that in terms of a a security that's mostly a matter of reputation, like we don't choose crates that we think will do a bad job for us or that were that are shady. We look at like number of downloads. Are people using this in the ecosystem stuff like that?

68
00:12:00.420 --> 00:12:02.709
Ran Reichman: Yeah, I think that's mostly it.

69
00:12:03.020 --> 00:12:04.909
Gabor Szabo: Have you? Have you

70
00:12:05.070 --> 00:12:16.720
Gabor Szabo: considered or tried, or do you think that there is a need of financing, sponsoring certain open source developers who are

71
00:12:16.960 --> 00:12:26.660
Gabor Szabo: unrelated to your company, who maintain these these packages, or or bug bounties, or things like this.

72
00:12:27.540 --> 00:12:30.942
Ran Reichman: Yeah, I I completely, I think,

73
00:12:31.620 --> 00:12:35.570
Ran Reichman: I think companies like us should definitely invest in these kinds of things.

74
00:12:36.193 --> 00:12:39.720
Ran Reichman: We haven't yet detected like a

75
00:12:40.000 --> 00:13:02.590
Ran Reichman: the right projects in terms of the ones that we're actually heavily using and in need of these resources. Because we're pretty in the mainstream, like lots of Apache stuff, like lots of stuff that's already has a lot of investment. But yeah, if we encounter something that we can contribute to or even contribute financially, definitely, something that we'd consider. We we want the ecosystem to be sustainable.

76
00:13:03.920 --> 00:13:28.949
Gabor Szabo: Okay. So I think we can go back to to the mainstream sort of of this conversation. But actually, there are questions here that sort of related to this, which let me read it, because that's easier. Also, could you please elaborate a bit more on the nature of rust in your use case, what do you mainly do with rust? Do you use Asyncrust?

77
00:13:30.480 --> 00:13:33.583
Gabor Szabo: Yeah. So great question.

78
00:13:34.680 --> 00:13:42.290
Ran Reichman: The the nature of our use of rust is we use rust, basically in all of the data processing that we that we do

79
00:13:42.792 --> 00:13:50.970
Ran Reichman: so the way our system works is that we connect to existing infrastructure. So it can be something like written in Scala, for instance, Apache spark

80
00:13:51.885 --> 00:13:58.399
Ran Reichman: or in python and some of the ray infrastructure. And once we pass on

81
00:13:59.141 --> 00:14:05.909
Ran Reichman: the responsibility of the data processing from the high level language to a rust.

82
00:14:06.340 --> 00:14:18.049
Ran Reichman: We do everything in rust. We use a Jni when it's a Java virtual machine, and and python, we also use the relevant bindings to pass from

83
00:14:18.500 --> 00:14:28.769
Ran Reichman: from python to rust. And then it's basically rust all the way until we we have to give back the some sort of result. And then and then send it back.

84
00:14:29.470 --> 00:14:36.040
Gabor Szabo: So besides rust, what are what other languages do you use? Separate.

85
00:14:36.040 --> 00:14:37.139
Ran Reichman: Yes, we we use.

86
00:14:37.140 --> 00:14:42.359
Gabor Szabo: Python and and but I don't know if you're using actually or just somehow connecting.

87
00:14:42.360 --> 00:14:47.319
Ran Reichman: We're mostly connecting like we're using Scala and Python Scala more than python

88
00:14:47.896 --> 00:14:51.089
Ran Reichman: because of its use in Jvm environments.

89
00:14:52.790 --> 00:14:53.930
Ran Reichman: And

90
00:14:54.420 --> 00:15:06.250
Ran Reichman: but whenever we can implement something that's that requires performance, we do. We do it in rost. And that's most of what we do requires a a pretty significant performance investment.

91
00:15:08.060 --> 00:15:20.129
Gabor Szabo: Okay, so we can now circle back to the to the beginning of the company, and how rust was introduced to the company, and how and and how you

92
00:15:20.600 --> 00:15:35.980
Gabor Szabo: find new employees. I mean, this was a startup company. So there were. There was no one there. So it's not like a company where you have to start teaching people rust or so. It's totally your decision. If you are bringing in people with rust experience or

93
00:15:36.120 --> 00:15:40.115
Gabor Szabo: without, and then teaching them, how was that process?

94
00:15:40.850 --> 00:15:46.509
Gabor Szabo: What kind of people arrived? They were? Were they already experienced in rust.

95
00:15:47.640 --> 00:15:51.330
Ran Reichman: Yeah. So most of the people we hired are experienced in rust

96
00:15:52.128 --> 00:16:12.230
Ran Reichman: they have. sometimes that means just a general like generally, like as a hobby, have built things in rust. There's some crypto aspect because of the success of the Solana as a blockchain, so that that spurred a lot of rust development

97
00:16:12.762 --> 00:16:20.679
Ran Reichman: so we have a bit from there. And there are a few people who don't really have a rust background, but are a strong C plus plus developers.

98
00:16:20.870 --> 00:16:23.070
Ran Reichman: And there we were able to.

99
00:16:26.037 --> 00:16:46.762
Ran Reichman: there, there! It's more of a an investment as a startup. We don't do a lot of training. But if someone in our interview process shows potential and is able to learn rust relatively quickly, then we. We also consider those people. And we have a few of those. I'll say that

100
00:16:48.140 --> 00:16:58.174
Ran Reichman: we try just as a as a philosophy not to push rust to the limit in terms of performance. We push it to the limit in terms of

101
00:16:58.640 --> 00:16:59.560
Ran Reichman: and

102
00:16:59.990 --> 00:17:12.010
Ran Reichman: low level implementations, and we push it to the limit. But we don't like we don't try to be too sophisticated or too clever with the language. That's kind of my experience with C plus plus as well that like, it's possible to be very.

103
00:17:12.190 --> 00:17:33.960
Ran Reichman: very smart and very clever, but sometimes that makes the code harder to maintain. So if if one of you took a look at our code base, you'd see that it's it's mostly straightforward. And when we're doing something that's elaborate, it's usually because that was the last resort. And that's kind of my philosophy, regardless of rust, but especially in rust. Because you you can do a lot of very sophisticated stuff.

104
00:17:35.790 --> 00:17:47.790
Gabor Szabo: Can you elaborate a bit more? What are these things that you feel that there are too sophisticated, or or that you would like to that you're avoiding in in this case.

105
00:17:48.830 --> 00:17:50.340
Ran Reichman: Yeah. So I think,

106
00:17:52.090 --> 00:18:02.329
Ran Reichman: I mean, not. Not all of these are necessarily sophisticated, but the they add complexity. So when it comes to handling channels. Async. Things things in that area.

107
00:18:03.990 --> 00:18:31.159
Ran Reichman: sometimes sometimes. You have to use it. But if you can, if you can be like single core, running very powerfully and achieving your goals. Then we prefer that when it's possible that that's kind of the things that we're looking for. So anything that's like, I mean, this is this is the way in computer science generally, right? Like, once, once you introduce concurrency of various forms. Then things. Things become much more complicated the way when we can avoid that, we avoid it.

108
00:18:32.520 --> 00:18:59.869
Gabor Szabo: Right? So there was a question here that I'm not sure that too relevant to you. But maybe you have some input on this. So how easy for you to recruit developers! Our company is mostly C plus plus and python. And when I introduced Rust Code into the Python group for performance. I saw a lot of resistance from the developers, so I know that it's probably not, really. You haven't probably encountered this, but.

109
00:19:00.310 --> 00:19:08.610
Ran Reichman: I actually have some relevant experience, because I've I've interviewed a lot of people with C plus plus background. And I think there's a there's a divergence there are people who

110
00:19:09.303 --> 00:19:24.626
Ran Reichman: who really love c plus plus really appreciate what it's able to bring to the table. And they don't wanna switch to another language. And when we encounter those in our recruiting process, then we don't try to convince them, because they love what we're doing, and we're not doing what they're doing. And that's great

111
00:19:25.030 --> 00:19:33.180
Ran Reichman: and that that really, I think when when you're in an existing company and you want to introduce rust. I think. Actually, the Linux kernel

112
00:19:33.630 --> 00:19:36.849
Ran Reichman: did a great job in this regard when they're saying like.

113
00:19:37.070 --> 00:19:40.390
Ran Reichman: you can't, you don't really wanna migrate to rust

114
00:19:41.645 --> 00:19:56.979
Ran Reichman: cause, you really want to just migrate to anything basically in the software world, you wanna gradually add functionality show value. See? See that you're able to provide better, developer experience and improved velocity.

115
00:19:57.130 --> 00:20:01.643
Ran Reichman: and then you can encourage people to move on. But I think

116
00:20:02.190 --> 00:20:18.619
Ran Reichman: so if you're starting a new project. This is also what we're seeing in the data processing world. So if a company is using Apache spark and you say, Oh, no, you should use ray, or you should use duck. dB, usually, it's a tough sell because they're using a certain technology. And they like that technology. And they're proficient in that technology.

117
00:20:18.840 --> 00:20:26.089
Ran Reichman: And it's much easier to say, Okay, we're like in this new new team that's building a new technology from scratch.

118
00:20:26.320 --> 00:20:34.400
Ran Reichman: We can try something new. And I found that throughout my experience, and that that's much easier than moving a system from one technology to another.

119
00:20:34.961 --> 00:20:45.909
Ran Reichman: Yeah, that's also the case, like, regardless of rust and C plus plus like Java, go like golang. All of that stuff like there's there's ton of tons of examples in that regard.

120
00:20:47.520 --> 00:20:56.690
Gabor Szabo: The the people who are so. So you recruited a number of people who who basically didn't know rust, but were interested in in rust as well. Right.

121
00:20:58.663 --> 00:21:05.120
Gabor Szabo: What was their learning, process, or learning resources that you might recommend to to everyone else?

122
00:21:12.360 --> 00:21:17.029
Ran Reichman: I think 1st of all, the the Standard Rust Book.

123
00:21:17.350 --> 00:21:31.880
Ran Reichman: It's a good book. There's a lot of good stuff there. I think I read it cover to cover, like online, obviously. But I think I think when you want to get acquainted with rust and what it has to offer, then that's a good thing to use.

124
00:21:32.722 --> 00:21:39.955
Ran Reichman: I think when you're in learning mode, and you want to understand things, then,

125
00:21:41.140 --> 00:21:52.829
Ran Reichman: AI is quite useful like saying like, Hey, I know how to write this in c plus plus like, how would you write this in rust? And then you say, like, explain to me this decoration. Explain why it's not compiling stuff like that.

126
00:21:53.446 --> 00:22:06.289
Ran Reichman: Like AI has its issues and its challenges. And it's hallucination. But when when you're asking like, basic questions of like, this is what I'm doing. This is what I'm trying to achieve. This is how I know how to do it in Python C, plus. Plus. What have you?

127
00:22:06.400 --> 00:22:19.699
Ran Reichman: Then I found that that to be very effective, and I I do very much believe that it's easier to learn a programming language by doing so. Like reading a book is great, but you only internalize things when you

128
00:22:19.840 --> 00:22:29.000
Ran Reichman: and when you try to build, and the compiler is mad at you, and it's like, no, you can't do that. You can't do that either. You can't do that either, and rust in that sense is very annoying. It's a it's a strict boss

129
00:22:29.641 --> 00:22:34.760
Ran Reichman: but the result is good. The result is one in which you can really rely on the

130
00:22:35.310 --> 00:22:39.499
Ran Reichman: and the accurate accuracy, and

131
00:22:40.990 --> 00:22:45.530
Ran Reichman: the reliability of the result. I think that's kind of a Us.

132
00:22:46.830 --> 00:22:59.520
Ran Reichman: a switch that people need to do when they're moving to rust, and when they're trying out rust that, like the compilation, is hard, like getting things to compile is oftentimes the hard part. Then, when things run. They usually run accurately

133
00:23:00.252 --> 00:23:07.230
Ran Reichman: so that's that. That's what we tell people who are who are starting to learn. We're trying things out.

134
00:23:08.220 --> 00:23:08.970
Ran Reichman: Yep.

135
00:23:10.130 --> 00:23:29.490
Gabor Szabo: Yeah, so I do a lot of training. And my experience is that it's nice to explain things to people. But they actually really learn when they do the practice and when they get feedback for whatever they did, and well in rust the compiler gives a lot of feedback, so I have less work. But

136
00:23:29.660 --> 00:23:39.220
Gabor Szabo: still the human human feedback is nice when when you might already solve the problem. But there are better ways to to write in in the given language.

137
00:23:40.770 --> 00:23:45.740
Ran Reichman: Yeah, yeah, 100%. I also say, I think that once you're in a rush scheme

138
00:23:45.910 --> 00:23:56.859
Ran Reichman: which not everyone is in. And it's kind of a priv privilege. Then more junior members just ask a lot of questions, and our senior members like to like to share and mentor.

139
00:23:56.970 --> 00:23:58.060
Ran Reichman: So

140
00:23:58.511 --> 00:24:14.310
Ran Reichman: sometimes you're like, Well, why is this happening? And people just explain like, this is the philosophy behind how this works. So once you have some sort of a a core of a team that's able to provide these insights. Then continuously growing, it

141
00:24:14.780 --> 00:24:16.490
Ran Reichman: becomes easier.

142
00:24:16.490 --> 00:24:29.740
Gabor Szabo: Yeah. A nice part about this actually is that it's not not just not just the juniors who learn from such interaction, but also the more experienced people who encounter interesting

143
00:24:30.200 --> 00:24:45.140
Gabor Szabo: code snippets from the juniors. And then they have to have to think about why these people, why, that person wrote that kind of code. And and what's the story behind it? So I think it's it's in bi directional learning. There.

144
00:24:45.400 --> 00:24:51.879
Ran Reichman: Yeah, I think I think that's true about mentoring in general, like as a mentor, as a professor, as a teacher, you, you learn a lot from your students.

145
00:24:53.490 --> 00:24:55.330
Ran Reichman: I feel the same way as a manager.

146
00:24:56.880 --> 00:25:22.669
Gabor Szabo: Okay. So there is a question here which is actually very close to what I wanted to ask. But let's ask this 1 first.st So do you have any suggestion for someone who would like to find work in rust? Well, see, and other things, and is comfortable with them, but only has web, dev experience, career, wise career, wise. The transition seems borderline impossible.

147
00:25:24.830 --> 00:25:46.349
Ran Reichman: Yeah. So I I understand the sentiment of the comment. It's it's definitely not easy. Because the companies look at your background. And if your background doesn't seem related to what you're looking for I mean the the inconvenient truth is that it's just hard to break through. It's hard to say, like, Oh, yeah, this is this is what I previously did. But what now? I want to do something new.

148
00:25:46.830 --> 00:25:50.715
Ran Reichman: and I I think the

149
00:25:52.780 --> 00:25:59.919
Ran Reichman: they're the main solution to this, and it's not a convenient one, because it requires work. But that's why it's powerful

150
00:25:59.950 --> 00:26:27.659
Ran Reichman: is open source is contributions. Because, like, we have people that we interviewed for the company like we pursued them like we reached out to them that we never met. We don't know what else they did in the world. We just saw the kinds of contributions that they were able to make to interesting projects and say, this is the kind of person we want in the team, and we didn't know that they were like they could be Web Devs. They could be python they could be. I don't know what I honestly don't know what they're doing in their day job.

151
00:26:27.670 --> 00:26:47.530
Ran Reichman: but if you see people if they have like, it's kind of in that sense like art. So if someone shows you their art and the art is beautiful, then you say, like, Okay, I don't really care about the rest. I would like that someone on the team. And it's not easy, because you have to invest. And you have to like, learn and learn these environments, etc. You gain a lot from that. But you have to invest a lot of that.

152
00:26:47.550 --> 00:26:52.829
Ran Reichman: So yeah, not not not the the fast route or the easy route, but one that I think is effective.

153
00:26:53.780 --> 00:27:22.260
Gabor Szabo: Yeah. So so I'm afraid that you already answered my next question. But maybe you can elaborate a little bit more. So let I assume that as a startup you're growing so you're looking for more people. So if I wanted to join your company in the next months, too or so, so I don't have too much time. But let's let's assume I have some background that is relevant to you. What would you suggest to make

154
00:27:22.580 --> 00:27:24.339
Gabor Szabo: to make it

155
00:27:24.570 --> 00:27:35.679
Gabor Szabo: easier for me to to. Yeah, to make it easier for me to to find you or or you to find me, or or okay, whatever right? I think you understand the question.

156
00:27:36.700 --> 00:27:37.200
Gabor Szabo: I hope.

157
00:27:37.200 --> 00:27:39.179
Ran Reichman: Yeah. So 1st of all, the the.

158
00:27:39.180 --> 00:27:48.569
Gabor Szabo: To improve my chances. Actually, I think that should be the question. So what should I do to improve my chances that you actually find me, or employ me, and so on.

159
00:27:49.140 --> 00:28:00.230
Ran Reichman: Yeah, so it's a good. It's a great question. The easiest answer is, 1st of all to apply. that's that's the the 1st step, and we do look at applications. That that we get.

160
00:28:01.072 --> 00:28:04.150
Ran Reichman: I think. Given a certain application.

161
00:28:05.470 --> 00:28:09.219
Ran Reichman: You have to like our interview process.

162
00:28:09.682 --> 00:28:14.819
Ran Reichman: We. We work very hard for our interview process to be very similar to the work itself.

163
00:28:15.491 --> 00:28:36.339
Ran Reichman: I found that the at the like big tech, and most of the interviews are kind of IQ test. And it's not a bad thing like these companies like they they optimize for getting just very smart people, and obviously they're doing very well for with that. But for us, we want people who like during the interview really understand how their day to day is gonna look like.

164
00:28:36.440 --> 00:28:43.669
Ran Reichman: And they like it. They they see like they they do the interview task that we give them. And they say, Okay, this. These are the kinds of things that I want to do in the future.

165
00:28:43.840 --> 00:28:47.260
Ran Reichman: And sometimes people look at them and and they don't

166
00:28:47.370 --> 00:29:05.450
Ran Reichman: like they? They realize that they don't want to work on. I don't know low level optimization database. So what I would encourage like our website, Clarionio, you can see the kinds of technologies that we're using, which is like polars, data, fusion, arrow stuff like that.

167
00:29:05.580 --> 00:29:14.820
Ran Reichman: So if you engage with these projects and look at them and think how to use them. You don't even have to contribute to the open source. You can think like, Okay, how do? How do I use.

168
00:29:14.990 --> 00:29:16.870
Ran Reichman: and something like polars

169
00:29:18.430 --> 00:29:38.030
Ran Reichman: in data processing like, How how does that work? And if you're able to answer that question and share your experience and stuff like that. Then you're already ahead of, like, 90% of the people who who have no context don't know how any of these things work. And so oftentimes putting in the work puts you pretty much ahead.

170
00:29:39.170 --> 00:29:39.889
Gabor Szabo: Right?

171
00:29:40.980 --> 00:29:53.530
Gabor Szabo: okay. The next sort of the question of the expansion of this question would be, which is slightly, I feel strange with this question, because it sort of assumes a little bit of

172
00:29:53.891 --> 00:30:16.080
Gabor Szabo: moving ahead or or back in in time. So a little bit of time travel. But what would you suggest to someone if if the person has one or 2 years? So decides that, okay, I have no clue about rust and all of all of this stuff, but I know your company, and I really would like to work at at your company.

173
00:30:16.080 --> 00:30:26.009
Gabor Szabo: So I have. Now I'm going to start to do, learn things and do things. So in a year or 2 I have a much better chances to

174
00:30:26.710 --> 00:30:31.020
Gabor Szabo: get hired by Florian. Obviously you don't know what Florian will look like in 2 years.

175
00:30:31.020 --> 00:30:31.989
Gabor Szabo: Yeah, of course.

176
00:30:31.990 --> 00:30:41.889
Gabor Szabo: So that's why it's a little bit tricky question. On the other hand, you could think about it. What would you have suggested 2 years ago, or a year ago, for someone like this

177
00:30:42.040 --> 00:30:43.340
Gabor Szabo: to be hired now.

178
00:30:43.340 --> 00:30:50.260
Ran Reichman: Yeah, no, I I think that's a really interesting question to me. The answer is, if if you're looking at like the medium and long term.

179
00:30:50.520 --> 00:30:55.980
Ran Reichman: I think the my advice would just to be more ambitious. Like, if if you're saying like I

180
00:30:56.270 --> 00:31:08.050
Ran Reichman: for the next year or the next 2 years. I'm gonna work on this technology. Then you can really achieve a whole lot in that time, even as a side project. So I I would try to do something that excites me.

181
00:31:08.090 --> 00:31:28.381
Ran Reichman: Like 99% of the of the things that people work on. They just churn, they start working on it, and they they enjoy it for a while, and then they stop working for some reason. And that that's true for myself also, like that just happens like you start working on something, and then life happens, and you have a lot of other things to do. But if you're committed for a long time frame. Then you can really achieve a lot.

182
00:31:28.790 --> 00:31:36.020
Ran Reichman: and I would. I would just try to choose the something ambitious like, try to understand something, try to change something, try to do something like that.

183
00:31:41.010 --> 00:31:49.180
Gabor Szabo: Okay, yeah, I, I thought about some specific types of projects to work on or.

184
00:31:49.790 --> 00:32:01.669
Ran Reichman: So some people like, if we talk about specific projects like some people work on, for instance, the game engines and rust. If you're interested in gaming. There's this kind of joke that everybody wants to build a game engine in rust, but nobody wants to build a game.

185
00:32:01.900 --> 00:32:27.190
Ran Reichman: and so I don't think that's necessarily bad, but like these are the kinds of things like, if you're excited about gaming, try to build a game engine if you're excited about AI. Then try to see like what's relevant in AI with rust. I don't know if there's that much, but if you're excited about data, then you can look at these big data projects. There are quite a few at this point. Big rust projects online.

186
00:32:27.380 --> 00:32:50.029
Ran Reichman: And then I, I look at like the top 1520 rust projects on Github and say, like, what excites me here? What's interesting like, what would I want to build? What would I want to recreate. What would I want to benchmark or test, or stuff like that? And that also brings you connections like, if you, if you one of the things that I learned throughout my career is that when you reach out to successful people

187
00:32:50.614 --> 00:32:55.280
Ran Reichman: that are technical and you engage with what they're working on.

188
00:32:55.659 --> 00:33:03.900
Ran Reichman: Like, the response rate is huge. It's like, I don't know, maybe 90% or something like people people like that. People are excited about their technology and want to build on it.

189
00:33:04.420 --> 00:33:09.119
Ran Reichman: so finding those things that you actually want to engage with and actually engaging

190
00:33:09.685 --> 00:33:12.880
Ran Reichman: I I think is a is a great investment.

191
00:33:18.840 --> 00:33:21.160
Gabor Szabo: Yeah, that's that's an excellent point.

192
00:33:21.716 --> 00:33:24.769
Gabor Szabo: I I tried to Co, to

193
00:33:25.130 --> 00:33:29.710
Gabor Szabo: to recommend people all the time to to invest in open source

194
00:33:29.900 --> 00:33:43.889
Gabor Szabo: doing things in open source. But it's always a people get excited. But they they most of the people actually don't follow through this, and they do a little bit, and and then they abandon it the whole thing.

195
00:33:44.210 --> 00:33:50.790
Gabor Szabo: So those who do continue I guess they will stand out.

196
00:33:50.790 --> 00:34:02.239
Ran Reichman: Yeah, yeah, I think I think that's exactly right. I think because of the churn, and because most people don't go deep. So if you're able to go deep and and commit to something. Then again, you're you're ahead of, like 90% of people already.

197
00:34:07.240 --> 00:34:12.890
Gabor Szabo: So there's a comment here that for those a good start is trying to help in an existing open source project.

198
00:34:13.030 --> 00:34:14.480
Gabor Szabo: What do you think about that

199
00:34:16.440 --> 00:34:20.340
Gabor Szabo: open source, project or or create your new, create your own.

200
00:34:20.340 --> 00:34:36.189
Ran Reichman: I I wouldn't try to create my own as a start. But I think, even before helping even just using an open source project like. There's a huge one of the things that we're seeing at the flare on is that there's a huge variability

201
00:34:36.420 --> 00:34:39.879
Ran Reichman: in terms of what the open source project is able to do

202
00:34:40.510 --> 00:34:43.570
Ran Reichman: like some projects, especially in their core.

203
00:34:43.860 --> 00:34:55.739
Ran Reichman: They're super robust, they work, they, they, you just plug and play, and others are like they can be hard to use. They have a rough edges of various sorts. And being a user and understanding like.

204
00:34:56.170 --> 00:35:23.629
Ran Reichman: where does this technology really work. Well, where does it not work well, like? Why doesn't it work? Well? These are the kinds of things that you don't even have to start by like. Sometimes people are intimidated by contributing, because, like maybe people won't like what I contributed, or maybe I'll have a back and forth with some maintainer for days, and I won't get anything through. But just like starting with using and gaining insights and opening issues and saying, like, Hey, I saw this, is this. Okay, is this expected?

205
00:35:25.040 --> 00:35:36.049
Ran Reichman: again, these are the kinds of things that people just don't do. It sounds simple. It is simple, like it's not hard to take it to choose a project and and try to try to engage with it. But most people just don't do that.

206
00:35:37.400 --> 00:35:42.869
Gabor Szabo: Yeah. So a lot of a lot of this work is not very difficult, but time consuming.

207
00:35:43.170 --> 00:36:01.749
Ran Reichman: It's time consuming. It's an investment. There's no going around that. That's why it's valuable in a sense, like in the in the cryptocurrency world. There's a concept of a proof of work that you like show that you invested the time in something. I think, that's applicable to many, many aspects of life. So if you, if you invest that work, yeah, I think it has value.

208
00:36:03.060 --> 00:36:10.739
Gabor Szabo: Okay, so how do you evaluate the the decision that that you used? Rust?

209
00:36:11.040 --> 00:36:14.909
Gabor Szabo: What did you that you that you use your rust? Yeah. So.

210
00:36:15.610 --> 00:36:34.949
Ran Reichman: Yeah, it's a great question. It's a piercing question, I'd say, because it's it's hard to evaluate, right? You made such a big decision. And now you have to decide like, see if it's a good decision. I haven't regretted it yet, like I haven't seen a point where it's like, Oh, man, why did we do we use this technology? That was a huge mistake. We also have the very significant benefit of starting from scratch.

211
00:36:35.515 --> 00:36:42.215
Ran Reichman: So when you start from scratch. You don't have any legacy and before before the conversation we talked about

212
00:36:43.127 --> 00:37:00.820
Ran Reichman: like a migrating code to rust, so we don't really do that like we, we rewrite some things. And when you rewrite, then you're just like starting from scratch. So a lot of the pains of moving from one technology to another. We just skipped. It's much easier to start from scratch

213
00:37:01.857 --> 00:37:06.942
Ran Reichman: so I think I think it's too early to say for now like no regrets at this point.

214
00:37:08.110 --> 00:37:09.156
Ran Reichman: We'll see

215
00:37:10.100 --> 00:37:32.040
Gabor Szabo: Okay, what are the parts of of rust that or creates, or like the whole package? That you find exceptional good for for you or in general. And what? What are the areas where the rust world could improve? Where? It's something that hurts you.

216
00:37:33.280 --> 00:37:37.579
Ran Reichman: Yeah. So I'm I'll start with a positive I think

217
00:37:37.980 --> 00:37:54.099
Ran Reichman: rust in the past, say, 5 years has really matured in a in an impressive way. Like, you see, we engage with customers using all sorts of technologies, whether it's data, lake technologies or data processing or storage and stuff like that.

218
00:37:54.320 --> 00:37:55.320
Ran Reichman: And

219
00:37:56.448 --> 00:38:10.391
Ran Reichman: there's usually a crate for that, like the echoing the apples. There's an app for that. So usually there's a crate for that. Usually someone builds something pretty good that's able to use this technology, and and help

220
00:38:11.480 --> 00:38:13.060
Ran Reichman: help work with it.

221
00:38:15.330 --> 00:38:36.489
Ran Reichman: so that's something that I think is very good. I also think that the reliability is real like we. We did something that where we were integrating like Scala code and rust code. And suddenly there was like a memory corruption. And someone on our team was like, Oh, a memory corruption. I haven't seen one of those in years, so that that was like a moment of, Okay, yeah, this this rust thing is working.

222
00:38:36.650 --> 00:38:45.180
Ran Reichman: I'd say, the the 2 negatives that I can think of is one a build time

223
00:38:45.744 --> 00:39:05.519
Ran Reichman: there really is. It's a pain like building rust pretty quickly becomes pretty time consuming. And it's hard to optimize like when I when I worked in c plus plus, there were all sorts of tricks to to make the build run faster, even though I also encountered many situations where we had a very long build.

224
00:39:05.883 --> 00:39:23.279
Ran Reichman: But yeah, that that's that's a pain. And another one is, I'd say, the flip side of the there's a crate for that. But there's a lot of the support is that there's a lot of vaporware. I'd say there are like crates that you use them. And you're like, okay, this crate. It doesn't really work. And that's fine. I mean, you know, nobody like

225
00:39:23.620 --> 00:39:42.310
Ran Reichman: nobody's selling you that crate. It's just something that someone published online. But you realize that you have to. You have to prune it. You have to make decisions. You have to make sure that this this relates to your previous about, how do you choose the right crate? So you have to choose. You have to make sure that you're using something that you can believe in.

226
00:39:44.770 --> 00:40:12.500
Gabor Szabo: For a second, returning to the earlier conversation or a part of the conversation. We have another question here follow up to finding work as a web dev any suggestions for types of repos, types of projects that would be appreciated by employees employers if you contribute to. In your opinion, of course, I am contributing to programming languages. For example, I guess the

227
00:40:12.520 --> 00:40:17.530
Gabor Szabo: person means compilers. What do you mean? Programming languages.

228
00:40:23.480 --> 00:40:24.930
Ran Reichman: Yeah. So I think

229
00:40:25.750 --> 00:40:35.789
Ran Reichman: I think that's definitely good, like working on these kinds of technologies. It's not easy, something that like that that does differentiate you. But I will say

230
00:40:36.060 --> 00:40:43.710
Ran Reichman: that it's valuable to be able to like. I, currently, this is changing. But currently

231
00:40:44.217 --> 00:40:57.030
Ran Reichman: there's still a limited number of companies that are hiring for rust roles, and the the number is growing. But it's different. If you're coming to a company like aws, which does a lot of rust.

232
00:40:57.340 --> 00:41:25.589
Ran Reichman: or a company like Clarion that does a lot of us, just because, like, there are different use cases. So at Aws or Google, or something like that. You'll spend a lot of a lot more time on concurrency, because that's just like there's a lot of input, output happening inside their systems and a lot of micro services. So if I were interested in in a role in one of those companies, I'd probably try to understand like they have blog posts. They write interesting stuff about their challenges. I tried to understand a bit their stack

233
00:41:26.150 --> 00:41:27.080
Ran Reichman: and

234
00:41:27.591 --> 00:41:36.040
Ran Reichman: gain relevant experience. There, if we're talking about a company like Ferryon then, it's it's a different stack. So there are many islands in the rust world.

235
00:41:36.180 --> 00:41:38.988
Ran Reichman: and I I try to

236
00:41:39.890 --> 00:41:55.309
Ran Reichman: like, I think compilers are, are a great great place to start and if you find a a company building a compiler in rust, and obviously that's that's fantastic. But if you're looking at a company that's that's not doing exactly that. Then I would try getting acquainted with the with the tech stack that they're using.

237
00:41:56.210 --> 00:42:00.079
Gabor Szabo: Actually. For to this point I sort of

238
00:42:00.330 --> 00:42:17.380
Gabor Szabo: partially dreaming of about companies that would say to their to their candidates that, Okay, here are a couple of issues open in an open project, and your homework or home assignment for

239
00:42:17.380 --> 00:42:36.719
Gabor Szabo: for the interview process is to deal with those issues, and maybe not even directly related to the company. So to see how the person interacts with open source projects and and without and contributing to the world. Sort of

240
00:42:37.365 --> 00:42:37.660
Gabor Szabo: yeah.

241
00:42:37.660 --> 00:42:58.200
Ran Reichman: Yeah, I totally agree. I'd also add that the big advantage of engaging with projects that are related to the company is that you get a better understanding if you want to work there. This touches on what I mentioned earlier, like sometimes you, you might think that you want to work with. But then you look at like the the stack, what they're building, and you say, that that's not really my thing.

242
00:42:58.270 --> 00:43:09.230
Ran Reichman: And the the best time I. This is what I tell the candidates as well. The best time to understand that is, there is like, before you join the company. And cause. Yeah, it's really important to write that to find the right fit.

243
00:43:09.810 --> 00:43:28.770
Gabor Szabo: So here we have a very technical question or more technical than what we had. So far, high performance data data processing is is what you are. That's, I think the statement fly around inside. I don't know where this sentence comes from, so could you share any valuable practical examples

244
00:43:28.770 --> 00:43:44.370
Gabor Szabo: or tricks, how you debug distributed and high performance use cases. Do you use Tokyo for development, or just something like wrappers around arrow data, fusion, and so on. Any other valuable recommendations.

245
00:43:46.010 --> 00:43:47.834
Ran Reichman: Yeah, it's a big question.

246
00:43:49.980 --> 00:43:56.000
Ran Reichman: 1st of all, I mentioned this. But I'll mention it again, because it's just so important, like tests

247
00:43:56.250 --> 00:44:00.129
Ran Reichman: across the stack matter a whole lot. And

248
00:44:00.850 --> 00:44:07.189
Ran Reichman: this varies across companies. Because if you're building a like, I've I have some experience like with like a front end development.

249
00:44:07.370 --> 00:44:27.850
Ran Reichman: and there I don't know if I didn't invest a huge amount of my time in tests. You mostly want to iterate and see that it works and find bugs and solve them. When you're building a mission critical software, which is what we're doing. You really want things to be exact, like some startups can move fast and break things at play around. We don't move. We try to move as fast as possible, but

250
00:44:28.350 --> 00:44:51.179
Ran Reichman: it really needs to be stable. So this means integration tests, system tests, unit tests. These are the kinds of things that that are very useful to debug even distributed and high performance systems, because even a distributed system eventually includes various components and interactions between those components. And if you test those well.

251
00:44:51.380 --> 00:44:54.490
Ran Reichman: then you reduce the number of cases that said

252
00:44:54.620 --> 00:45:01.289
Ran Reichman: our team finds itself running something for 3 h on 2 TB of data in order to find all sorts of edge cases.

253
00:45:01.630 --> 00:45:02.950
Ran Reichman: And there.

254
00:45:03.110 --> 00:45:04.190
Ran Reichman: I think.

255
00:45:05.150 --> 00:45:12.210
Ran Reichman: 1st of all, it's not something that we've solved, I'd say, and we're working hard on building tooling.

256
00:45:12.650 --> 00:45:37.429
Ran Reichman: That extracts data from the data processing pipeline and visualizes it in a way that's easy to understand. So that you understand, like, how much memory you're using, how much CPU like, why is it happening? How much disk? And interestingly, this is something that is also gonna be valuable for our customers. So like, we make data processing much more efficient, much faster, much more performant.

257
00:45:38.376 --> 00:45:49.580
Ran Reichman: But by doing that we also wanna visualize this to our customers. So in a sense, we wanna visualize it to ourselves to understand what the bottlenecks are, and then visualize it to our customers. This is a hard thing to do.

258
00:45:51.640 --> 00:46:01.180
Ran Reichman: so I I would say, we we've solved it. But I'd say, adding a lot of debugging. We use also a a Tracy

259
00:46:01.360 --> 00:46:03.801
Ran Reichman: actually, as a tool to

260
00:46:04.570 --> 00:46:11.639
Ran Reichman: defined performance issues, and it's quite effective so when you're able to attach things to your code.

261
00:46:11.800 --> 00:46:14.280
Ran Reichman: and they do all sorts of

262
00:46:16.100 --> 00:46:24.969
Ran Reichman: performance, monitoring of various sorts and benchmarking we found that to be valuable, especially when when debugging performance issues.

263
00:46:27.390 --> 00:46:47.120
Gabor Szabo: So the next question that coming from the audience is actually the next question I wanted to ask. So I'm going to ask in the way I thought about sort of is that to try to understand what is your experience? Using AI tools to write rust code, which

264
00:46:47.770 --> 00:46:51.410
Gabor Szabo: tool you might be using, which model.

265
00:46:52.960 --> 00:46:59.420
Ran Reichman: Yeah. So 1st of all, let's say, we're model agnostic at Flareon I found that the AI

266
00:46:59.910 --> 00:47:19.169
Ran Reichman: it's similar to ides and even operating systems. People have their preferences. And as a as a manager, I I'm not gonna push someone to use a certain flavor of AI, because because I have a preference for it. Like, I trust the the team members to

267
00:47:19.650 --> 00:47:26.699
Ran Reichman: try to be as effective and efficient as possible. And we bring people that like are trustworthy in that regard.

268
00:47:27.287 --> 00:47:42.000
Ran Reichman: I've had good experience with Cloud. I think it's it's good for software development. Some team members really like a chat gpt some like deep seek they found, especially our one to be effective.

269
00:47:43.830 --> 00:47:45.410
Ran Reichman: I'll say that the things that

270
00:47:45.540 --> 00:47:50.200
Ran Reichman: the AI is best at in terms of our development is,

271
00:47:52.070 --> 00:48:06.599
Ran Reichman: 1st of all, needle in a haystack issues where you have like this huge error. And you say, like, Hey, AI! Like, what? What do you think is going on here? And then it's like, Oh, you! You wrote this incorrectly, and you should fix it. And usually, usually that's just right. Like you take a blob of code. You take the error

272
00:48:07.190 --> 00:48:11.500
Ran Reichman: 9 out of 10 times. It says like this is probably there. And and it's right.

273
00:48:13.520 --> 00:48:42.860
Ran Reichman: basic functionality. Is good. But usually that doesn't save that much money. I think the metric Sam Altman has said, I said this, and I 100% agree. I think the metric of like percent of code written by AI is is what in startup world they call a vanity metric which is like it's. It sounds interesting, but it doesn't really tell you how much how much you're using. Because, like autocomplete, like, how much of my text that I write in emails is written by autocomplete? Probably a lot. But like, how significant is that to my

274
00:48:43.170 --> 00:48:45.649
Ran Reichman: in my process, and not too much.

275
00:48:48.040 --> 00:48:53.059
Ran Reichman: I will say that where AI isn't as good is when things get complex.

276
00:48:53.310 --> 00:48:55.449
Ran Reichman: and that happens a lot for us.

277
00:48:55.880 --> 00:49:02.270
Ran Reichman: If you're trying to write something very performant. If you're trying to write something that's distributed and

278
00:49:02.390 --> 00:49:27.841
Ran Reichman: different things impact each other, whether it's memory. CPU disk, usage networking. Once many parameters get into the equation. Usually the AI focuses on some local minimum and not on like the whole system, and we found it to be very difficult to get it to solve. Those kinds of problems which is kind of fun. Because, like you want to be working on things that are

279
00:49:29.031 --> 00:49:34.358
Ran Reichman: not like the AI has trouble with, because otherwise, I mean, I don't know. You can just get the AI to do them. So

280
00:49:37.560 --> 00:50:06.820
Gabor Szabo: Let's switch a little bit to a slightly different audience. The audience, that part of the people who are here. I guess they're part of that group. If you're talking to other startup founders, or Ctos or so manager level people, or or co-founders, what would you recommend them? How to decide whether to use rust

281
00:50:06.870 --> 00:50:08.860
Gabor Szabo: or or not to use rust.

282
00:50:11.550 --> 00:50:12.220
Ran Reichman: Yeah, so.

283
00:50:12.220 --> 00:50:15.819
Gabor Szabo: How to, how should they evaluate it, or.

284
00:50:17.270 --> 00:50:25.230
Ran Reichman: So the place I'd start is, you want to have some familiarity with it. You like you want to champion. I think that's true with any new technology.

285
00:50:26.069 --> 00:50:29.501
Ran Reichman: You don't wanna like if you have.

286
00:50:31.026 --> 00:50:40.180
Ran Reichman: an engineer with the 3 years of experience, and they say, like, hey? Rust is exciting. I want to bring this to the company. You're unlikely to be successful by doing that.

287
00:50:40.560 --> 00:50:42.820
Ran Reichman: but if you have someone who can.

288
00:50:43.080 --> 00:50:50.090
Ran Reichman: who can experiment. Who can mentor who can make decisions on the right projects for this technology?

289
00:50:50.879 --> 00:51:01.740
Ran Reichman: Then I think you're much more likely to be successful. And I mentioned this earlier, but I'll mention it again, because I really think it's it's crucial, is choosing the right projects

290
00:51:02.040 --> 00:51:09.050
Ran Reichman: is super super important. So if you have like a contained project that you know what you want to achieve there, and you're starting from scratch.

291
00:51:09.230 --> 00:51:17.280
Ran Reichman: and you want to do that with rust, and you have the the know how, then I don't see why it shouldn't succeed like it's a very much set up for success.

292
00:51:17.400 --> 00:51:20.084
Ran Reichman: But the more the more the project is

293
00:51:20.720 --> 00:51:28.519
Ran Reichman: distanced from that situation, I think the less likely it is that that, it'll it'll be a success.

294
00:51:32.260 --> 00:51:33.280
Gabor Szabo: Okay.

295
00:51:34.410 --> 00:51:44.839
Gabor Szabo: I am, I think, mostly at the end of of my questions. I don't know if the audience have more questions. The audience who is here, or if you have the

296
00:51:45.320 --> 00:52:02.240
Gabor Szabo: subjects that you would like to bring up, that I I didn't ask because I didn't think about that important, and you think it would be important to for people to know about the company, about the use of rust, how to apply for a job at you to your company.

297
00:52:03.370 --> 00:52:06.200
Ran Reichman: Yeah. So how how to apply?

298
00:52:06.320 --> 00:52:19.700
Ran Reichman: It's pretty easy, like we have linkedin posts at our company, so you can look at Linkedin, at at flare on and see probably the most relevant here at the low level developer, which is how we call our rust position.

299
00:52:21.420 --> 00:52:27.170
Ran Reichman: Yeah, I think there's a lot of exciting momentum in the rust world. Lots of talent.

300
00:52:27.370 --> 00:52:30.350
Ran Reichman: I think the fact that big tech is investing in rust

301
00:52:30.570 --> 00:52:36.710
Ran Reichman: is really significant. Because, it just creates

302
00:52:37.390 --> 00:52:48.250
Ran Reichman: a whole set of people who are being trained and using the rust in production, and they also contributing to open source projects. So I think that's great.

303
00:52:48.870 --> 00:52:55.240
Ran Reichman: And yeah, lots of exciting stuff. Here, we're we're in terms of Larry on. So we're

304
00:52:56.010 --> 00:53:14.129
Ran Reichman: like, the world of data is so vast. And there's a there's so much to do here, and so many engines and technologies that can be applied when a company wants to do data processing very efficiently. So we're obviously excited about that future. And we think about rost has a has an important role. There.

305
00:53:16.500 --> 00:53:20.470
Gabor Szabo: Actually, I just thought about a new question, what are the main

306
00:53:20.890 --> 00:53:33.290
Gabor Szabo: problems that Florian is is facing that your engineers will need to solve in the foreseeable future that the new people who are joining the team might need to work on.

307
00:53:34.600 --> 00:53:43.313
Ran Reichman: Yeah. So I mean, I'd say, our engineers are pretty fortunate to have pretty tough challenges. so

308
00:53:44.160 --> 00:53:51.380
Ran Reichman: in in many senses. We're building a a distributed database for data processing, and that means

309
00:53:51.710 --> 00:53:55.649
Ran Reichman: handling issues of a very significant scale

310
00:53:55.900 --> 00:54:12.259
Ran Reichman: of a disk usage of memory management. these are the kinds of things that we're we're excited about like, how much, how much CPU is this using? How much memory is this? Designed correctly. Should we redesign this part of code of the code, should we?

311
00:54:14.072 --> 00:54:19.147
Ran Reichman: sometimes we, we insert many fast paths into our code, so that,

312
00:54:20.238 --> 00:54:25.790
Ran Reichman: when a customer is using is using it naively, and we can get a big performance boost

313
00:54:26.350 --> 00:54:39.550
Ran Reichman: so lots of challenges in that area, it's like CPU memory large scale of data and and performance. Those are the kinds of things that we encounter basically every day.

314
00:54:39.790 --> 00:54:49.780
Ran Reichman: And we also encounter a lot of like new and interesting technologies because our customers have a lot of new and interesting technologies that we we have to learn about them support them, etcetera.

315
00:54:50.850 --> 00:54:55.979
Gabor Szabo: Actually, who? Who are your customers, or what type of of companies are? Are your customers, or

316
00:54:56.750 --> 00:54:58.330
Gabor Szabo: could be your customers.

317
00:54:58.900 --> 00:55:04.656
Ran Reichman: Yeah. So our our customers are mostly the enterprise. meaning the companies that

318
00:55:05.478 --> 00:55:13.230
Ran Reichman: that are very significant data infrastructure. Because they're the ones that carry them care the most about performance. So if the company has a

319
00:55:13.360 --> 00:55:19.639
Ran Reichman: hundreds or thousands of data processing jobs, whether it's a spark array, or hadoop, or trino.

320
00:55:19.830 --> 00:55:27.419
Ran Reichman: These are the kinds of companies that really care about performance that really have pains in terms of managing their spark and their data infrastructure in general.

321
00:55:27.590 --> 00:55:32.639
Ran Reichman: And they're they're the ones that that are most excited about what Flarion brings to the table.

322
00:55:34.620 --> 00:55:41.769
Gabor Szabo: Okay, probably that should. That shouldn't have been the the last question. But so if somehow, this is how it turned out, anyway.

323
00:55:41.770 --> 00:55:42.170
Ran Reichman: Okay.

324
00:55:42.420 --> 00:55:50.540
Gabor Szabo: I don't see any more questions from the audience any any final words for this meeting podcast.

325
00:55:51.180 --> 00:55:58.989
Ran Reichman: No, I mean, I I appreciate this conversation and thanks everyone for joining and asking interesting questions. And yeah, thank you, Gabor, for organizing.

326
00:55:59.380 --> 00:56:22.929
Gabor Szabo: Thank you very much, and I really enjoyed it. And if you're watching this as a video, then please like the video. And if you listen to on, on the podcast, then in the show notes, there are going to be links where you can find the company and the other things that we discussed. And thank you, everyone for. Thank you, Ron, to

327
00:56:22.930 --> 00:56:30.949
Gabor Szabo: be here, and everyone else who joined us and asked questions that were enhanced. I think the the whole conversation.

328
00:56:31.000 --> 00:56:33.560
Gabor Szabo: and thank you everyone for listening.

329
00:56:33.960 --> 00:56:34.890
Gabor Szabo: Goodbye.

330
00:56:35.260 --> 00:56:35.960
Ran Reichman: Thank you.

