---
title: "Meet Elusion: New DataFrame Library powered by Rust 🦀 with Borivoj Grujicic"
timestamp: 2025-02-12T08:30:01
author: szabgab
published: true
show_related: true
description:
---

Attention Data Enthusiasts!🌟

Introducing [Elusion Library](https://github.com/databora/elusion), the brand-new data manipulation PowerHouse that combines the user-friendly syntax similar to Python libraries: Pandas and Polars, with blazing efficiency of Rust. Elusion seamlessly handles in-memory data formats like CSV, Parquet, JSON, and Delta Tables, ensuring versatile and high-performance data processing for any project.


![Borivoj Grujicic](images/borivoj-grujicic.jpeg)



{% youtube id="H-GhJIFreHY" file="2025-02-11-meet-elusion-new-dataFrame-library-powered-by-rust.mp4" %}


## Transcript

1
00:00:02.020 --> 00:00:26.390
Gabor Szabo: Hello, and welcome to the Codeme Event Channel. My name is Gabor Sabo. I usually teach rust and python at companies, and I also help introducing testing to companies and continuous integration and trying to help improving their development environment in in general. In addition, I think it's sharing

2
00:00:26.670 --> 00:00:52.369
Gabor Szabo: information, sharing knowledge is a very important thing in our in our community. And so we have this channel in the Youtube channel called me events. And these meetings that I'm organizing and inviting people from around the world to share their knowledge about rust. So if you are interested, 1st of all, please like the video and join and follow the Youtube channel. If you are watching this in Youtube.

3
00:00:52.960 --> 00:01:17.310
Gabor Szabo: also join or below the video, you're going to find a link to where you can join or group, and I'm really interested in in hearing suggestions for new other presentations about rust. And now I would like to thank you for for coming and and for joining us and giving this presentation. So

4
00:01:17.430 --> 00:01:24.410
Gabor Szabo: please, yeah, please introduce your yeah.

5
00:01:24.410 --> 00:01:25.539
Borivoj: Okay. Okay.

6
00:01:25.540 --> 00:01:26.889
Gabor Szabo: Go ahead, go ahead, sir. Just.

7
00:01:26.890 --> 00:01:30.230
Borivoj: It's hard to. It's hard to remember and to pronounce my name.

8
00:01:30.230 --> 00:01:40.030
Gabor Szabo: Yeah, yeah, yeah, I'm going to pronounce your name. I'm not sure how to pronounce it. My name so

9
00:01:40.030 --> 00:01:45.160
Gabor Szabo: difficult for pronunciation, for non-hungarian speakers. Okay, so.

10
00:01:45.160 --> 00:01:45.780
Borivoj: Yeah. So

11
00:01:46.530 --> 00:01:54.640
Borivoj: no problem. So my name is. But everybody calls me Bora, like in Ireland, Bora, Bora, this is the easiest way to remember.

12
00:01:55.420 --> 00:02:13.800
Borivoj: yeah, when I was younger, I was working on cruise ships. So all Americans, it's for them. It's very easy to remember. Bora Bora. Bora. Yeah. So Bora is simple to remember and and to pronounce, yeah. So I'm coming from Serbia. I'm 37 years old in a couple of days. Have 2 beautiful kids, 10 and 12.

13
00:02:14.280 --> 00:02:23.479
Borivoj: I'm a data engineering expert group currently working for on different projects, working mostly with with like American clients, healthcare companies.

14
00:02:23.620 --> 00:02:37.639
Borivoj: And I want to shout out to my colleagues that are working now without me, because usually now it's working time for us, you know. Second shift from 2 to 10. We usually work. So yeah.

15
00:02:37.740 --> 00:02:40.160
Borivoj: to cover this, you know, difference time difference.

16
00:02:40.840 --> 00:02:52.800
Borivoj: Anyways, yeah, I work as a data engineer, mostly working in fabric environment. Microsoft. And yeah, you're gonna hear a little bit more about this in in in the talk. But this is just a quick introduction of myself.

17
00:02:53.650 --> 00:02:55.550
Gabor Szabo: Go, go ahead, share your slide and.

18
00:02:55.810 --> 00:02:57.729
Borivoj: Yes. Okay, so let me just.

19
00:02:57.730 --> 00:03:17.079
Gabor Szabo: Just one more thing. Anyone in the group in in who is around now the live presentation feel free to ask questions. Obviously, you'll be in the video or or ask questions in the chat. If you prefer that way, and I'll when I see it, I'll read it, read out the questions.

20
00:03:17.480 --> 00:03:40.770
Borivoj: Okay, so you can, you can see the slide. Okay? So very good. So illusion, data frame library for everybody. So why, I'm usually saying like this, because my whole idea was to create something that will be easy to understand and easy to use by everybody in data world. So everybody who knows sequel. Everybody who knows data frame different data frame libraries.

21
00:03:41.090 --> 00:03:45.970
Borivoj: My goal was from the 1st day on when they come in to to understand. Okay.

22
00:03:46.130 --> 00:03:51.470
Borivoj: So as you can see below, I just want to mention one more time. Shout out to my guys at Expert group.

23
00:03:51.640 --> 00:04:04.309
Borivoj: I'm gonna see you later, after this talk. So what we're gonna talk about this today about a little bit about me. I'm gonna try to skip this fast. What is the motivation and what motivates me to to create this library? Why, another

24
00:04:04.590 --> 00:04:09.429
Borivoj: data frame library? Right? We. We have such a great libraries already in python world.

25
00:04:09.960 --> 00:04:27.190
Borivoj: And we're going to talk a little bit about data fusion, because we cannot talk about illusion without data fusion, because I'm heavily leaning on this SQL engine of data fusion. So I'm going to talk about illusion a little bit. Then we're gonna start covering a little bit of

26
00:04:27.680 --> 00:04:33.380
Borivoj: syntax, and why I decide to do and how I designed the syntax and why I did what I did.

27
00:04:33.740 --> 00:04:39.330
Borivoj: Okay, so the whole thing is, the whole idea is to promote the the library, not myself. So I gonna just

28
00:04:39.610 --> 00:04:40.709
Borivoj: quickly skip

29
00:04:41.020 --> 00:05:03.529
Borivoj: this part. So I finished music school. I think this is very important to have any sort of arts in your in your background, like painting any kind of arts you go through. I think it develops creativity. So which is very important. So I want to motivate anybody who is art who is in arts to join programming. I think you're going to do very well.

30
00:05:04.410 --> 00:05:08.160
Borivoj: So I finish sports gymnasium. I'm

31
00:05:08.350 --> 00:05:13.010
Borivoj: finish a master's in business economics. So I'm mostly heavily leaned

32
00:05:13.240 --> 00:05:22.919
Borivoj: most of my career in business only last 4 or 5 years. I'm I'm heavily coding and heavily programming before that I was mostly in in business realm.

33
00:05:23.700 --> 00:05:32.910
Borivoj: I started my career as a soldier in a special forces. Then I went on a cruise ships. This was my my younger days.

34
00:05:33.460 --> 00:05:41.249
Borivoj: Then I start working as a man in management, most in companies in like, let's say, top 5 to 10 people in in some big companies.

35
00:05:41.630 --> 00:05:56.030
Borivoj: And then I started working as a logistics and production manager. But I 1st time implemented back end in rust for the warehouse management system. That was. I was switching from Java to rust in that period. So it rust was a big

36
00:05:56.410 --> 00:06:00.709
Borivoj: a revelation for me in that period of time. So

37
00:06:01.030 --> 00:06:06.190
Borivoj: and then. Now, I work as a data engineer, lead data engineer, I mean in expert group

38
00:06:06.560 --> 00:06:08.949
Borivoj: here in Serbia. And, as I said, we

39
00:06:09.380 --> 00:06:11.920
Borivoj: bringing with a lot of lot of foreign clients.

40
00:06:12.250 --> 00:06:15.460
Borivoj: So yeah, a lot of work for for data

41
00:06:15.570 --> 00:06:20.229
Borivoj: people these days. Okay, so some programming languages that that I learned.

42
00:06:20.620 --> 00:06:31.889
Borivoj: And I'm also doing some courses on Udemy which you can also find data fusion like introduction for the beginners. Because I like data fusion a lot.

43
00:06:32.570 --> 00:06:38.420
Borivoj: And in December last year, I open, sourced this library illusion.

44
00:06:39.820 --> 00:06:42.979
Borivoj: Okay, so why would you use illusion?

45
00:06:43.450 --> 00:06:44.730
Borivoj: Okay. So

46
00:06:44.940 --> 00:07:02.750
Borivoj: one of the things that I was obsessing about when I was starting. This project is okay. So what I don't like about other libraries. It's this enforcement that you, forcing me to to use your library and changing your functions the way you create your process in the background.

47
00:07:02.850 --> 00:07:06.230
Borivoj: So this means that I need to remember for different libraries.

48
00:07:06.400 --> 00:07:22.749
Borivoj: I need to remember different syntax. Right? So one of the things that is very important for me was okay. How I can allow users to use their own logic to create their own queries and to get the same result. This was very, very important for me. And you're gonna see how I did this. So

49
00:07:23.220 --> 00:07:38.469
Borivoj: it has very simple data frame Api construct. If you know already. SQL, and any data frame library, it's gonna be very simple for you to understand. Okay, you're not gonna maybe know how to write immediately. But when you see it, you're gonna know what what actually, this thing does. Right?

50
00:07:39.660 --> 00:07:41.790
Borivoj: So if you want to have power of rust.

51
00:07:41.990 --> 00:07:46.160
Borivoj: of course, because it's 100% built with rust, but without the need to know. Rust

52
00:07:46.920 --> 00:07:50.120
Borivoj: thing, that the the let's say 3rd thing, that I was

53
00:07:50.310 --> 00:08:09.330
Borivoj: heavily, heavily thinking about, how can I create a library that any other problem language users can use? Of course, I'm mostly thinking about python users. Right? Because, let's say, Pandas, the most popular data frame library. It's such as, let's say, not simple, but it's

54
00:08:09.470 --> 00:08:12.150
Borivoj: it has easy constructs and easy.

55
00:08:12.250 --> 00:08:25.289
Borivoj: Let's say easy syntax to to remember without knowing python. You can actually work with pandas without knowing how to build some complex thing in python. Right? So

56
00:08:25.740 --> 00:08:30.610
Borivoj: this was like very important for me when I create this to, to, to

57
00:08:30.870 --> 00:08:33.350
Borivoj: to enable people that don't over us.

58
00:08:33.490 --> 00:08:36.629
Borivoj: and to still be able to use this use this library.

59
00:08:37.740 --> 00:08:46.959
Borivoj: Okay, we also have native pipeline scheduler. So I created the schedule with chrome scheduling. So you can automate your your jobs.

60
00:08:48.220 --> 00:08:57.439
Borivoj: And we have native reporting and dashboarding. And today I just this morning I published 2.7 point 0 version, which I added a grid to it

61
00:08:57.830 --> 00:09:14.699
Borivoj: before I had this interactive dashboards. Now, I added a grid tables which you can download to excel and Csv files as well. So this is also very important for business users to be able to export, to excel right. This is a whole whole circle, right

62
00:09:15.076 --> 00:09:29.299
Borivoj: export to give me ex export to excel button. That's very important for users. And one of the things because I saw there's a back end development in back in development. For me, it was very important to to have some sort of back end development capabilities.

63
00:09:29.790 --> 00:09:37.990
Borivoj: I consider working with the rest Apis and gathering data from Rest Api as part of the betting development. Okay, today. Data engineers work

64
00:09:38.090 --> 00:09:44.230
Borivoj: a lot of things. But my belief is that engineer do that, Etls. But

65
00:09:44.910 --> 00:09:56.389
Borivoj: without rest, Api handling mostly backend developers should do this. I believe so, but I will also want to add the rest. Api handling in the sense that we can take

66
00:09:56.680 --> 00:10:06.040
Borivoj: data from Api. Store it in Json, then use it to you. Can you can forward it to the data frame and work with it later on.

67
00:10:06.540 --> 00:10:09.048
Borivoj: Okay, so today, we have

68
00:10:09.850 --> 00:10:12.570
Borivoj: over 10,000 downloads in just one month.

69
00:10:12.940 --> 00:10:20.710
Borivoj: So one month and 20 days, let's say, yeah. So I'm very happy with this. And I'm happy that people are using this, I get some nice feedbacks.

70
00:10:21.130 --> 00:10:31.800
Borivoj: I had also nice, some nice suggestions from the community. So thank you for this. I had like much more experience. People working with us to give me some very nice, very good.

71
00:10:32.900 --> 00:10:36.490
Borivoj: very good explanations of things. How things should should break. Right?

72
00:10:36.630 --> 00:10:38.399
Borivoj: We learn every day, of course.

73
00:10:38.760 --> 00:10:48.390
Borivoj: Okay, so motivation? Why, I start. Why start this project? And what still motivates me to to work on this? Okay? So I love data fusion.

74
00:10:48.870 --> 00:10:50.689
Borivoj: When I was working with data fuse.

75
00:10:51.040 --> 00:11:08.950
Borivoj: But the thing about data fusion, you have data frame Api, and you have this SQL Api, right? So working with SQL. Api. It's good, of course, native, it's good, but it doesn't give you any sort of error while you're writing. You can write the whole sequel like this.

76
00:11:09.330 --> 00:11:26.320
Borivoj: and and only when you execute, you get some errors, and you can find where is the error? Things like this here, when I separate the concerns for for the different functions you can pretty quickly figure out like, where is the error or find it? It's much easier to work with right, because it has this

77
00:11:26.560 --> 00:11:33.070
Borivoj: sort of has this SQL execution kind of a type, but still has this data frame Api function

78
00:11:33.190 --> 00:11:38.569
Borivoj: kind of a look. So it's it has like I think best from both

79
00:11:38.860 --> 00:11:46.820
Borivoj: Apis, let's say. But one of the things that is very important is that mixing different Apis from data fusion

80
00:11:47.470 --> 00:11:52.740
Borivoj: forced me to rewrite the whole library twice.

81
00:11:53.370 --> 00:12:06.080
Borivoj: So I will. I will explain why in just a bit. Okay. So another thing. So this new norm in today's data world is, let's build something with rust. And let's create python bindings. And

82
00:12:06.200 --> 00:12:10.459
Borivoj: let python users use it right? There's like a new norm today in data world.

83
00:12:10.660 --> 00:12:15.339
Borivoj: Also, data fusion is doing this, creating this wrapper for python.

84
00:12:15.890 --> 00:12:24.120
Borivoj: just for the fact that should be easier to use. Right? That's 1 of the goals of them, which I also have for this library

85
00:12:24.570 --> 00:12:28.100
Borivoj: same thing with polars. They start with the rust Api, which was

86
00:12:28.680 --> 00:12:31.409
Borivoj: pretty hard to use for anybody who is at

87
00:12:31.750 --> 00:12:46.579
Borivoj: from Russ world. Then they create very nice Api, right? So now everybody can understand this like not everybody but who who works with with the data frame. Apis can easily understand what is going on here. Right? They ramp all their functions in

88
00:12:46.980 --> 00:12:48.810
Borivoj: in Python, let's say, and

89
00:12:48.980 --> 00:12:56.059
Borivoj: you can now see and understand much, much easier. So this like a new norm that is happening in data role, I would say.

90
00:12:57.720 --> 00:12:59.240
Borivoj: okay, so

91
00:13:00.350 --> 00:13:15.059
Borivoj: as a data engineer working in Microsoft fabric. I work a lot in notebooks, of course, and what usually can happen in one notebook. We can have price part cell. Then we have SQL cell. Then you have price, part cell SQL. Cell.

92
00:13:15.700 --> 00:13:18.189
Borivoj: They they go on and on. So

93
00:13:18.530 --> 00:13:23.580
Borivoj: I would like to have something. Why is this? Because usually SQL. Have

94
00:13:23.810 --> 00:13:36.609
Borivoj: their advantages past park here, its own advantages in flexibility and stuff like this. So when usually, when I come to some project like in middle of the project, or you know, some project is already developed.

95
00:13:36.710 --> 00:13:46.750
Borivoj: You you come to the notebook which has price per sequel price per sequel. Right? So my goal was to create something that would have SQL.

96
00:13:46.870 --> 00:13:52.189
Borivoj: Query engine in the background to be executed like an SQL. But still to have

97
00:13:52.460 --> 00:14:08.629
Borivoj: some sort of flexibility like like Price Park. So this this was when the whole idea came from to to like to somehow mix these 2, to have one library that would replace both of these and to be able to work with it.

98
00:14:10.630 --> 00:14:13.610
Borivoj: Okay, so goals, I have a lot of goals

99
00:14:13.710 --> 00:14:21.820
Borivoj: when I was starting this. So what was, as I already said, to create simple data frame Api, that can be understand and used by anybody

100
00:14:22.400 --> 00:14:40.080
Borivoj: to make rust fields approachable to anybody who is not from rust right? So already said you should not be worried about. If you don't know rust, it's not. It's not a big problem, because you will see now how much of rust you need to to know, to be able to use this very, very little.

101
00:14:41.130 --> 00:14:51.819
Borivoj: So, as I said, to allow users to create their own query constructions. So if you want to start with, join, start with, join. If you want to start with, select start with select. You will always get the same.

102
00:14:52.000 --> 00:14:54.090
Borivoj: you will always get the same result.

103
00:14:54.190 --> 00:15:01.379
Borivoj: So also create betting capabilities. That was a part of Rest Api that I was talking about

104
00:15:01.910 --> 00:15:04.819
Borivoj: and to create reporting capabilities.

105
00:15:05.250 --> 00:15:09.960
Borivoj: Okay? So, as I said, we cannot talk about delusion without data fusion.

106
00:15:10.230 --> 00:15:12.169
Borivoj: The king of all kings. Right?

107
00:15:12.360 --> 00:15:21.579
Borivoj: This, what I believe, I think data fusion has has a great potential. But, as already showed, maybe in this previous slides.

108
00:15:22.080 --> 00:15:31.179
Borivoj: maybe it's not so easy usable by the data frame users like, or you need to know row SQL, or this data frame. Api.

109
00:15:31.370 --> 00:15:37.450
Borivoj: It's little bit, you know, not so easy to use. So what I did

110
00:15:38.150 --> 00:15:43.889
Borivoj: for this library after I rewrote it a couple of times. And why did I rewrote it? Because

111
00:15:44.550 --> 00:15:51.670
Borivoj: data frame Api expression Api SQL Api. I try to mix all together like to use the best

112
00:15:51.820 --> 00:16:04.549
Borivoj: from all Apis. And then the query engine was so confused that I couldn't. I couldn't execute half of the functions, and in order to execute most of the functions I would need to rewrite them from the scratch.

113
00:16:04.780 --> 00:16:07.030
Borivoj: and that would take me probably

114
00:16:07.390 --> 00:16:11.979
Borivoj: whole year. I don't know how much. But it would be a lot a lot of time. So

115
00:16:12.700 --> 00:16:17.900
Borivoj: 3rd time, when I was start rewriting the full library. Okay, I said, I'm gonna lean on on sequel

116
00:16:18.070 --> 00:16:30.519
Borivoj: reference. I'm gonna lean on just some pure SQL, and I'm gonna build a top of that. So all of the functions are going to execute any of these now functions you can see on screen.

117
00:16:33.530 --> 00:16:47.598
Borivoj: Okay, let's meet illusion. So how illusion came from. So I start to build, maybe year and a half 2 years ago, some project that should be similar, like, excel like, but just like

118
00:16:48.530 --> 00:16:51.910
Borivoj: to use SQL. Instead of these excel functions.

119
00:16:52.190 --> 00:17:20.179
Borivoj: This I couldn't. I didn't have time at that time. I didn't have enough time on my hands to do this. So this this project fails. And then, when I started learning about data fusion, and I already have this project for excel. Somehow this illusion came to my mind like this is the way I create this name. Illusion, excel plus illusion. Illusion, right? Not very original. But this just came up to my mind.

120
00:17:20.690 --> 00:17:29.099
Borivoj: So yeah, okay, so that's it. So why? So

121
00:17:29.220 --> 00:17:38.649
Borivoj: what is the illusion? It's just like illusion, right? And because it's built on something like data fusion. What you're actually doing, you are triggering

122
00:17:39.440 --> 00:17:44.740
Borivoj: SQL per engine in the background. So it's just basically like an illusion like a wrapper around the

123
00:17:45.050 --> 00:18:06.039
Borivoj: around this data furion. So this, I think, comes together like you're using abstraction over something that is very powerful, very strong. So it's just like it's like, illusion. Illusion. So 1st thing that I thought is a chameleon, because it can change colors and still do the same thing. Right?

124
00:18:07.880 --> 00:18:10.650
Borivoj: Okay, so prerequisite to use this library.

125
00:18:10.830 --> 00:18:31.059
Borivoj: So you need to have at least 1.8 81 ras. Because of the Delta leak, I implemented Data Lake, which is not part of data fusion. So Delta Lake requires 1.8 1 ras, because of the writers to be able to write Delta Leak, you need to have at least 1.8 1

126
00:18:31.520 --> 00:18:32.770
Borivoj: version of rust

127
00:18:34.454 --> 00:18:41.459
Borivoj: latest, and the great solution is 2.7 point O point on from this morning and Tokyo 1.42 for.

128
00:18:42.340 --> 00:18:46.570
Borivoj: And you need to enable features rt, primetime multi-trend.

129
00:18:47.190 --> 00:18:52.099
Borivoj: Okay? So what I was talking about not need to know. Rust.

130
00:18:52.260 --> 00:18:56.129
Borivoj: All that you need to know from the rest is literally

131
00:18:56.340 --> 00:18:59.299
Borivoj: main function. How to write this main function.

132
00:18:59.680 --> 00:19:07.059
Borivoj: Okay? So this user illusion payload, we're gonna give you everything that you need from this library, literally everything.

133
00:19:07.570 --> 00:19:12.389
Borivoj: And then when you create the main function should be asynchronous, obviously, because this is

134
00:19:12.853 --> 00:19:23.850
Borivoj: made us with the stickers Runtime Tokyo, which you put on top of this main function, as you can see Tokyo Main. I'm talking like this because I'm assuming there may be some rust

135
00:19:24.230 --> 00:19:29.310
Borivoj: or the some python or some other other listeners. So

136
00:19:29.860 --> 00:19:39.180
Borivoj: just you literally just need to know this 1, 2, 3, 4 lines of code from the rest. Everything else you're gonna see. It's pretty much similar to any other.

137
00:19:39.390 --> 00:19:41.029
Borivoj: any other data frame.

138
00:19:42.620 --> 00:19:44.989
Borivoj: Okay, readers and writers.

139
00:19:45.650 --> 00:19:56.699
Borivoj: So to create a data frame, you don't need to establish schema from version 0 point 2.5 I created dynamic schema

140
00:19:57.310 --> 00:20:01.989
Borivoj: in in to to be able to infer schema dynamically just from the

141
00:20:02.110 --> 00:20:18.320
Borivoj: file extension. So if your file extension is Csv. Per K. Json, or it's just a folder name. It's gonna know which file it's coming in. And it's gonna pass it like this. And it's gonna establish schema by peeking into the file. So

142
00:20:18.460 --> 00:20:34.670
Borivoj: there is like a function that is peeking into your 100 lives of lines of of file, takes the data, creates a schema for you already knows which data type, that data file is coming in and using the proper loader for this.

143
00:20:35.520 --> 00:20:54.159
Borivoj: So what you need to supply for the creating. The new data frame is just the path to your file, as you can see, and the alias, alias, is very important. If you're importing multiple files and you're going to need to join them later on. You need to provide, alias because you're going to see, you need to use, alias Dot.

144
00:20:54.290 --> 00:20:58.449
Borivoj: for your name, very similar, like like you when you're using SQL

145
00:21:00.890 --> 00:21:04.550
Borivoj: to just the path and the and the areas is very important.

146
00:21:05.040 --> 00:21:29.909
Borivoj: And this is a synchronous task of force. So you need to add a weight in rust. It's it's like in any other program language you you when you have asynchronous Runtime, you need to add a weight keeper. Right? This question mark in rust is just propagating the error. So if you made, if there is some mistake that you made, or something that you don't do right. This is gonna return you an error. It's gonna

147
00:21:30.240 --> 00:21:36.200
Borivoj: evaluate against the function that is running in the background. And I think that I provide very, very nice

148
00:21:37.021 --> 00:21:48.669
Borivoj: errors. I did create all the custom errors. So you're gonna get some nice messages like, okay, check this. Okay, check that. So I think I cover most of the of the thing that you can make a mistake with.

149
00:21:49.000 --> 00:21:51.860
Borivoj: So I think that are very helpful. The error messages.

150
00:21:53.060 --> 00:22:00.309
Borivoj: Okay, so readers, so I also have database readers. So you're gonna see now. So for the database

151
00:22:00.840 --> 00:22:06.960
Borivoj: for the other things you're gonna see? Now, you need to have to to provide the connection which is

152
00:22:07.510 --> 00:22:20.240
Borivoj: create from the string. And you need to provide the SQL. Query. So besides the connection for the database, we also have the connection from the blob storage. Currently, I'm still trying to implement for S. 3

153
00:22:20.450 --> 00:22:35.269
Borivoj: for aws still working on this. But currently you can. You can take the debt. Besides the files you can take from databases and from azure blob storage databases that I covered currently are my SQL

154
00:22:35.950 --> 00:22:39.709
Borivoj: and I covered the podcast skills. So these 2.

155
00:22:39.920 --> 00:22:52.120
Gabor Szabo: Sorry. There's a question. I think I don't know. I missed it earlier, just out of curiosity, curiosity! I can't see this English in word in English which create did you use for defining your errors?

156
00:22:52.520 --> 00:22:53.710
Gabor Szabo: This error.

157
00:22:54.620 --> 00:22:58.570
Borivoj: No, no. So data fusion and delta leak

158
00:22:58.670 --> 00:23:07.699
Borivoj: these 2. These 2 crates have their own error, data, fusion, error, and delta error. So on top of these errors I created a custom error.

159
00:23:08.040 --> 00:23:19.430
Borivoj: You can. You can go. It's very easy. The project is very simple to navigate through on Github. So you can check all the whole implementation literally, like in one file for errors.

160
00:23:20.080 --> 00:23:42.509
Borivoj: So I just wrap, wrap those errors and create my own errors which gonna come from? Is it the joining? Is it the select? Is it? Is it a string function? It's gonna target different parts of your code. And it's gonna evaluate against these parts. Gonna tell you. Oh, okay, check this. Maybe it's this problem. Oh, check this. Maybe joining is the problem. So things like this.

161
00:23:42.960 --> 00:23:46.670
Borivoj: Of course it's not a bulletproof error system, but

162
00:23:47.113 --> 00:24:03.470
Borivoj: I still don't work on it. But I'm trying to target each part. This is this is why, this is very good when you separate functions for different parts of execution, that you can target errors much better to send to users what? What is actually the problem.

163
00:24:04.630 --> 00:24:12.669
Borivoj: Okay? So I said, about my secret postgres azure. Okay? So for the azure you need price, sas token.

164
00:24:13.030 --> 00:24:38.249
Borivoj: And blob URL, your container. URL. This is going to implementation. So with the Sas token and with the with the blob URL and filtering is optional. You see some. You can. You can put none if you don't want. If you just want to grab everything from the container, if you want to grab certain part of the path. You just right there. Or if you want to just take a file, you write the file name is going to filter out

165
00:24:38.490 --> 00:24:50.450
Borivoj: everything from your container. Gonna find this file and download this for download, this for you. And of course, you need to provide areas for this table, because this is gonna directly like, evaluate this for you

166
00:24:50.920 --> 00:24:52.480
Borivoj: and add, alias.

167
00:24:53.540 --> 00:24:55.980
Borivoj: So yeah, URL and Sas token.

168
00:24:56.130 --> 00:25:25.920
Borivoj: Okay? So Json readers, this was one of the most difficult implementations I did here because Json readers from data fusion. And this any Json reading is mostly leaning through K and value pairs, very little implementations, and very, I don't know I couldn't find any crate that is, can read erase objects and flatten files.

169
00:25:26.030 --> 00:25:28.780
Borivoj: Usually you need to create your own structs

170
00:25:28.970 --> 00:25:32.050
Borivoj: and then, like deserialize the data.

171
00:25:32.350 --> 00:25:43.480
Borivoj: So this was very, very challenging, not just how to implement, but to make somewhat performant, because flattening fast, you know, it can be very challenging

172
00:25:43.760 --> 00:26:08.080
Borivoj: for performance. So I implemented Byte stream reading for this. And I think it's basically performed. But just for you to know, you can flatten if you have field and arrays field and objects. Illusion. Gonna flatten this file for you and gonna fill like gonna fill all these places that usually you will need to create struct for, or some more complex type

173
00:26:08.270 --> 00:26:10.789
Borivoj: to to be able to deserialize. But

174
00:26:11.310 --> 00:26:19.210
Borivoj: I spend a lot of time on this, so you can easily easily flatten files. And of course, key value pairs are the most performant. It just

175
00:26:19.390 --> 00:26:21.000
Borivoj: like blazing fast

176
00:26:23.380 --> 00:26:32.140
Borivoj: writers. Okay, so we have market writers. So we can write to a parquet. We can write to Csv files, and we can write to Delta

177
00:26:32.560 --> 00:26:46.119
Borivoj: this for the for the local file. So per K, you have overwrite and append mode. Csv. You have also write an append mode. But for the Cs you need to provide CC right options. This, this is mandatory without.

178
00:26:46.120 --> 00:26:49.890
Gabor Szabo: Second, just a second. I think Ray wanted to ask something. I don't know if.

179
00:26:49.890 --> 00:26:52.156
Ray Lutz: Yeah, if you don't mind

180
00:26:52.610 --> 00:26:55.070
Gabor Szabo: No, no! Feel free just to jump in and start asking.

181
00:26:55.300 --> 00:27:06.370
Ray Lutz: Well, I sometimes it's better to hear hear more, just to make sure that he wasn't going to cover it. But in terms of Json. Some databases.

182
00:27:07.245 --> 00:27:14.700
Ray Lutz: Do offer additional Json interpretation. I know postgres is is

183
00:27:14.850 --> 00:27:19.160
Ray Lutz: really good at it. Even SQL. Lite

184
00:27:19.370 --> 00:27:23.829
Ray Lutz: has some additional Json functions where you can iterate through

185
00:27:24.100 --> 00:27:29.420
Ray Lutz: Json structures that may be contained in one field, for example.

186
00:27:29.740 --> 00:27:38.390
Ray Lutz: but because you're basing everything on on the fusion

187
00:27:39.160 --> 00:27:42.100
Ray Lutz: data fusion. Or what is this?

188
00:27:43.290 --> 00:27:49.379
Ray Lutz: the Apache data fusion? It doesn't offer that. And so you're not offering anything other than what they offer. Correct.

189
00:27:49.380 --> 00:28:00.180
Borivoj: So for the databases. I'm not leaning on data fusion execution. I'm targeting through all. Dbc, you're gonna see later. So you need to create all the BC connections. So you can, you can create.

190
00:28:00.957 --> 00:28:16.839
Borivoj: You can. You can take 2 paths. You can just target directly database. Write your SQL. And put all the pressure on the SQL. Engine like Postgres or Mysql is gonna wrangle data and is gonna return the flat

191
00:28:16.890 --> 00:28:36.509
Borivoj: form for you. So in in form of columns and rows. So you put all the pressure directly on the on the query engine of there. So this is not coming through data fusion. So what comes from data fusion is once you retrieve the data from the database, then when you push the data into the data frame, then data fusion

192
00:28:36.640 --> 00:28:43.030
Borivoj: bronze, the query that you wrote right? So the whole point of the connectors is to take the data

193
00:28:44.120 --> 00:28:50.859
Borivoj: from any source and then to push it into the data frame. And then you work with the data fuse. So.

194
00:28:50.860 --> 00:29:09.709
Ray Lutz: Okay, okay. So so that's important to understand, are you gonna get to this? Because I don't want to jump the gun if it's coming later. So basically, basically you can work with other databases. And the way you do that is you're 1st bringing the data in from the database into your in memory data fusion, implementation.

195
00:29:09.710 --> 00:29:10.190
Borivoj: Yes.

196
00:29:10.190 --> 00:29:13.409
Ray Lutz: And then when you create a data frame

197
00:29:14.378 --> 00:29:21.360
Ray Lutz: it's creating it within the data fusion namespace, for example.

198
00:29:21.520 --> 00:29:22.530
Ray Lutz: And

199
00:29:22.690 --> 00:29:30.090
Ray Lutz: when you create a data frame. Normally in these SQL. Things. They haven't that each table has a name.

200
00:29:30.410 --> 00:29:32.380
Ray Lutz: So does does your.

201
00:29:33.670 --> 00:29:43.180
Ray Lutz: Does the name? Do you name the tables, or does the name wind up being just like Df. For example, is it the local name.

202
00:29:43.180 --> 00:29:50.729
Borivoj: Yes, yes. So so yes, that's a good. That's a good question like this part of implementation. So I create a very

203
00:29:51.100 --> 00:29:55.480
Borivoj: crazy aliases. That's, I assume nobody will hit

204
00:29:55.790 --> 00:30:04.919
Borivoj: when you create a query, but you need to apply your own, alias. So this might Crazy, alias will be replaced with your alias.

205
00:30:05.150 --> 00:30:24.450
Borivoj: So that's how I I overcome this. But yeah, you don't need to worry. I'm pretty sure you're not gonna hit the alias that I wrote for the when you trigger the the database. But later on, when you provide your own alias is gonna be replaced. You're not gonna feel this. You're gonna know this, you just supply your own alias, and then you work with it. Later on.

206
00:30:25.130 --> 00:30:27.469
Ray Lutz: Okay, I'll let you continue. Thanks.

207
00:30:27.470 --> 00:30:35.569
Borivoj: No problem. So for the Cs writing. So you need to provide the limited escape for double quote and null value. How you want to replace the new values.

208
00:30:36.040 --> 00:30:52.329
Borivoj: And then it's gonna work. So he has headers. It's true. So you need to work with headers. I don't think anybody in data will work with Csv without headers these days, maybe. But I disabled this. So if you want to work with Csv, you need to have headers right?

209
00:30:53.000 --> 00:31:04.639
Borivoj: And I just wanna say for the for the partition. I don't know if you can see this part that, says Delta, but it's got it's got to be this thing on on my screen. I don't know if you can see it.

210
00:31:05.330 --> 00:31:11.419
Borivoj: because I can see the the screens of of who is on the on camera. But anyway, so for for delta

211
00:31:11.690 --> 00:31:19.460
Borivoj: partitioning. So you also have a very important thing to know when you want to partition the table. You see, it's optional.

212
00:31:19.570 --> 00:31:27.190
Borivoj: It's a 3rd argument for partitioning. So I I wrote here some vector and the name of the of the column and dot into

213
00:31:28.990 --> 00:31:37.449
Borivoj: it's very important for you to know you're not gonna be able to retrieve this column anymore. Once you partition the data on. So my recommendation is create new column

214
00:31:37.980 --> 00:31:48.799
Borivoj: which you wanna partition the delta table on. So if you wanna partition on date or on integer, mostly, it's dates or some integer create new column.

215
00:31:49.300 --> 00:32:04.970
Borivoj: however, you want to call it, call it, and then partition your data. And that's very important, because once you want to read back the data, this column is not going to appear, because it's it's sucking in partition for appending. When you want to append on that table, make sure you have the same

216
00:32:04.970 --> 00:32:21.480
Borivoj: column name and the same column. We're gonna that you already partitioned on to have this same column on this new new data that is coming in because you're gonna otherwise, it's gonna fail. You need to have the same column name and the same data type is date, or is integer right

217
00:32:21.480 --> 00:32:28.749
Borivoj: to be able to partition and to be able to append on the Delta table. So that's very important for for for you to know, this is current

218
00:32:30.264 --> 00:32:52.110
Borivoj: currently how it's implemented. Okay? So just wanna say, if anybody want to implement create data frame, just for you to know, overwriting is easy. Appending is very hard to implement very hard. If anybody wanna try this to do, it's very hard, because you need you cannot lean on any implementation is coming. Because at this point, you're gonna

219
00:32:52.620 --> 00:32:58.250
Borivoj: have so many different intricacies to work on. And you, you're gonna need to implement.

220
00:32:58.950 --> 00:33:24.469
Borivoj: you need to work against your record batches. And you're gonna need to prick a lot a lot on performance. This is much much harder to append than to, and then to overwrite for overwriting. I I usually use the native overwriters of the data fusion for the Csv and packet. But appending it's it's not working. So you need to. So just if anybody will try to do something like this. It's much harder to append than to overwrite.

221
00:33:24.920 --> 00:33:42.079
Borivoj: Okay? So to azure blob storage writing is currently only supported in parquet. Why is this? Because at my current one of my current projects I need. I had this requirement to grab the data from Api and to write in the into azure to park it like more

222
00:33:42.650 --> 00:33:48.810
Borivoj: did all like file types of work. Come, I will implement for Json writing as well, and Csv.

223
00:33:49.890 --> 00:34:02.289
Borivoj: for now only only the per key file you can write. So this is a small example. So let's say you have some data custom data frame. Df, dot select here is just like an example. Of course it's can be much more complex.

224
00:34:02.770 --> 00:34:08.630
Borivoj: Then you you state your URL to the your container folder, sas token.

225
00:34:08.830 --> 00:34:27.029
Borivoj: And you use this data frame data. You see above what is called query dot illusion. It's very important. Illusion function is like an execute function in other data frame. So illusion is actually evaluating your query. And it's creating a new

226
00:34:27.909 --> 00:34:47.689
Borivoj: alias for the later on usage. If you want to use multiple like in same execution. You want to use the same. Let's data source. And you want to create multiple queries until you come to your final query. So it's very important to use illusion, evaluation. Before that you don't need to use it. You can just change, change and then

227
00:34:47.830 --> 00:34:52.590
Borivoj: make sure to use illusion. It's like an execution on top of your all of your queries.

228
00:34:52.889 --> 00:35:14.540
Borivoj: and then you can see this use right parquet to azure with Sas. I try to create all the function as as more as most logical, as I think. Like to remember. I'm writing per key to azure with us. So that's how I I name my function somehow. For me it's easier to remember. I don't know if this is very practical. But yeah.

229
00:35:14.690 --> 00:35:31.070
Borivoj: maybe somebody gonna disagree with me. But for me to azure with Sas. And this how I call this. So you have 2 2 modes, overwrite and append, and, as you can see, you provide your URL to to the storage and the sas token.

230
00:35:31.450 --> 00:35:56.619
Borivoj: So for writing, it's very important to make sure for such token that you have writing permissions to check this. Some of my colleagues are persistently trying to write with Sp. Tokens without writing and creating permissions, so make sure to to have this to have this insight. So writing is set to default. Compression is snappy 2.0 per kit, and the buffer size are put 1 GB.

231
00:35:56.790 --> 00:36:14.650
Borivoj: I can extend this, but I think 1 GB is. It's a it's a big parquet file. So like I, I said, this limit this mostly for also for my purposes. If something goes wrong with some Api not to not to, you know, make a big bill

232
00:36:14.840 --> 00:36:16.129
Borivoj: for my company.

233
00:36:16.980 --> 00:36:22.057
Borivoj: So data wrangling. So we're gonna now cover a little bit of syntax and

234
00:36:22.860 --> 00:36:35.219
Borivoj: a bit of design. So we're gonna go through different functions that I create. And why I decided to to use some things in some functions and take it out from from other ones.

235
00:36:35.750 --> 00:36:46.849
Borivoj: So when we talk about select select has a case insensitive. S, so can you use capital S or S. Small? You can see here.

236
00:36:47.620 --> 00:36:58.099
Borivoj: You can, of course, select star. You can select everything, or you can select, count everything and and etcetera. So from this 1st glance.

237
00:36:58.220 --> 00:37:03.520
Borivoj: you already, I think you understand what is going on here. I think you already realize that I created

238
00:37:04.180 --> 00:37:23.610
Borivoj: functions which you're gonna give you greater flexibility. But you're gonna still be able to use this sort of a SQL syntax, because, of course, we are targeting data fusion Sequel SQL engine in the background. So this is kind of a design like, maybe you're not gonna like it. Maybe you're gonna like it. But

239
00:37:23.980 --> 00:37:34.330
Borivoj: we're gonna see now. So for numerical operators. So currently operators operators that are supported is plus minus multiplication division, the mode.

240
00:37:34.520 --> 00:37:45.545
Borivoj: So later on, you're gonna see in which functions you should use. Which currently, you can see a small example here of how you can use the filter filter is basically like,

241
00:37:46.380 --> 00:37:47.380
Borivoj: where

242
00:37:47.570 --> 00:37:56.090
Borivoj: where in in SQL, so you're filtering a pre aggregated data with filter having which you're gonna see later on, is filtering

243
00:37:56.750 --> 00:38:07.599
Borivoj: data after aggregation like very similar like you have in the in, in, the, in this, in the sequel. So for order by here, it's a single single

244
00:38:08.200 --> 00:38:19.410
Borivoj: argument, because you also have order by many which you can supply multiple multiple columns to order by. Currently, I have for ascending and descending. I have true and false

245
00:38:20.390 --> 00:38:24.459
Borivoj: still thinking how to to implement this, maybe just to to be able to to add

246
00:38:24.680 --> 00:38:32.099
Borivoj: a SC, or DESC. Instead of true false, but currently is true. False, true is ascending false is descending

247
00:38:32.260 --> 00:38:55.159
Borivoj: and limit. We all know what is limit is just you're limiting the the amount of of roles you, wanna you wanna see, and then you evaluate with illusion. As already said, you added a new alias, because this, if you want to use it further, you can. You can evaluate with illusion function, and then, later on, you can use it in other. In other query

248
00:38:55.340 --> 00:39:01.800
Borivoj: and display function. We already know what is display function which is gonna display the the data frame on the on the console for you.

249
00:39:02.430 --> 00:39:05.955
Ray Lutz: So on this one before you go on the

250
00:39:08.050 --> 00:39:15.219
Ray Lutz: The 1st part, when you're creating this variable called num OP. Sales.

251
00:39:15.750 --> 00:39:20.939
Ray Lutz: those the functionality there just pretty much constructs the query right?

252
00:39:21.340 --> 00:39:26.086
Ray Lutz: And then in the the let down below.

253
00:39:27.120 --> 00:39:33.510
Ray Lutz: that actually essentially executes the query. Is that the way to think of it?

254
00:39:33.700 --> 00:39:41.970
Borivoj: Is, is exactly the way you should think about it. So you change. Change, then, with illusion. You are

255
00:39:43.210 --> 00:39:53.710
Borivoj: what is the point in illusion. We have another function in the background that is called construct sequel, which you're never gonna see in implement for usage. But it's in the background.

256
00:39:54.460 --> 00:40:08.800
Borivoj: So the the way this functions, and why you can actually write in any sequence. The the function says, like select filter, you can like literally mix up and down any of these functions is going to work the same, because

257
00:40:08.980 --> 00:40:17.420
Borivoj: when you evaluate allusion, it's constructing in the background the SQL. Query in the order which query needs to be constructed.

258
00:40:17.420 --> 00:40:18.200
Ray Lutz: I see.

259
00:40:18.200 --> 00:40:24.850
Borivoj: So now you need illusion to evaluate your credit. You wrote against

260
00:40:25.290 --> 00:40:35.860
Borivoj: SQL. Construct in the background. And that's how I implemented this. Okay, use whatever you want to use, and however you want to use it is going to be, give you the same result in the background. So this is

261
00:40:36.030 --> 00:40:36.880
Borivoj: like.

262
00:40:37.750 --> 00:40:41.729
Ray Lutz: So, except except for the that is there.

263
00:40:42.090 --> 00:40:57.380
Ray Lutz: In other words, I'm trying to figure out what is the benefit of using illusion. Certainly you get that benefit because you don't have to get these in the right order, and you're checking that in creating the query properly.

264
00:40:57.600 --> 00:41:01.479
Ray Lutz: But pretty much you have to know about queries and how they all work

265
00:41:02.202 --> 00:41:07.689
Ray Lutz: very much like a regular SQL. Query. Right? I mean, yes, very much.

266
00:41:07.690 --> 00:41:11.290
Ray Lutz: Yes, very much similar. But you're gonna see some advantages.

267
00:41:11.850 --> 00:41:29.550
Borivoj: As I said, it's it's it's I'm trying to make a i i already made it a mix of pie spark and sequel, so to have flexibility to write the way you want to write. You don't need to use like, select everything from where this group by this order, this right? You don't need to write it like SQL,

268
00:41:29.780 --> 00:41:36.889
Borivoj: but it's basically like a sequel, right? But it just you. Can. You have this flexibility to write any way you want to write it. So it's like

269
00:41:37.400 --> 00:41:37.940
Borivoj: the one.

270
00:41:37.940 --> 00:41:40.289
Ray Lutz: Okay, and then, and the result of this.

271
00:41:40.620 --> 00:41:44.190
Ray Lutz: So when you do an execute, then

272
00:41:44.360 --> 00:41:49.569
Ray Lutz: would you get out of that a, an object which is.

273
00:41:49.690 --> 00:41:52.210
Ray Lutz: you get this object? None ops res.

274
00:41:52.770 --> 00:42:00.550
Ray Lutz: and then you say, dot display now is that none ops res is that like a proxy object that is.

275
00:42:01.150 --> 00:42:03.210
Borivoj: It's new data framework. It's new data.

276
00:42:04.200 --> 00:42:11.620
Borivoj: So against this, Num Ops rest. Now, you can change more. So, for example, you aggregated, aggregated something.

277
00:42:11.620 --> 00:42:11.990
Ray Lutz: Okay.

278
00:42:11.990 --> 00:42:36.559
Borivoj: You. You join 10 tables, or whatever, and you evaluate Num. Ops results. Then this, this variable is going to hold all result of previous square. Then you can work later on. If you're on a union or something, or append, or whatever you can use against this. So because it's storing a data frame which is evaluated with illusion. Right? So.

279
00:42:36.560 --> 00:42:42.949
Ray Lutz: Is that always a new frame in your mind? Or do you sometimes return the same one like if it's a join?

280
00:42:43.220 --> 00:42:47.959
Ray Lutz: Well, maybe we're gonna get to join. So I'll wait. Okay, thanks. I'll stop there.

281
00:42:48.060 --> 00:42:48.585
Borivoj: Okay.

282
00:42:49.640 --> 00:42:53.050
Borivoj: So filtering as I said, you can filter many

283
00:42:54.780 --> 00:43:05.490
Borivoj: very important thing which I'm still dribbling in my mind. I wanna implement inline parameters. I gonna talk about this at the end. So current implementations and development

284
00:43:06.416 --> 00:43:19.979
Borivoj: but hopefully, if I figure it out the way I wanna figure it out so soon. We should have also be able to use inline parameters meaning you can create some scalar value or just

285
00:43:20.110 --> 00:43:27.338
Borivoj: any value in in. You want to store in a in a variable, and then you can. You should be able to evaluate against

286
00:43:27.800 --> 00:43:55.959
Borivoj: your data frame data frame column. So this couldn't what is coming. But currently you have filter which saw before, and you have filter many. Just make sure to make it in square brackets, because it's like like a list writing in in rust. I'm talking like this because of the I assume python users. So you put in square brackets and each inch filter you separate with a with a with a with a regular like close bracket. So this, how you you create multiple filtering

287
00:43:55.960 --> 00:44:00.160
Borivoj: again. As I said, filtering is used for pre-aggregated

288
00:44:00.160 --> 00:44:24.850
Borivoj: values. So having is used after aggregations. But you're going to see now. Order by we covered and everything of this we covered already. So scalar functions, so scalar functions are used exclusively in select function, they will not work in other functions like aggregation. And like string functions that I created. You're gonna see now. So you can use scalar functions directly in in select.

289
00:44:25.670 --> 00:44:30.070
Borivoj: Of course, you can use this nested, nested scalar functions as well

290
00:44:30.290 --> 00:44:36.440
Borivoj: filtering recovery. But we cover already so aggregating functions, so aggregating functions.

291
00:44:36.560 --> 00:44:56.580
Borivoj: of course, can can use nested functioning so round, average, absolute, but also can use operative based aggregations, as we also see previously in the in the select function. But here, if you use aggregate function, you can also, like illusion, is able to evaluate your

292
00:44:56.690 --> 00:44:59.560
Borivoj: operator operator based aggregations as well.

293
00:45:00.060 --> 00:45:21.090
Borivoj: So just something to keep in mind. And we come to the new function group by group by have 2 variants you're gonna see? Now have group by and group by many. I try to use the like similar logic for everything. Is it single? Or it's many group by or group by many right field filter, many order by order, by many. So everything is like this

294
00:45:21.470 --> 00:45:34.629
Borivoj: and usually the same. So currently group by, you obviously need to put all the non aggregated columns in order for this to work. If you if you miss some non aggregated column, it's gonna fail. You're gonna you're gonna get error message

295
00:45:34.930 --> 00:45:36.820
Borivoj: for a group. By all.

296
00:45:37.050 --> 00:45:45.920
Borivoj: it's automatically evaluating all of your non aggregate column. So I usually just use group by all if I need something. But

297
00:45:46.120 --> 00:45:50.859
Borivoj: of course, if it's small in enough example, you can use group by as well.

298
00:45:52.610 --> 00:45:55.630
Borivoj: Okay. So now we come to the more complex query.

299
00:45:56.000 --> 00:46:04.289
Borivoj: starting so select aggregation, filtering group by all, as you can see below, so group by all is gonna evaluate all of your

300
00:46:04.720 --> 00:46:24.139
Borivoj: non aggregate functions. And then it's going to give you the the right result and ordered by many, as you can see here is, similarly used like any other multiple argument function. So you put square brackets and in individual brackets, you put the the conditions right. So

301
00:46:24.320 --> 00:46:28.949
Borivoj: in this case, total billable, false, and Max, absolute billable value.

302
00:46:29.450 --> 00:46:36.170
Borivoj: and the rest is the same. You use illusion, relation, and and you you break on it as as usual.

303
00:46:36.280 --> 00:46:40.139
Borivoj: Okay, so currently supported functions are these

304
00:46:41.230 --> 00:46:50.939
Borivoj: so for aggregations and for scalar functions? So this what is currently supported, I think, is more than enough. But if some somebody have some suggestions.

305
00:46:51.750 --> 00:46:56.800
Borivoj: Please let me know, because you would think, Okay, I can use probably everything from sequel. No, you cannot.

306
00:46:57.660 --> 00:47:01.079
Borivoj: This is still restricted, because I'm passing this

307
00:47:01.240 --> 00:47:07.109
Borivoj: very heavily in the background, each of these each of these words, each of these bracket.

308
00:47:07.220 --> 00:47:27.010
Borivoj: if everything from this, it's heavily passed in the background. So like, if I like, you know, when I say, Okay, this is supported literally, this supported. So if anybody like needs more, please, so you can easily find me, and if you need some more functions. But I think this cover most of the what is needed.

309
00:47:27.590 --> 00:47:36.850
Borivoj: Okay, so joins. So we have different type of join. So we can join 2 data frames for joining 2 data frames we're gonna usually use join.

310
00:47:37.360 --> 00:47:53.419
Borivoj: So you're gonna very similarly, like you in any other data frame libraries. You provide the this, for example, df, sales, this some evaluated data frame from before and then. Now you have another data frame that you had

311
00:47:53.640 --> 00:48:01.039
Borivoj: somewhere above in your code. Now you want to join these 2. So you have Df sales and you have Df customers, and then you provide the joining column.

312
00:48:01.450 --> 00:48:07.190
Borivoj: and of course you you provided the the type of joining. Is it in there left, or or whatever?

313
00:48:07.600 --> 00:48:22.250
Borivoj: And the rest is the same aggregation group by having okay. So having this is single having, and you provide total quantity greater than 10. As you can see, it's evaluating against aggregated function.

314
00:48:22.430 --> 00:48:30.089
Borivoj: This very important. So having filtering aggregations filter filtering before aggregations. Okay.

315
00:48:31.680 --> 00:48:37.240
Borivoj: this I'm saying like this, because if you don't know sequel, this is how usually you speak sequel as well as well

316
00:48:37.610 --> 00:48:42.059
Borivoj: to okay, this for the single joints. Okay? So we have.

317
00:48:42.270 --> 00:49:07.110
Borivoj: If you need to join more than one data frame you, you can use join many. So you join many again, same as any other, many function that they show already square brackets, individual brackets for for different data frames. So here we are joining 3 different data frames, 1st one, then the second, and the 3, rd and the rest is pretty much the same.

318
00:49:08.745 --> 00:49:14.300
Borivoj: Besides this, what is also supported is joining on multiple columns.

319
00:49:14.990 --> 00:49:29.640
Borivoj: Okay, so joining multiple columns for us, for 2 data frames, we again use joining function. But you also can provide in the square brackets, multiple columns 2, 3, 4. However much you want.

320
00:49:29.820 --> 00:49:38.550
Borivoj: Same thing goes with the for the join menu. When you want to join multiple data frames again. Same thing, square brackets. Then you put

321
00:49:38.870 --> 00:49:51.019
Borivoj: individual brackets, and inside you supply multiple joining columns. Very important thing to know is what restriction we have right now here. The restriction right now here is that you cannot

322
00:49:51.180 --> 00:50:06.420
Borivoj: join uneven number of volumes, which means what which means if you wanna join 1st and second data frame on 2 columns, and you want to join second 1st and the 3rd

323
00:50:06.670 --> 00:50:19.930
Borivoj: data frame on one column is not gonna work. So you need to have equal number of columns. Currently, I was, I was unable to implement this. It was failing in testing. So I I currently.

324
00:50:20.040 --> 00:50:24.840
Borivoj: oh, I'm still, I'm still. I still didn't finish this implementation.

325
00:50:25.200 --> 00:50:36.019
Borivoj: But currently, just for you to know, this is a limitation. So if you want to evaluate on multiple columns. You need to have a same number of columns that you are evaluating against.

326
00:50:36.180 --> 00:50:44.949
Borivoj: Okay, so but now we are starting another function for me it was the most logical thing to do when you work on a string columns.

327
00:50:45.270 --> 00:51:00.620
Borivoj: Okay? So we were gonna string columns. My whole idea to create this new function called string functions, which I don't think you have in any other data frame. So maybe this is going to be a little bit weird for you, but for me it was very logical. If you want to work on some

328
00:51:01.120 --> 00:51:20.519
Borivoj: string function or like any function that that of course you have string type. Utf. 8. Or if you come from Json or any other text, virtual whatever, from whatever data source coming and whatever string kind of time it has, you should use string function

329
00:51:21.010 --> 00:51:45.449
Borivoj: because string function is differently. Parsing in the background is differently parsing in the rest of the functions. So if you're gonna place some string function in select, it's gonna fail. It's not gonna work. So make sure if you work. If you are applying some function that is usually working on string functions, make sure to put it in the string function, as you can see just here below select.

330
00:51:47.340 --> 00:52:16.360
Borivoj: So yeah, so we're gonna talk about more string functions just now. So this is more example, as you can see here, join many select string function, aggregation group by all, having many order by many. As I said already, probably 5 times. You can literally write here, select string functions, join aggregation, you can write string, function, select aggregation, join literally. You can write here, however, is most logical for you. It's gonna work the same.

331
00:52:17.410 --> 00:52:24.570
Borivoj: And now, as you can see, 3 fashions have all these stream concant left right. All this most usable.

332
00:52:24.840 --> 00:52:27.849
Borivoj: like most frequently use string functions.

333
00:52:27.850 --> 00:52:31.070
Ray Lutz: Okay? Quick question. When you do joins.

334
00:52:31.660 --> 00:52:38.719
Ray Lutz: can you join without doing the aggregations and selections and create views.

335
00:52:39.630 --> 00:52:44.370
Borivoj: You cannot create views because everything is stored as a data frame in memory.

336
00:52:44.640 --> 00:52:50.629
Borivoj: So each of I don't have lazy evaluation like like polars, has.

337
00:52:50.790 --> 00:52:59.980
Borivoj: for example, porous, has lazy evaluation. So you write right right, and only when you execute, then actually do something here.

338
00:53:00.170 --> 00:53:07.340
Borivoj: There is no such thing as a view. You cannot store ticket as a view, you can only sort thing as a data frame in memory. So

339
00:53:07.340 --> 00:53:09.036
Borivoj: is that part of

340
00:53:09.920 --> 00:53:14.130
Ray Lutz: Apache data, fusion, functionality that they don't have use.

341
00:53:14.990 --> 00:53:31.339
Borivoj: Yeah, they have used. You can use, you can create views. But I didn't implement it. So so the the way I implement this is everything. Come and go through the data, frame everything, come and go through your local memory. So

342
00:53:31.670 --> 00:53:40.230
Borivoj: this is how I implement this. So you cannot create the if. If you're saying view, you want to interview where you want to create view in memory, or you want to create view in.

343
00:53:40.260 --> 00:53:45.579
Ray Lutz: Well, it's just well, but because it's in memory, I mean.

344
00:53:45.760 --> 00:53:58.850
Ray Lutz: this, this thing operates in memory. You're not operating off on a disk somewhere. And that's a limitation of this, right? Because you can't have too big of a data frame. It will run out of memory

345
00:53:59.613 --> 00:54:00.680
Ray Lutz: and then.

346
00:54:00.680 --> 00:54:05.340
Borivoj: Depending on, of of your memory of computer. If you have 32 GB of memory.

347
00:54:05.340 --> 00:54:15.439
Ray Lutz: So if you have lots of you'd have to have RAM, though it can't be on disk. Right? Yeah. So even SQL lite allows you to have some pretty large

348
00:54:15.690 --> 00:54:18.170
Ray Lutz: tables.

349
00:54:18.410 --> 00:54:25.489
Ray Lutz: and then what they do in those other SQL, maybe it's I, just. I haven't used data fusion before, but

350
00:54:25.600 --> 00:54:30.339
Ray Lutz: they? They let you make views, and then, in order to make

351
00:54:30.900 --> 00:54:41.309
Ray Lutz: you know, then you can. You can join 2 tables. You can join another table. You can join another table, and nothing happens until you select on it.

352
00:54:41.310 --> 00:54:41.810
Borivoj: Yes, yes.

353
00:54:41.810 --> 00:54:42.260
Ray Lutz: Then.

354
00:54:42.260 --> 00:54:43.480
Borivoj: And she's like on it.

355
00:54:43.480 --> 00:54:43.800
Borivoj: Say no.

356
00:54:43.800 --> 00:54:45.950
Ray Lutz: Then it goes back, and it figures out

357
00:54:46.070 --> 00:54:52.889
Ray Lutz: how it's going to, you know, because it has to has to behind the scenes.

358
00:54:53.030 --> 00:55:05.860
Ray Lutz: you know. Take the 1st one, get the key, you know. Find the other one, find the key in that one. Get the data that you're looking for. Pair it up, you know, all the way through. Okay, all right. Thank you. That's my question.

359
00:55:05.860 --> 00:55:07.950
Borivoj: Yes, yes, yes. Yeah. Yeah. So

360
00:55:08.290 --> 00:55:29.849
Borivoj: it is what it is. Currently, it's in, built for in memory data format without lazy evaluation. So when you run it. How you construct your query at the moment you evaluate is gonna start crunching your data. There is some optimization for compressing bytes and everything. But you cannot go

361
00:55:29.940 --> 00:55:43.339
Borivoj: over your limits of your local machine. That's impossible, because this is, for in memory data formats like, say, similarly, like pandas like, you know, Polars has more advanced lazy evaluation. So similarly, like Pispark.

362
00:55:43.850 --> 00:55:56.369
Borivoj: it's lazy, validated as well. You can write, write, write, write, write, you can store store store, use, use whatever, and then you execute, and then it's figuring out. And then you wait for 5 min. Basically here.

363
00:55:58.070 --> 00:56:01.029
Borivoj: here it's a little bit different. It's made, it's

364
00:56:01.750 --> 00:56:07.740
Borivoj: quite performant. But you have this limitation of of your own network.

365
00:56:08.000 --> 00:56:14.799
Borivoj: It is what it is. Okay. So currently implement the joints in the left, right, full left, same right, same left and right and the left mark

366
00:56:14.920 --> 00:56:21.070
Borivoj: think covers. Most of the are normal humans working with data

367
00:56:22.301 --> 00:56:29.630
Borivoj: string functions. Okay? So, as I already said, let's just repeat a little bit more and show it a bit more examples. So in string functions

368
00:56:30.050 --> 00:56:38.149
Borivoj: you can, of course, create new columns, non-existing like this, like new as new old customer. You can add string values to the.

369
00:56:38.320 --> 00:56:45.290
Borivoj: to the, to the coins. You can, of course, use all these string functions as as usual.

370
00:56:46.520 --> 00:56:59.159
Borivoj: Here are more examples left, trim, right, trim, upper, lower length, left, right, any, as I already said, not to repeat myself any string functions. You need just feel free to to put it in the string functions.

371
00:57:00.010 --> 00:57:11.119
Borivoj: Yeah, so that's it. More examples here replace and stay the repeat reverse left pad, right, bed, and all this other stuff. Still, I'm working more on string types of dates.

372
00:57:12.200 --> 00:57:22.480
Borivoj: There there is casting there is to chart and stuff like this. But I still think I think I still didn't cover all the time and date functions that data fusion

373
00:57:22.610 --> 00:57:32.439
Borivoj: has natively still need to to go through all of these. But most of the like general things I covered. So these are the currently available string functions

374
00:57:33.020 --> 00:57:38.300
Borivoj: that you can use on the in the string in the string function, in in your query.

375
00:57:38.890 --> 00:58:07.930
Borivoj: okay, so windowing, windowing, I was thinking how to implement this because you already saw there is, mostly this logic. We like filter filter, many order by order, by many things like this. But for windowing it's overly complex to have window many, because now you're not going to be able to figure out for me, at least for writing errors, was a little bit difficult if I have window many, and I have a list of windows

376
00:58:08.210 --> 00:58:22.199
Borivoj: to actually to to designate which one is failing. So for me it was a little bit easier to implement this like this, and I think it's also enough, is it's it's more than enough complex windowing to have it as a single as a single function. So

377
00:58:22.430 --> 00:58:25.150
Borivoj: I implement like this. So you can do.

378
00:58:25.630 --> 00:58:30.950
Borivoj: aggregated window functions, ranking analytical window functions. And you also can do rolling.

379
00:58:31.250 --> 00:58:32.330
Borivoj: So

380
00:58:33.080 --> 00:58:43.230
Borivoj: anything that you need. I think it's covered here, and it's gonna be properly parsed. This is was the the hardest thing to implement for parsing, because it can get pretty quick. Pretty complex.

381
00:58:43.650 --> 00:59:01.399
Borivoj: But yeah, I think I covered the the most cases for for windowing, and you can use, of course, in windowing for aggregations all the as it showed before all the aggregation functions you can use here you here we can see some and average, but you can use all the other that they showed before for aggregations as well.

382
00:59:02.650 --> 00:59:05.149
Borivoj: Yeah, that's that's it for that.

383
00:59:05.450 --> 00:59:24.810
Borivoj: Okay, append append many. So I also added, this was not necessary. This is literally like Union, and like Union all, and union all, many, literally the like same functionalities. But I know some people like append and can remember, is it append? So you can use this one as well? It's gonna keep all the rows. It's not gonna remove duplicates

384
00:59:24.900 --> 00:59:36.689
Borivoj: in any case. So what I think it's I'm a little bit biased here in Pispark and other data frames you need to. If you want to append. If you want a union. If you want to do any of this.

385
00:59:36.800 --> 00:59:47.059
Borivoj: you need to provide data frame dot function data frame dot function inside data frame. But here I implemented, if you wanna

386
00:59:47.230 --> 00:59:54.510
Borivoj: append multiple data frames, you just literally say, append many. And you just add each data frame you want to append.

387
00:59:54.660 --> 01:00:00.130
Borivoj: So I think this is a bit easier usage than in other data frame libraries out there. So

388
01:00:00.490 --> 01:00:23.779
Borivoj: yeah, you can see, example here, 5 Json files, you create data frames you do some whatever. This is just simple, like queries. You evaluate all of these. As I said, if you want to use it further, you evaluate through illusion. You need to do this, and then you can use it, append, append many and all these other functions that are coming right now you're gonna see

389
01:00:24.170 --> 01:00:32.420
Borivoj: Union Union all accept and intersect. So Union compiles the rows for both and removes duplicates. Union. All, as I said.

390
01:00:32.650 --> 01:00:37.260
Borivoj: similar like appendment is, gonna keep all the rows, not removing duplicates

391
01:00:37.670 --> 01:00:54.989
Borivoj: except an intersect difference between 2 sets and intersection of 2 sets as well. So this is a small example of of union. So you have 2 data frame, df, one df, 2, and then you evaluate through illusion. And then you union these 2, these 2 tables

392
01:00:55.590 --> 01:01:02.939
Borivoj: and same thing is for union all except and intersect you. You provide you provide this tool.

393
01:01:03.470 --> 01:01:06.629
Ray Lutz: So do the columns have to be exactly the same.

394
01:01:06.630 --> 01:01:14.490
Borivoj: Yes, need to be the same need to have same number of columns need to have same data. Types need to have same call names.

395
01:01:15.660 --> 01:01:16.270
Ray Lutz: Thank you.

396
01:01:18.860 --> 01:01:21.820
Borivoj: Thank you for this question, because I will forget to say this.

397
01:01:23.001 --> 01:01:31.429
Borivoj: So, okay, so union, many union, all, many same same logic. So you create 5 different data frames union many.

398
01:01:31.690 --> 01:01:36.850
Borivoj: You provide us the 1st data frame, because on the 1st data frame, this is very peak

399
01:01:37.000 --> 01:01:39.900
Borivoj: on the on the data. And I this how I establish

400
01:01:40.250 --> 01:01:54.699
Borivoj: after the evaluation, I establish the again, I need to establish again in this case the schema, and then against this 1st file that you provide 1st data frame that you provide. Then I'm comparing all the the rest. Right? So

401
01:01:55.490 --> 01:01:59.377
Borivoj: yeah, so union many and union all many. It's

402
01:02:00.040 --> 01:02:13.030
Borivoj: yeah. So it's you can see union. Many same thing like union, just with many. It remove duplicates and union all, many is same like union. All just you can provide multiple data frames is gonna keep the duplicates.

403
01:02:15.220 --> 01:02:21.309
Borivoj: the pivoting. Okay, pivoting was challenging, very challenging and pivot and on pivot

404
01:02:21.470 --> 01:02:29.120
Borivoj: because are not supported by data, by data fusion. Natively. So I need to

405
01:02:29.320 --> 01:02:40.570
Borivoj: create pretty interesting break around this thing. So a very important thing. This is a sequence function, it cannot be changed with other functions.

406
01:02:40.950 --> 01:02:47.260
Borivoj: So just remember this, you cannot write select target string and then

407
01:02:47.870 --> 01:02:55.489
Borivoj: change. Pivot is not gonna work, pivot and pivot are separate functions from other functions. So you need to evaluate your data frame

408
01:02:56.100 --> 01:02:58.110
Borivoj: to work this. So you're gonna

409
01:02:58.420 --> 01:03:01.089
Borivoj: let's say you wanna people directly on the

410
01:03:01.350 --> 01:03:11.599
Borivoj: row. Csv, you create custom data frame, and you build on the data frame or another version, you work your chaining, you write your query, and then

411
01:03:11.850 --> 01:03:15.570
Borivoj: you evaluate with illusion, and then you can use

412
01:03:15.690 --> 01:03:30.049
Borivoj: pivoting. So you you need to provide illusion. Then, separately, on this data frame, you need to pivot or unpivot. So this is just an example. Here on the screen, CC. Files little bit of data crunching. You use illusion.

413
01:03:30.430 --> 01:03:35.779
Borivoj: This display is not necessary. If you want to see what or or not, you can remove this, but then on this

414
01:03:36.230 --> 01:03:41.480
Borivoj: scalar results scalar rest. You can peel with. Now. Okay, it's very important

415
01:03:42.200 --> 01:03:50.870
Borivoj: to to know this part. You cannot change. You cannot add Pyota above, let's say, after a filter, you, you cannot do this, so you need to be used separate.

416
01:03:53.640 --> 01:04:13.809
Borivoj: Okay, I don't. I don't want to go in the story about futures. But when you work with a sequence, runtimes and sequence functions, just as I said at the beginning, use await. And the question mark, just remember this. You don't need to think about this too much. Just await dot await question. Mark, dot await question, mark. That's all you need to remember from the rest.

417
01:04:14.140 --> 01:04:16.010
Borivoj: You don't need to worry about this.

418
01:04:17.090 --> 01:04:20.990
Borivoj: Okay, people. Same story, identical story.

419
01:04:21.460 --> 01:04:30.740
Borivoj: You can work directly on the on the data frame or after the illusion evaluation you can. You can. You can just call your compute.

420
01:04:32.610 --> 01:04:34.749
Borivoj: Okay, statistical functions.

421
01:04:34.930 --> 01:04:45.189
Borivoj: statistical functions I provide, maybe are going to do more. But for me, whenever we did, it's very important to quick evaluate the data frame to see how many nulls I have, how many rows like

422
01:04:45.210 --> 01:05:06.119
Borivoj: like just checking the data a little bit. So I create 3 different statistical functions. So here you can see on the data frame. So you get some metrics for the for the columns you have. How many records have coming on? All values mean standard deviation and mean. And Max range just a little bit that you have some, you know.

423
01:05:07.272 --> 01:05:15.330
Borivoj: Idea about your columns, what what is going on. So here you're gonna get a report for each individual column. I didn't put all the

424
01:05:15.450 --> 01:05:24.519
Borivoj: all the volumes. It will be too big picture. But below this, you're gonna get 2 more, 2 more results, right for each for each of these.

425
01:05:24.880 --> 01:05:33.689
Borivoj: for each of these columns in your data frame. Of course, if you wanna for example, create new analysis, which I usually do, it's kind of a

426
01:05:33.930 --> 01:05:57.357
Borivoj: thing to do to see how many nulls you have in data frame one of the things to check your data so you can provide none in rust. You have this option type. It's like to think about this like, let's say, is it or is it not I? I can use it, or I don't need to use it. This is option. Right in the background is a little bit more complex. It's

427
01:05:57.910 --> 01:05:59.569
Borivoj: actually helping you.

428
01:06:00.033 --> 01:06:02.020
Borivoj: Not to get the null if you

429
01:06:02.370 --> 01:06:12.049
Borivoj: checking some value, and if he's gonna come now, just to cover this, not to get these problems with with errors. But just think of this like a option like the thing

430
01:06:12.510 --> 01:06:31.320
Borivoj: I can use it. I don't need to use it in this case. You don't need to use it, and what you don't need to use is to specify the column name if you just like, say, data frame any data frame that you created and display new analysis, none is gonna evaluate all of your columns. If you provide the column name in the in the

431
01:06:31.970 --> 01:06:41.449
Borivoj: I call this in the braces. How I say, then, you're gonna validate on the on the on the quality. But if you want to just place none, it's gonna validate the whole data frame. Usually you'd use this.

432
01:06:41.890 --> 01:06:57.869
Borivoj: But anyways, okay, so display correlation matrix for some people that I don't usually use this, but if if you have any need to to see correlation between the the columns in your data frame, you can supply the columns.

433
01:06:58.310 --> 01:07:02.930
Borivoj: And then you're gonna get correlation between between the columns. Usually, I think this is used

434
01:07:04.000 --> 01:07:07.970
Borivoj: from data scientists. They usually do this. I don't usually do this. But

435
01:07:09.050 --> 01:07:11.620
Borivoj: I implement this just in case if anybody needs it.

436
01:07:13.150 --> 01:07:41.389
Borivoj: Okay? So we come now to the Odpc open database connectivity and talking about Mysql and postgres which I implemented here. So for Mysql. If anybody use work with Odbc connections before, you need to know that you need to have a driver without a driver. It didn't work. It's not gonna work so you need to have locally your driver installed for your my SQL, so currently, for example, I use this 9.1 unico driver.

437
01:07:41.800 --> 01:07:51.300
Borivoj: and then it's very important for you to remember. You need to supply these keywords as they are right here on the screen. Because SQL is looking for this

438
01:07:51.410 --> 01:07:54.139
Borivoj: keywords for postgres is different

439
01:07:54.310 --> 01:08:02.810
Borivoj: keywords. You're gonna see now. So basically, this is in multiple rows. But this is just like one string I just as you can see, I slashes just to have. I can

440
01:08:03.290 --> 01:08:07.700
Borivoj: see it nicer. But you can just play this in in one line as well, of course.

441
01:08:08.700 --> 01:08:29.370
Borivoj: And then, as I said before, try to push as much as possible pressure on the SQL. Server. So don't put too much pressure on on data frame execution, because you want to get from the I don't know you can. I test this on 1.6 million rows of data with 32 columns with like with 2 tables.

442
01:08:29.470 --> 01:08:31.579
Borivoj: So you have very nice performance.

443
01:08:31.689 --> 01:08:45.609
Borivoj: But try to try to put pressure on the on the sequel in the SQL engine. So don't put too much pressure. Because if you have multiple millions, multiple tables, you need to join with multiple 1 million rows.

444
01:08:45.950 --> 01:08:54.250
Borivoj: I don't know 5 min rows each. I don't know. It's gonna be too much for your memory to do this like put crunching as much as you can on the

445
01:08:54.689 --> 01:09:07.310
Borivoj: on the SQL. Engine. So or or limit amount of rows you want to take. If you, you can also keep like limit, 20,000 100,000 rows. It's gonna be

446
01:09:07.420 --> 01:09:23.509
Borivoj: extremely fast without you importing all the data. Once you are happy with what you get. Then you can remove limitation, limit, clause, and just run your query as you want. Just my recommendation. Don't try to break your computer right?

447
01:09:23.850 --> 01:09:24.850
Borivoj: So

448
01:09:25.149 --> 01:09:44.420
Borivoj: once you create the connection string and the query, you just use custom data frame from dB, where you're gonna where you're gonna create the the execution. Then you need to use illusion and give it new alias. As I said in the background, I create the crazy alias

449
01:09:45.340 --> 01:10:01.049
Borivoj: when I, running from dB, but you need to worry about this. Add your alias in illusion, illusion, evaluation. Then, later on you can work with with, you can change your functions, work with the data frame as as usual, as we already saw before.

450
01:10:02.140 --> 01:10:10.700
Borivoj: so similar to Odbc. Remember these names. So driver. Several poor database uid pwd, so this is what

451
01:10:10.830 --> 01:10:14.989
Borivoj: probably is expecting. It's a little bit different than my my skill.

452
01:10:15.610 --> 01:10:36.480
Borivoj: And then, yeah, so driver is post SQL unicode that I'm using. You're gonna maybe need some. I don't know which version policies have, but just check which driver you need. Install the driver in your computer and then you can work. You can work with your database. And same, just create illusion, evaluation. Give it alias, and then you can work it later on with the data frame. If you need.

453
01:10:36.660 --> 01:10:43.901
Borivoj: Why is this? Why would why would you use this like? If you can run sequel directly on SQL, let's say you want to work with

454
01:10:44.500 --> 01:11:03.590
Borivoj: you have a analyst in your team. You wanna give them the data from the database and then they work in fabric. So they have native Gen, 2 data lake connection with azure. Right? So you take the data from the database. You push data on the

455
01:11:04.400 --> 01:11:12.350
Borivoj: you push the data on the azure, and then they take it in fabric and then work later on whatever. So just this is like, let's say, use case

456
01:11:12.560 --> 01:11:21.630
Borivoj: for for, or if you. If you if you do locally and publish, let's say power Bi reports or tableau, or whatever reports

457
01:11:21.670 --> 01:11:45.749
Borivoj: you crunch the data with a data frame, you give Csv files to your analysts that they create locally. They publish, publish, you know, to to to the cloud the reports, I don't know, like there are different use cases. In. In in expert group we have different clients, some smaller Serbian clients that we have. Like some furniture company, they work like this.

458
01:11:45.750 --> 01:12:04.600
Borivoj: we have a VPN to their computer to to their station there in the company, and we just supply from the database the the files, and locally we are analysts are creating reports and just publishing, you know. So smaller companies need the smaller solutions. Bigger companies need bigger solutions. But

459
01:12:04.930 --> 01:12:08.440
Borivoj: yeah, you can, you can find use case, probably

460
01:12:08.670 --> 01:12:12.530
Borivoj: for yourself. So azure bulb storage. We already talk about this.

461
01:12:12.690 --> 01:12:21.030
Borivoj: So we don't need to talk too much. So reading from the azure blob storage currently support the Csv. Json. Still, I need to implement the reading packet.

462
01:12:21.740 --> 01:12:34.269
Borivoj: So it's opposite way. I can read Json and Csv, and then I can store parquet. But I cannot read parquet. Okay, so currently. But I gonna implement reading parquet and storing Json and Csv.

463
01:12:34.480 --> 01:12:42.680
Borivoj: I gonna do vice versa as well. So as I already showed you very important thing. If you didn't work with azure blob storage before.

464
01:12:43.070 --> 01:13:09.479
Borivoj: just check like URL to your container, if it has the word blob that's gonna run on the SDK azure SDK, which is not performant if you're gonna if you have word, Dfs is gonna also work through. SDK, but it's gonna be much faster, because using Delta Lake storage second generation. So it's much, much faster. So ask your

465
01:13:09.650 --> 01:13:20.670
Borivoj: principal, or whoever you are, who is your manager? Just okay. Please give me Dfs code. Right? So dfs, URL, so it's much, much faster execution and reading and everything.

466
01:13:21.290 --> 01:13:28.629
Borivoj: Okay, so you provide your URL, your saas token, and then use from azure with sas token

467
01:13:28.640 --> 01:13:30.740
Borivoj: very bluntly to

468
01:13:31.450 --> 01:13:59.139
Borivoj: function name, but from azure with us token you use to create custom data frame. Again, you can use filtering you, you give it alias. Here I just give alias data. And then later on, you, you write your query, and you work with your data if you need to put back your usually. So, for example, we have one company from from Dubai Emirates. So we take the data from in Json format and we store it back in parquet. For example, this is used case

469
01:13:59.140 --> 01:14:03.669
Borivoj: so you can take the Json files which are like we you have like

470
01:14:03.700 --> 01:14:06.360
Borivoj: I don't know over 500 Json files.

471
01:14:06.580 --> 01:14:18.510
Borivoj: We take it we do some aggregation calculations. We pack it in a pocket and we push it back to the, to the azure. So this one use case that your real life use case as well.

472
01:14:20.510 --> 01:14:47.289
Borivoj: Okay. Rest. Api, okay. So rest. Api. I tried to put as much as possible things I could the most like flexible options I could think of. So you can write from Api. Just simply, this is the most simple use case. You just give the URL, and you just give it the path. So all the from Apis. All rest. Api implementation is taking directly from the Api data and storing in Json

473
01:14:47.550 --> 01:14:59.120
Borivoj: in your local local files. Right? So you just provide the URL, and you provide the path and the name in. I just made this before. It was in the 1st implementation was separately

474
01:14:59.400 --> 01:15:11.520
Borivoj: path and the the file name. Now, I just, I think it's easier to use like this where you just remove the this argument. So you use 2 arguments. So yeah, this is from Api

475
01:15:11.740 --> 01:15:17.549
Borivoj: and then from Api with headers. So you can just create a hash map. So hash map

476
01:15:19.593 --> 01:15:21.286
Borivoj: in order to be

477
01:15:21.880 --> 01:15:50.209
Borivoj: program languages. You have different data structures. Right? Hash map is one of the structures you can use without going too much into the hash map. You just basically storing different values before you're going to use it here. So if you're going to create multiple headers so you can create hash map which you're going to use later on in from Api with headers function so as you can see here in example, one you create, you create mutable

478
01:15:50.960 --> 01:15:51.530
Borivoj: headers

479
01:15:52.070 --> 01:16:17.790
Borivoj: variable? Because why is mutable because you are adding more stuff to it. So it needs to be mutable. You are changing the structure right? Rust is very strict of this, of of types, because rust natively, when you create a variable, it's immutable by default. So if you want to create mutable, variable, you need to put this word mut so moot.

480
01:16:17.910 --> 01:16:28.230
Borivoj: So now you can, you can change. You can change this, this this variable and store store multiple things in in a sequence, in in the variable. So

481
01:16:28.380 --> 01:16:55.670
Borivoj: this is simple example with a single header which so now doesn't make much sense right? But it's going to make more sense. In the second example. But you have 2 headers now. You can store. You can see you can store multiple headers, and then you can use it from Api with headers, and it goes on, I don't want to bore you with this, but you can check Github repo. You have much more example. So parameters also hash map. You can hash map, you insert parameters and use parameters.

482
01:16:55.740 --> 01:17:25.160
Borivoj: Then you can mix with parameters and with headers. So now you have parameters. You have headers, and you supply this to the to the from Api with parameters and headers. So I try to create as much as possible. This is from just if you need dates, you can use this one with dates. You evaluate some data from date range. If you need like, for one year data or something. You can use this one also. You can use this with with parameters as well, of course. But I just create just, you know, if you need something

483
01:17:25.640 --> 01:17:50.380
Borivoj: more specific pagination. So you can grab data with different number of pages. And how many roles per page you want, you want to grab depending, if, like, if data is huge or something. So you can page paginate the data and and then evaluate to the data frame with sort. You can also sort like before even you using the the data frame Api from the illusion, you can sort the data before you receive the data

484
01:17:51.530 --> 01:17:56.579
Borivoj: with headers and source. So headers and sort and and so on. Okay.

485
01:17:56.990 --> 01:17:59.770
Borivoj: now, this is the last, the most

486
01:18:00.371 --> 01:18:06.880
Borivoj: relaxing feature I implemented. This is the my favorite feature, because this is the easiest thing to implement

487
01:18:06.910 --> 01:18:35.679
Borivoj: in a sense that it's you know, it's the most creative thing here. It's not so much technical. It's a lot of Javascript. So Javascript is very flexible. So here we have ability for reporting and dashboarding. So you have, you can create plots, and you can create tables from today last version, latest and the greatest. So these are currently available plots.

488
01:18:36.230 --> 01:19:05.029
Borivoj: Here is an example. So plot, line, plot, plot, time series. I don't want to mention all of this. You can check on the on the Github repo, and then X and y axis you. You give it the true for line. If you want to have these dots and see you're gonna check the I don't wanna talk too much. It's super simple. So it's natively baked into the data frame. So after you create data frame, if you want to create a report, you can create your plots, you're gonna see now. And you also, you can create your tables.

489
01:19:05.570 --> 01:19:09.830
Borivoj: Okay, this is more example of plus histogram box plus character.

490
01:19:09.970 --> 01:19:30.719
Borivoj: And then you can create tables. So you you create some query. So now you want to take certain columns from the query, and you want to order it, or whatever, and you need to pass it through illusion to to create the, to create the alias again, just for evaluation. You need to do this, and then it's ready to be used in dashboard.

491
01:19:31.310 --> 01:19:45.570
Borivoj: So you create list of plots that you create. For example, if you create line Time series, you, you pass this to the to the list. Same thing for the tables. You pass all the tables that you created into the tables, and then

492
01:19:45.800 --> 01:20:02.849
Borivoj: for the report creation, you create report layout and table options. How you wanna have your layout for the report. 2 columns, 3 columns, one column grid gaps, and everything. You can check on documentation there as well, and the table options.

493
01:20:02.880 --> 01:20:25.089
Borivoj: You want to budget tables, you wanna what kind of tables you want, etc? You can all the options you can. You can check there and then you can edit Lg, team. Alpine is the. I think I like this one the most. Keep this one because it have green green lights like our chameleon. So I, using this one ag ag team Alpine.

494
01:20:25.370 --> 01:20:39.040
Borivoj: So it's have very nice and modern look, you're gonna see now and then, as a final step you use create report. So you adding the plots, you adding the tables report title, and where you're going to create your HTML file.

495
01:20:39.240 --> 01:20:41.299
Borivoj: So now the whole point is.

496
01:20:41.360 --> 01:20:55.649
Borivoj: you are creating report into the HTML file which you can share with anybody with your company. Of course, bigger solutions have much more advanced this like tableau power bi whatever. But I wanted to create this

497
01:20:55.680 --> 01:21:24.870
Borivoj: full circle of taking the data, crunching the data, creating the report. This was very important for me, because this was my dream for a long time now. So I want to create some full implementation of this whole circle, as you can see here, where is the sum for python users and other programming language users? It's again optional. You can keep it none. If you don't have plots. You have just tables, keep none for plots.

498
01:21:25.120 --> 01:21:39.159
Borivoj: and vice versa. So everything where you see some this is optional. You do need to use it. Same goes for layout, config and table options. I create default structures for the report. So if you want to use default structures.

499
01:21:39.290 --> 01:21:54.959
Borivoj: you just keep this as none NONE, and it's gonna create for you. And now we're gonna show you the demo how the report looks like. So this is the tables. I this is ag grid table, so you can sort it. You can reorder the columns. You can just

500
01:21:55.130 --> 01:22:06.042
Borivoj: quick like you can make it easier to look. And and whatever you want to do, you can, of course, filter. You can choose the dates and stuff like this for the plus. Of course, you have this

501
01:22:06.990 --> 01:22:10.340
Borivoj: different information you can see from you can zoom in, zoom out

502
01:22:11.120 --> 01:22:22.209
Borivoj: stuff like this. You can slide on the timeline. This is for the time series, if you need. And then most important thing for the users of the business is export to excel button.

503
01:22:22.350 --> 01:22:28.749
Borivoj: Of course, now we have export to excel, and you can export the tables. Of course you cannot export the

504
01:22:29.020 --> 01:22:34.219
Borivoj: the, the plots. You can export everything which you placed in the tables down below.

505
01:22:35.300 --> 01:22:38.059
Borivoj: of course. Okay. So that that's that's about it.

506
01:22:38.920 --> 01:22:59.839
Ray Lutz: Hey? So I got some questions. I love your dashboard thing that is really tremendous. I think that's that's a real selling point. Now, when you're reading Csv, it normally comes in as as strings. Right? So how do you get the data types? Is there some functionality where it tries to guess what they are.

507
01:23:00.190 --> 01:23:13.299
Borivoj: Yes, Arrow, I'm using heavily. Apache Arrow, my best friend in the whole world, Apache Arrow. So I'm peeking into the the I'm peeking, as I said, in 100 rows of the of the data.

508
01:23:13.750 --> 01:23:17.830
Borivoj: or 100 lines depending what it is, or if a Json couple of objects

509
01:23:18.090 --> 01:23:23.359
Borivoj: and then I'm comparing against all the possible existing arrow types.

510
01:23:23.680 --> 01:23:34.980
Borivoj: And then arrow does this actually actually for us? So it's recognized the data type. And it's stored if it's not able to recognize. It's automatically goes to Utf 8,

511
01:23:35.470 --> 01:23:52.970
Borivoj: meaning from Utfa to string. If I'm for example, there is some like very complex date formats which currently I was. I was not previously. Recently. I was working with some pos data from the cash register and the dates on that is like 0. 1

512
01:23:53.550 --> 01:24:09.450
Borivoj: name in the in the string, then the and you have the the here. Then you have this time zones, then you have. You know all this, so it's kind of hard to evaluate all the data types. Right? But 95% of data types are covered.

513
01:24:09.450 --> 01:24:17.439
Ray Lutz: Mostly dates. But what can you override? Is there an override to set the data types? Because since Russ is very picky about this.

514
01:24:17.690 --> 01:24:28.630
Borivoj: No, it's not. That's not. It's stuck like this because I didn't. Wanna as I said already, this should be as much user friendly as possible. But trust me.

515
01:24:28.930 --> 01:24:33.319
Borivoj: 95% of data types are covered. 5% goes to string.

516
01:24:33.650 --> 01:24:48.880
Borivoj: And then later on, if you need to cast, you can use string functions. You can like, you know, you can take the parts out. You can cast to date or whatever you need, so you can. You can work your way around. If there's some complex string that I cannot, I cannot.

517
01:24:48.880 --> 01:24:57.439
Ray Lutz: Is it fair to say that the the main impetus for this is because rust really doesn't have a good data frame package.

518
01:24:58.310 --> 01:25:05.710
Borivoj: Rust has good data frame packages. As I said, polar has very good data frame package, but

519
01:25:05.870 --> 01:25:14.950
Borivoj: user unfriendly, in my opinion. And the reason why they switch 100% to python Api is because

520
01:25:15.190 --> 01:25:19.110
Borivoj: even python, even rust users are having hard time to use it.

521
01:25:19.490 --> 01:25:24.190
Borivoj: So that was the question, is there any? I think I.

522
01:25:24.190 --> 01:25:28.170
Ray Lutz: Because, of course, Pandas has one.

523
01:25:28.330 --> 01:25:33.860
Ray Lutz: This is not the Pandas Api, and that's the more traditional like python based

524
01:25:34.473 --> 01:25:38.809
Ray Lutz: or if you're using numpy, you know. Then they have data frames. There.

525
01:25:38.810 --> 01:25:39.280
Borivoj: Yeah.

526
01:25:39.870 --> 01:25:45.949
Ray Lutz: And they're more of a pythonic kind of, you know. Square bracket.

527
01:25:45.950 --> 01:25:46.370
Borivoj: Yes.

528
01:25:46.370 --> 01:25:49.479
Ray Lutz: Know row column, whatever kind of thing where you would.

529
01:25:49.480 --> 01:25:52.589
Borivoj: Yes, indexing everything here. You don't have indexing stuff like this.

530
01:25:52.590 --> 01:26:07.706
Ray Lutz: Kind of like ours. Data frames, too, where you you say row and column. And then, you know, there's a group by and stuff. Okay? I, yeah, I love this. So in the dashboard, though, that's implemented as is that a Javascript?

531
01:26:08.645 --> 01:26:09.240
Borivoj: Yeah.

532
01:26:09.490 --> 01:26:17.649
Ray Lutz: Does that like? If you create a report, are are the is the report still active like this? Or.

533
01:26:17.650 --> 01:26:26.249
Borivoj: Yes, it's interactive. It's HTML. When you run xtml, you close it, you open it, you refresh it. Whatever you do. It's always active like this.

534
01:26:26.430 --> 01:26:32.059
Ray Lutz: Is there a way to get just the dashboard part separate from the rest of the stuff? I mean? Is it cause that seemed.

535
01:26:32.060 --> 01:26:39.869
Borivoj: I remove. I just removed the Png because I thought it's not necessary. But if you think, for example, it would be.

536
01:26:39.870 --> 01:26:43.750
Ray Lutz: Know what I let's say

537
01:26:44.710 --> 01:26:53.990
Ray Lutz: you, because it seems like a separate layer, if you will, of your solution where it's not specific to

538
01:26:54.110 --> 01:26:58.990
Ray Lutz: the data frame syntaxing or anything else.

539
01:26:59.100 --> 01:27:07.370
Ray Lutz: and would be very useful with just about any other data frame, for example, or anything else

540
01:27:07.500 --> 01:27:12.939
Ray Lutz: is, did you? Did you like come up with the the dashboard

541
01:27:13.260 --> 01:27:19.710
Ray Lutz: as a custom implementation? Or did you use some prepackaged.

542
01:27:19.710 --> 01:27:47.158
Borivoj: Package. So I use for all our package use and all our community versions. So you're not gonna ever gonna spend the time on this also one thing that is very important. This is open source from the 1st day, and it's free to use and everything I'm implementing. I'm checking the licensing and I'm checking is gonna be costly in the future. So all the packages from Javascript for

543
01:27:47.650 --> 01:27:49.530
Borivoj: for excel is Ajax

544
01:27:50.080 --> 01:28:13.939
Borivoj: is Ajax. The library for the excelling and Csv for the for the Ag. Grid is agrid community license or community library. So it's free to use. So this is what I'm using so ag grid for the tables, Ajax for the exporting and the plotly community version, not the the paid one that goes in the dash is used for the, for the plotting.

545
01:28:14.390 --> 01:28:15.380
Borivoj: so everything.

546
01:28:15.865 --> 01:28:20.359
Ray Lutz: Dash, it's mostly plotly dash is what you're using for this.

547
01:28:20.360 --> 01:28:23.160
Borivoj: Not dash, not dash, just plot, link, no plus

548
01:28:23.160 --> 01:28:29.542
Borivoj: and dash are yeah separate thing. I'm not using dash, because for dash, dash is using.

549
01:28:30.570 --> 01:28:37.109
Borivoj: they have the, if I remember correctly, they have us like, think of a

550
01:28:37.690 --> 01:28:46.030
Borivoj: the functions that you need to call the function on the action to be, and to re crunch the data every time I think for that.

551
01:28:46.030 --> 01:28:46.630
Ray Lutz: No worries.

552
01:28:46.630 --> 01:28:58.360
Borivoj: Yeah. So this is, you need to consult. You will need to have constant connection with your data and with your queries once you create HTML, you are totally detached

553
01:28:58.730 --> 01:29:14.420
Borivoj: from your queries and from your implementation and from your execution. That is so. This is the reason why I can implement dash community version dash. But then, I need to also figure out how to have constant connection with your data.

554
01:29:14.820 --> 01:29:17.989
Ray Lutz: And have sliders and things like that. Interactive controls.

555
01:29:17.990 --> 01:29:21.189
Borivoj: Yeah. So currently, I can do some

556
01:29:21.430 --> 01:29:47.529
Borivoj: gymnastics to create some functions, to to be able to filter the data in the HTML report. I can probably do this like, here, you can filter. If you can see for the tables, you can filter the data. You can do stuff like this. So, yeah, you can do this for the tables, but it's not connected with the charts. It will be perfect if you can slider this filter that, and everything is interact in this case. Currently, it's not.

557
01:29:47.640 --> 01:29:59.790
Borivoj: But yeah, it's the most recent. I just start doing this like, maybe 10 days ago, 15 days ago, I don't know. So yeah, we'll work on more. We'll work more on this.

558
01:29:59.790 --> 01:30:01.310
Ray Lutz: For answering my questions.

559
01:30:01.310 --> 01:30:02.450
Borivoj: No problem.

560
01:30:02.630 --> 01:30:07.220
Borivoj: Okay? So pipeline scheduling for me, this is very important.

561
01:30:08.120 --> 01:30:18.949
Borivoj: okay, this is obviously in memory data frame. This is obviously run in memory. But of course you can create rust image in a docker.

562
01:30:19.210 --> 01:30:27.480
Borivoj: And you can create this more out, like, let's say, automatic. But again, if you have a computer in your company that is not

563
01:30:27.860 --> 01:30:32.030
Borivoj: doing much, or you can run this process through the

564
01:30:32.180 --> 01:30:39.449
Borivoj: on your server through. Let's say shell script or something. You can also automate this like this. But, for example, if you just open

565
01:30:39.610 --> 01:30:59.240
Borivoj: Vs code or any code id that you're using, you can create your functions, your queries, whatever you want to do, you wrap it in a pipeline scheduler, and then you can set the oh, sorry. Then you can set the frequency of job repeating. So it comes from 1 min to 30 days.

566
01:31:00.080 --> 01:31:01.920
Borivoj: I try to implement

567
01:31:02.627 --> 01:31:08.840
Borivoj: but it's it was failing so far. I tried to implement the exact time when you wanna run the

568
01:31:09.010 --> 01:31:11.769
Borivoj: when you want to run the the job. But

569
01:31:12.240 --> 01:31:19.260
Borivoj: it's based on Utc time. So now I want to cover all the time, zones and everything, and it's

570
01:31:19.520 --> 01:31:28.910
Borivoj: takes a lot a lot of more more things to cover and to think about and to have, you know, internal evaluation of your daytime

571
01:31:29.330 --> 01:31:46.170
Borivoj: your system date time. Then to check which time zone you are then to a lot of stuff needs to be covered for this to work. So I just keep it currently like this. So for example, you run it right now. And it's gonna tell you. Okay, process is finished, give you result. And then you're gonna tell next

572
01:31:46.490 --> 01:31:56.169
Borivoj: job run is that time that mean that second, based on the time that you have on your local machine without knowing what is your time zone? Okay.

573
01:31:56.550 --> 01:32:02.239
Borivoj: so quickly is implemented like this. So you create pipeline schedule new, you create

574
01:32:02.926 --> 01:32:08.610
Borivoj: the how like how many frequent, how frequent! You want Job to be run

575
01:32:08.750 --> 01:32:14.119
Borivoj: in minutes, hours, or days, as you can see here on the right hand side. And then.

576
01:32:14.810 --> 01:32:18.580
Borivoj: okay, this is a little bit more specific. But just remember double pipe.

577
01:32:18.680 --> 01:32:21.509
Borivoj: Just remember this. If you're coming from Python, just

578
01:32:22.010 --> 01:32:36.300
Borivoj: double pipe, Async, and then you've wrap it in the credit brackets. Okay, just try to remember this like this. Don't think too much about this. I couldn't run the process multiple times without this.

579
01:32:36.640 --> 01:32:59.469
Borivoj: But anyways, just you go pipeline schedule, new frequency, battle pipe, async, and acrylic brackets, and everything else is, as you already saw. Here, example is, we are taking the data from azure, from azure blob storage. We do some crunching and we are saving data to park it. This is locally saving. You can also do saving back to azure, for example.

580
01:32:59.850 --> 01:33:12.429
Borivoj: So you run this every day once a day, couple of like, or every 4 h, or every 6 h. And it's gonna repeat, it's gonna repeat its job. And this is implemented with talk, your chrome scheduler. So

581
01:33:12.830 --> 01:33:17.660
Borivoj: yeah, it's very, very interesting implementation, how they calculate minutes, hours, and everything.

582
01:33:18.590 --> 01:33:29.140
Borivoj: Okay, development. So we come to the end. I hope I didn't bore you too much. So currently, what I'm working on. So, as I said before, inline parameters for filtering and having

583
01:33:30.491 --> 01:33:41.100
Borivoj: problem. Here is the securities problem here securities problem here. And I tried to implement something, but my audit was failing.

584
01:33:42.370 --> 01:33:50.589
Borivoj: There is different template templating crates that you can use here, there, or you can use formatting, but it's kind of a dangerous.

585
01:33:51.100 --> 01:33:53.890
Borivoj: for you know, for the.

586
01:33:55.060 --> 01:34:11.200
Borivoj: you know. So for the injection sequel injection. But maybe this is unreasonable to think, because if you want to use it locally. Who can? Who can do anything to you? Right you. You use it, you execute it, you close it. But anyways I couldn't. Because I have this github actions that I'm running. Every time I'm

587
01:34:11.480 --> 01:34:19.440
Borivoj: I'm uploading new new version. It's it's failing. So I need to think very thoroughly about this because of security reasons

588
01:34:20.390 --> 01:34:26.799
Borivoj: as well as how you're gonna be able to use it without again knowing too much trust. That's very important. So

589
01:34:27.120 --> 01:34:50.940
Borivoj: try to make it simple and safe. It's kind of challenge. So S, 3 Connector working on it. Date functions, I said, already working still didn't cover all date functions in from data fusion and reading packet from azure blob and writing. I didn't say this here, but, as I mentioned before, writing Json and Csv to azure blob. So these are 5 implementations that I'm

590
01:34:51.520 --> 01:34:54.859
Borivoj: currently working on. Of course.

591
01:34:55.720 --> 01:35:16.009
Borivoj: if anybody has any suggestion, please write to me. Tell me, okay, Bora, I need this for my job. Can you implement this? Give me a couple of days? We'll see what I can do if I can implement something, because I'm using this for my job. I have very spectacle skeptical principle that use C sharp and.net for many years.

592
01:35:16.200 --> 01:35:33.949
Borivoj: So every. It was very hard for me to convince him. Okay, rust is good. Let's use rust. Rust is, you can do Job, you know, with less cost. We can do this locally. We can do that, but they're still skeptical, many people. But if you're going to use this in for your work.

593
01:35:34.400 --> 01:35:55.110
Borivoj: just tell me what you need. If you need to convince your boss. We can talk. We can schedule the chat. I can explain everything how it works. I can show implementations, whatever you need. I'm here for you. I spending a lot of my free time on this without expecting anything in return, because 1st and the foremost, I'm using this library for my work.

594
01:35:55.220 --> 01:36:00.789
Borivoj: And I'm doing this firstly, for myself, okay, and then for everybody else.

595
01:36:01.170 --> 01:36:02.130
Borivoj: So

596
01:36:02.280 --> 01:36:10.440
Borivoj: please don't feel free to contact me, to to ask for any feature or or anything that you need. I'm here to. I'm here to

597
01:36:11.290 --> 01:36:13.460
Borivoj: oblige. Is that the word?

598
01:36:14.220 --> 01:36:28.500
Borivoj: Okay? So thank you. If you are one of these 10,000 people that are using illusion. Thank you for using illusion. Here is the crates. You can check the crates, Github and my Youtube channel, where I'm posting from time to time and not so active on Youtube channel.

599
01:36:28.810 --> 01:36:36.740
Borivoj: But yeah, so contact me, let's chat. I'm always open for for chat and for implementations and for improvements.

600
01:36:37.360 --> 01:36:41.230
Borivoj: And of course I do things wrong. And then I fix it.

601
01:36:41.810 --> 01:36:47.170
Borivoj: Okay, so I'm not, of course, perfect. Nobody's perfect. And this library is not perfect, but

602
01:36:47.390 --> 01:36:52.280
Borivoj: what it does currently, I think it does. Well, so yeah, that's it.

603
01:36:52.450 --> 01:36:54.950
Borivoj: And thank you for your patience.

604
01:36:55.450 --> 01:36:57.510
Borivoj: That would be it.

605
01:36:58.260 --> 01:37:04.560
Gabor Szabo: Well, thank you. Thank you very much. It was a along with an excellent.

606
01:37:04.954 --> 01:37:09.690
Borivoj: Sorry I told you before. I thought oh, 9 o'clock! Oh, my.

607
01:37:09.690 --> 01:37:11.930
Dragana Dragana: Thank you, Bora. Thank you.

608
01:37:11.930 --> 01:37:12.610
Gabor Szabo: Thank you.

609
01:37:12.610 --> 01:37:22.660
Dragana Dragana: Everybody must go. It was very. It was very interesting. Thank you so much. Thank you for. And greetings from Uk. Bye, bye.

610
01:37:22.910 --> 01:37:23.620
lapid: Bye-bye.

611
01:37:24.420 --> 01:37:25.769
Gabor Szabo: Okay, so thank you.

612
01:37:25.770 --> 01:37:26.190
Ray Lutz: Or.

613
01:37:26.190 --> 01:37:28.010
Gabor Szabo: Everyone, and thank you for the questions.

614
01:37:28.420 --> 01:37:35.070
Ray Lutz: Did. Did you do any benchmarking, I mean, how is this an attractive thing for speed? Because I know.

615
01:37:35.070 --> 01:37:55.659
Borivoj: I did. I did a lot of benchmarking, still fastest pocket reader and writer in the world for a single node execution, because I didn't implement this. This has come from data fusion. We have fastest reader and writer in the world. So if you need fast reading and writing with pocket. Okay, I I was, as I said before, I was messing with appending.

616
01:37:55.830 --> 01:38:00.229
Borivoj: So it's it's not so easy. But still performance is.

617
01:38:00.230 --> 01:38:01.040
Ray Lutz: Or pending.

618
01:38:01.210 --> 01:38:08.070
Borivoj: Yeah, so appending is a little bit different. But writing and reading is the fast in the world

619
01:38:08.290 --> 01:38:13.169
Borivoj: for Json grading. It's very, very fast, even for the flattening.

620
01:38:14.440 --> 01:38:20.170
Borivoj: I didn't want my selling point to be. I'm faster than pandas, like some other libraries.

621
01:38:20.780 --> 01:38:24.759
Borivoj: I don't think that's the most important thing. I think the most important thing is accuracy.

622
01:38:25.240 --> 01:38:27.630
Borivoj: It's a good and safety.

623
01:38:27.870 --> 01:38:49.079
Borivoj: Okay? And simplicity of using. Okay? So I'm not in a I'm not in a running track trying to beat anybody by speed. I want to create something that is safe, stable and and good to use. So yeah, I did benchmarking. There is some benchmarking post results from benchmarking. But

624
01:38:49.250 --> 01:38:51.379
Borivoj: yeah, as I said, most important thing.

625
01:38:51.570 --> 01:38:54.160
Ray Lutz: Yeah, I think it's not that hard to beat Panda so. And.

626
01:38:54.160 --> 01:38:58.530
Borivoj: Yes, that's what I want to say. Some people brag. They are faster than pendants.

627
01:38:58.750 --> 01:39:13.329
Borivoj: you know, but I never want to do this. Pandas is my love. I love pandas. I use pandas like I was using pandas a lot before, so I would never trash and bash pandas on speed. Pandas is great.

628
01:39:13.520 --> 01:39:15.679
Borivoj: Go use blunders, go use.

629
01:39:16.085 --> 01:39:20.144
Ray Lutz: Did you? In entertain the idea of using the pandas?

630
01:39:22.155 --> 01:39:23.130
Ray Lutz: Syntax!

631
01:39:24.280 --> 01:39:40.240
Borivoj: As I said the reason I was overwriting these 2 times, it's because the problem is with the data fusion different. Apis. I tried to create very similarly at the beginning. If if I go back to 1st couple of versions, it was very, very similar, like

632
01:39:40.340 --> 01:39:53.689
Borivoj: column dot function, or, you know, aggregations with dots like some mean. But, as I said, I hit the wall for expansion with more complex things.

633
01:39:54.030 --> 01:39:57.690
Borivoj: Then I lean back on the the SQL. SQL. Reference.

634
01:39:57.690 --> 01:39:58.350
Ray Lutz: Hmm.

635
01:39:58.350 --> 01:40:07.970
Borivoj: It is what it is. It's life. You need to make your peace with the limitations and work on what you can work. Otherwise I will need to implement, like polars, everything from the scratch.

636
01:40:08.250 --> 01:40:13.259
Borivoj: all my functions, and then it would take couple of years for this. But.

637
01:40:13.260 --> 01:40:21.320
Ray Lutz: Another thing that's that's come up with, because I've been working on a similar data frame package called Daffodil, and I invite you to take a look

638
01:40:21.510 --> 01:40:23.820
Ray Lutz: within python.

639
01:40:24.060 --> 01:40:29.610
Ray Lutz: But one thing that's come up lately are are embeddings, so they want to put in

640
01:40:30.314 --> 01:40:34.030
Ray Lutz: say, like a numpy array in one cell.

641
01:40:36.760 --> 01:40:39.379
Ray Lutz: You want to have 384 bits.

642
01:40:40.390 --> 01:40:44.950
Ray Lutz: all right, 384, 32 bit floating point numbers.

643
01:40:46.200 --> 01:40:48.810
Ray Lutz: And that's what fits into a

644
01:40:49.050 --> 01:40:53.716
Ray Lutz: cuda like a a you know. Gpu

645
01:40:55.620 --> 01:40:59.829
Ray Lutz: at one time. So and then so the the record

646
01:41:00.010 --> 01:41:10.740
Ray Lutz: is these, these individual embeddings, and this is a word that they use so. And then they really don't want to index into these.

647
01:41:11.170 --> 01:41:11.780
Borivoj: Oh, okay.

648
01:41:11.780 --> 01:41:17.320
Ray Lutz: Put that into one cell. So what do you think the outlook is? Oops?

649
01:41:19.330 --> 01:41:21.250
Ray Lutz: For putting in

650
01:41:23.830 --> 01:41:31.199
Ray Lutz: somehow, like this is what daffodil is good at because it uses python data types. So you can nest whatever you want into a cell.

651
01:41:31.610 --> 01:41:34.040
Borivoj: Tell me you have a front end in what.

652
01:41:34.960 --> 01:41:37.820
Ray Lutz: Pyth daffodil is a python.

653
01:41:37.820 --> 01:41:44.700
Borivoj: So no, okay, but you, you're saying sell. I'm imagining like a spreadsheet of some sort, and you are.

654
01:41:44.700 --> 01:41:49.910
Ray Lutz: Yeah. So in your case, if there's a column and a row in a database, so you have

655
01:41:50.030 --> 01:41:52.800
Ray Lutz: a record and you have a column name.

656
01:41:53.290 --> 01:41:54.310
Borivoj: And that.

657
01:41:54.750 --> 01:42:01.840
Ray Lutz: That's that item normally isn't is a number in there, like in data frames, they think of it as one number.

658
01:42:01.840 --> 01:42:02.250
Borivoj: Yes.

659
01:42:02.250 --> 01:42:08.500
Ray Lutz: What they want to do is put 384, the 32 bit floats.

660
01:42:08.890 --> 01:42:18.689
Ray Lutz: Okay in one cell. So they don't really want to split out and have 32 big, 384 columns, because they never referenced the columns.

661
01:42:19.793 --> 01:42:21.320
Ray Lutz: And so

662
01:42:22.100 --> 01:42:26.370
Borivoj: But suppose that it's tricky, with a floating point in a single set.

663
01:42:26.370 --> 01:42:30.649
Ray Lutz: Now in in postgres, they have a thing called Pg. Vector.

664
01:42:30.870 --> 01:42:31.450
Borivoj: Oh, okay.

665
01:42:31.450 --> 01:42:41.350
Ray Lutz: Which which provides not only that, but functionality in terms of doing nearest neighbor

666
01:42:43.980 --> 01:42:48.810
Ray Lutz: searches. So you can say, I want to look in end. Space. So.

667
01:42:48.810 --> 01:42:49.190
Borivoj: Good.

668
01:42:49.190 --> 01:42:53.240
Ray Lutz: 384 is like a 384 dimensions.

669
01:42:53.430 --> 01:43:00.899
Ray Lutz: and you have a float, me, you know, being where you are, and then each one of these may be many hundreds of thousands of

670
01:43:01.020 --> 01:43:01.980
Ray Lutz: items.

671
01:43:01.980 --> 01:43:02.420
Borivoj: Yes.

672
01:43:02.420 --> 01:43:07.699
Ray Lutz: Are different, you know. I guess settings within a neural network or something right and.

673
01:43:07.700 --> 01:43:08.410
Borivoj: Okay.

674
01:43:09.110 --> 01:43:24.880
Ray Lutz: And so you want to find the nearest neighbor either. And there's usually in in Pg vector and postgres, they have 3 different ways of doing the searching. So that's 1 thing I was coming across lately of a

675
01:43:25.020 --> 01:43:34.400
Ray Lutz: I'll just let you know, in case other people may be interested in that in within your package, that they may be wanting to have these embeddings.

676
01:43:34.930 --> 01:43:39.300
Borivoj: I will. I will need to see. I need to do some research on this.

677
01:43:39.300 --> 01:43:46.650
Ray Lutz: Check out check out postgres in terms of their in terms of

678
01:43:46.650 --> 01:43:49.959
Ray Lutz: their pg, vector which I think is a 3rd party package.

679
01:43:50.580 --> 01:43:53.966
Ray Lutz: But yeah, you've done a lot. And

680
01:43:54.560 --> 01:44:02.749
Ray Lutz: I think that maybe my recommendation would be maybe to skinny down some of the things that you have in your package.

681
01:44:03.790 --> 01:44:13.529
Ray Lutz: so that you don't have so much to maintain and and allow people to implement things such as the Api. I would probably recommend

682
01:44:13.690 --> 01:44:15.829
Ray Lutz: not having that in the package.

683
01:44:16.608 --> 01:44:19.001
Ray Lutz: Because it isn't really

684
01:44:19.730 --> 01:44:24.960
Ray Lutz: Well, maybe you can come up with some sort of a a very.

685
01:44:25.220 --> 01:44:28.999
Ray Lutz: you know. Simplified version of that, or something.

686
01:44:29.000 --> 01:44:43.650
Borivoj: Yeah, I was thinking to split, to split. I was thinking to create couple of versions like 3 point O, 4 point. Oh, I don't know something like this. And to strip to just leave data frame in one version, and maybe, like, you know, in 4 point O, to add

687
01:44:43.810 --> 01:44:48.640
Borivoj: dashboarding in this one to 8 Api, something like this. Because, yeah, it's heavy.

688
01:44:49.050 --> 01:44:51.320
Borivoj: Just to test single thing. It takes.

689
01:44:51.320 --> 01:44:52.500
Ray Lutz: Heavy. Yeah.

690
01:44:52.500 --> 01:44:54.800
Borivoj: 5, 6 min to compile and then wait.

691
01:44:54.800 --> 01:45:09.770
Ray Lutz: Well, the thing is that also you're doing a lot of you're doing a lot of redoing of the SQL. Syntax, and I would wonder cause I was thinking about a similar extension to Daffodil, and my 1st

692
01:45:11.440 --> 01:45:12.550
Ray Lutz: way of

693
01:45:12.920 --> 01:45:19.254
Ray Lutz: wondering if it would be better is to just allow people to put the the SQL.

694
01:45:20.090 --> 01:45:22.350
Ray Lutz: in in a string right.

695
01:45:22.590 --> 01:45:30.577
Ray Lutz: and let them just run that. Now, once you do that, when what you're doing you may as well just use it directly, but

696
01:45:31.180 --> 01:45:31.900
Borivoj: Yeah.

697
01:45:31.900 --> 01:45:39.890
Ray Lutz: For the daffodil has a syntax that's similar to what Python normally uses, with rows and columns.

698
01:45:39.890 --> 01:45:40.450
Borivoj: Hmm.

699
01:45:40.450 --> 01:45:43.209
Ray Lutz: Indices right? And then those can be.

700
01:45:43.330 --> 01:45:49.710
Ray Lutz: Those can be strings or integers, or they can be lists of strings or lists of ranges, or whatever you want.

701
01:45:49.830 --> 01:45:54.470
Ray Lutz: And then you can select from your data frame

702
01:45:55.950 --> 01:46:05.849
Ray Lutz: and what I was looking at doing was because everything is fine with daffodil and memory. Yeah, I don't need to do any of the.

703
01:46:06.230 --> 01:46:06.660
Borivoj: Yeah, yeah.

704
01:46:06.700 --> 01:46:14.750
Ray Lutz: The the sequel commands, and also the one thing that it lets you do very well is to use python.

705
01:46:15.260 --> 01:46:26.729
Ray Lutz: You know, custom, functionality, to create new things, and that may be something that would be hard to do. I know they have. I was looking in the the documentation for data fusion where they have.

706
01:46:26.980 --> 01:46:29.370
Ray Lutz: You worry. You can create a function.

707
01:46:29.750 --> 01:46:34.199
Ray Lutz: a a custom function, right? And it's for the whole.

708
01:46:34.460 --> 01:46:40.519
Ray Lutz: So it looks like they take a whole data frame. You take one data frame, maybe 2 and create a result.

709
01:46:42.365 --> 01:46:44.200
Ray Lutz: That's yeah. Okay.

710
01:46:44.200 --> 01:46:46.889
Borivoj: It's not you want. So I think.

711
01:46:46.890 --> 01:46:48.681
Ray Lutz: No, I'm just saying that that

712
01:46:49.900 --> 01:46:55.789
Ray Lutz: with with you, if you stay in python data types. So there's a data type conversion issue

713
01:46:56.030 --> 01:46:57.960
Ray Lutz: when you go from

714
01:46:58.610 --> 01:47:05.000
Ray Lutz: from python into pandas. It takes forever. I mean to go from, you know, when you think of it.

715
01:47:05.260 --> 01:47:13.340
Ray Lutz: Basically, unless you're doing 32 call, you know, column or

716
01:47:13.460 --> 01:47:17.530
Ray Lutz: columnar functions like summing that, you know.

717
01:47:17.780 --> 01:47:25.750
Ray Lutz: Deviations, Max. Min. All of that stuff on columns that you want to do, which pandas is really fast doing.

718
01:47:27.980 --> 01:47:35.760
Ray Lutz: But you have to do 32 of them to make it worth changing from python data types into pandas.

719
01:47:35.960 --> 01:47:37.329
Ray Lutz: It takes that long.

720
01:47:37.590 --> 01:47:47.370
Ray Lutz: So what I was doing initially, I was using pandas all the time. I was just using data frames like nothing's flat. And I would. And I found that I was going in and out of them.

721
01:47:47.530 --> 01:47:52.759
Ray Lutz: And this was this was a big hog of my time, because I was time sensitive.

722
01:47:52.880 --> 01:48:01.039
Ray Lutz: And so then I was saying, well, I'm just going to stay within python data types, and it runs much better and easier in many ways and sometimes smaller, too.

723
01:48:02.660 --> 01:48:12.910
Ray Lutz: But it is something that people are used to the syn, the syntax of pandas, you know, and if you go to them, and you say I want you to use a new data frame

724
01:48:13.724 --> 01:48:18.550
Ray Lutz: methodology. And if you say it uses the same syntax as pandas. But it.

725
01:48:18.880 --> 01:48:23.150
Ray Lutz: you know, 10 times faster or whatever problem. Big problem.

726
01:48:23.150 --> 01:48:27.641
Ray Lutz: Great, that's great. You can just pop it right in. And

727
01:48:28.660 --> 01:48:37.739
Ray Lutz: So anyway. So thank you so much. I would like to continue the conversation. I'm doing a presentation, I think. In is it? March?

728
01:48:41.140 --> 01:48:42.300
Ray Lutz: On Daffodil.

729
01:48:43.014 --> 01:48:50.421
Ray Lutz: Come and take a look at that. I really like your dashboarding. I would like to somehow adopt that, and and maybe port that over to Daffodil.

730
01:48:51.774 --> 01:48:57.349
Ray Lutz: Because that's that's pretty cool, and would would be really helpful to

731
01:48:57.807 --> 01:49:05.309
Ray Lutz: and and maybe maybe it is easy if you're using a lot of stuff. That's I just don't know what those packages are that you were using.

732
01:49:05.310 --> 01:49:05.750
Borivoj: Hmm.

733
01:49:05.955 --> 01:49:08.010
Ray Lutz: But I'll take a look at your code and see.

734
01:49:08.010 --> 01:49:19.249
Borivoj: It's very easy. I am like a C plus plus programmer. It's I'm like a header file. It's very easy to go through my code. I'm not a modular guy. So you go. You find one file and you find everything you need

735
01:49:19.920 --> 01:49:24.039
Borivoj: perfect. Hey? Bora, thank you so much for your time appreciate it.

736
01:49:24.040 --> 01:49:29.959
Borivoj: Thank you for all the questions and for engagement. It was very helpful, because some things I will forget if we didn't. If you didn't.

737
01:49:29.960 --> 01:49:33.940
Ray Lutz: I like it better when people ask questions myself, because

738
01:49:34.390 --> 01:49:41.629
Ray Lutz: it's like, Oh, gee! I'm actually bouncing off something other than just names on a wall. Okay, thanks.

739
01:49:41.890 --> 01:49:47.669
Borivoj: Thank you, Ray. Thanks again. Thank you for both of you. It was even more interesting.

740
01:49:48.118 --> 01:49:51.710
Borivoj: It was 3 h long. But thank you.

741
01:49:51.710 --> 01:49:57.579
Gabor Szabo: Yeah, okay, it's fine. So thank you very much. Everyone. And goodbye.

742
01:49:58.320 --> 01:49:59.360
Borivoj: Hey, Joshua.

