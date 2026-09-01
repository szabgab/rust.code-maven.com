---
title: Add tests to an open source Rust project - xmlem
timestamp: 2026-09-01T07:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - Rust
---

## Description

Automated tests are a power multiplier as they help us make sure that changes to the code or to the environment don't have unintended changes in the results.

Therefore contributing tests to an open source project is very valuable.

It is usually also easier than contributing a feature or fixing a bug as you "only" need to know how to use the project, you don't have understand the code. At least not as deeply as you would need to make changes to the project.

During this hands on workshop you will learn how to find out what tests are missing, you will learn how to write a test and you will send a Pull-Request.

Prerequisite: You need to have an account on GitHub, you need to have your Rust tool-chain set up on your computer, you need to pick a crate, or other Rust project you'd like to contribute to.

We will do several steps. In each step:

* I'll demonstrate what to do.
* You will try to follow the what I did.
* I'll help you with any issues you might encounter.


## Links and Notes

* We had some workshop elements in this session, but basically I ended up showing how to generate a test coverage report and a mutation testing report. Then I generated a test to cover some more lines of the code-base using Google Antigravity.

* The [OSDC Rust](https://osdc.code-maven.com/rust) page where I collect the ideas how to select a project, what to contribute, and what did we do during the previous sessions.
* The [Pull Request](https://github.com/xmlem/xmlem/pull/20)


{% youtube id="KX9cGEJ1oZw" file="2026-08-31-add-tests-to-an-open-source-rust-project-xmlem.mp4" %}

## Transcription

1
00:00:00.720 --> 00:00:02.840
Saile: Meeting is being recorded.

2
00:00:02.840 --> 00:00:19.179
Gabor Szabo: So, hello, and welcome to the Code Maven YouTube channel, if you are watching this on YouTube, and welcome to the session, which is… either you call it Code Maven, that's the global name, or Rust Maven, because we are talking specifically about Rust.

3
00:00:19.310 --> 00:00:21.230
Gabor Szabo: My name is Gabor Szabo.

4
00:00:21.850 --> 00:00:40.729
Gabor Szabo: I am the host of this event, and in this time, we have lots of events. If you go to live… I'll share the screen, actually, and I'll explain. That's much easier than just speaking, so…

5
00:00:41.450 --> 00:00:50.950
Gabor Szabo: Hopefully, you can see my screen now, yeah. Okay, so here there is this website, the Live Code Maven, and I turn on the beeps, so we won't…

6
00:00:51.310 --> 00:00:53.130
Gabor Szabo: Play drone, okay.

7
00:00:53.170 --> 00:00:55.260
Gabor Szabo: So, it won't bother us.

8
00:00:55.260 --> 00:01:17.820
Gabor Szabo: Anyway, this is the Live Code Maven website, probably you're familiar with it. That's where we have the sessions. There are going to be this workshop and another workshop in Python I'm doing, and most of the others are guest speakers in various topics, so you are invited to look at them. I'll share it in the chat as well.

9
00:01:19.470 --> 00:01:27.330
Gabor Szabo: this time it's going to be a workshop, and we are going to try to contribute to some open source Rust project, and I've done this

10
00:01:27.330 --> 00:01:41.140
Gabor Szabo: quite a few of these already, and so I have this website called osdc.codemaven.com, and specifically this is the Rust page. I'm sharing this link as well in the

11
00:01:41.140 --> 00:01:53.929
Gabor Szabo: in the… in the chat, so you can visit it. In general, this open source… OSDC stands for Open Source Developer Development Conference, or course, or community, or whatever club, whatever you name it.

12
00:01:54.350 --> 00:01:57.370
Gabor Szabo: But the point here was that I had

13
00:01:57.510 --> 00:02:11.750
Gabor Szabo: quite a few sessions in which we were contributing to some open source project, and we had sessions in Rust, in Python, in Ruby, in Perl. These are the languages that I covered.

14
00:02:12.140 --> 00:02:14.150
Gabor Szabo: And here you can find notes.

15
00:02:14.150 --> 00:02:35.830
Gabor Szabo: on how to select a Rust project to contribute to, if you don't have one that is already… you want to contribute to this one specifically. You just want to find something and contribute to that project. So, various ways to pick a project. Then, I have a bunch of links, and this is now highlighted. Let's…

16
00:02:35.830 --> 00:02:49.509
Gabor Szabo: get rid of the search here. A bunch of things that you can contribute to. So, low level, low-hanging fruit, I would call, is, writing… configuring the CI.

17
00:02:50.290 --> 00:02:52.780
Gabor Szabo: I don't know, there is someone who's trying to…

18
00:02:52.780 --> 00:02:53.146
Saile: Oh, Mr.

19
00:02:53.330 --> 00:02:54.070
Gabor Szabo: Trying to speak?

20
00:02:54.070 --> 00:02:55.360
Saile: Take a time to speak!

21
00:02:55.720 --> 00:02:57.310
Antonin Malzieu Ridolfi: What side is echoing?

22
00:02:57.650 --> 00:02:59.069
Saile: What's savings and agreement.

23
00:03:00.830 --> 00:03:20.509
Gabor Szabo: Okay, if you have some comments, and you don't want to have it recorded, that's fine. Write it in the chat, and then I'll see it, and I can comment on it, and I won't even say your name, so no one will have to know that… who asked it, or… well, besides the ones who are already here.

24
00:03:22.050 --> 00:03:35.360
Gabor Szabo: So feel free to ask the questions in the chat. It's probably also better, especially if the voice is not that good, as we had some, but now. Anyway, so one of the things that you can…

25
00:03:35.360 --> 00:03:43.889
Gabor Szabo: relatively easily contribute is configuring the CI, the continuous integration. So most of the projects, especially the Rust projects, are on GitHub.

26
00:03:43.890 --> 00:04:08.219
Gabor Szabo: Because the crates I.O. is integrated with GitHub, for now, at least. You need an account on GitHub in order to upload something. So, most of the projects are on GitHub, or primarily on GitHub, and you can configure GitHub Actions, which basically will… what it will do is just run the tests whenever you put

27
00:04:08.220 --> 00:04:17.279
Gabor Szabo: out, or the author pushes out some changes. And then, of course, it's a good idea to have tests for that project, and

28
00:04:17.279 --> 00:04:26.640
Gabor Szabo: you, you won't believe how many projects are there on crates.io… on… yeah, so here.

29
00:04:27.150 --> 00:04:32.899
Gabor Szabo: wait a second, why do I have this banner here? Can I get rid of it? Can I get… oops, or…

30
00:04:33.660 --> 00:04:35.450
Gabor Szabo: I hate this when it happens.

31
00:04:36.860 --> 00:04:39.380
Gabor Szabo: Do you still see my share?

32
00:04:39.620 --> 00:04:46.250
Gabor Szabo: Or, stop share, I'll have to stop the share and share again.

33
00:04:48.090 --> 00:04:51.279
Gabor Szabo: There is some strange behavior in…

34
00:04:52.010 --> 00:04:59.070
Gabor Szabo: And then I minimize this so it won't bother us later. Okay, hopefully it's better now. So…

35
00:04:59.380 --> 00:05:02.099
Gabor Szabo: If we go to craze.io,

36
00:05:03.540 --> 00:05:20.189
Gabor Szabo: There are tons of crates here, and many of them are very good and well-tested and whatever, but there are also tons of them which are good beginner, good beginnings, without probably many tests, so you can encounter those.

37
00:05:20.190 --> 00:05:33.939
Gabor Szabo: And you can also find just projects on GitHub, which weren't even uploaded to craze.io, because they are not major enough, or I don't know, whatever reason, without,

38
00:05:34.220 --> 00:05:37.979
Gabor Szabo: without tests, or without enough tests.

39
00:05:38.010 --> 00:05:55.260
Gabor Szabo: So I thought… oh, and one couple of more things. You can run, if you are… so you can set up CI, then you can write tests, and that's the major thing we are trying to… going to try to do now, that… write some tests, and…

40
00:05:55.410 --> 00:06:12.259
Gabor Szabo: And here, Bill… and then, once you find the project that has already tests, you want to ask the question, what other tests you might want to write? So… and the way to find it out is that you can run… where is it?

41
00:06:14.900 --> 00:06:17.419
Gabor Szabo: Where is it? I don't see it now.

42
00:06:18.550 --> 00:06:21.180
Gabor Szabo: Yeah, it's here. Test coverage.

43
00:06:21.470 --> 00:06:34.359
Gabor Szabo: generate test coverage report, okay? So you can generate a test coverage report, and then from there you can see which tests are… which areas of the code were not executed while testing.

44
00:06:34.360 --> 00:06:52.169
Gabor Szabo: And that's a good idea to try to write a test to cover that area. So… and then if you have a good test coverage, then you can also go to something else which is called mutation testing, and I'm going to show you that as well, where what mutation test does

45
00:06:52.170 --> 00:07:01.770
Gabor Szabo: is takes your codebase, runs your tests. The tests are passing, okay, hopefully. If the tests are not passing, then you already know what to fix, okay? Because something is broken.

46
00:07:01.980 --> 00:07:17.209
Gabor Szabo: Either in the tests or in the code. But let's say the tests are passing, and let's say you have a very good test coverage. Now, you run the mutation test, what the mutation test does is changes your source code a little bit, maybe replacing,

47
00:07:17.450 --> 00:07:20.660
Gabor Szabo: Yeah, I'm recording. Am I not? Yeah, I'm recording.

48
00:07:21.490 --> 00:07:27.410
Gabor Szabo: Yeah, someone just right, kindly recorded. Okay, I'm recording, but when I said, I think, that I'm not…

49
00:07:27.870 --> 00:07:31.680
Gabor Szabo: Sure, what I'm going to do is this. Anyway, if I'm going to upload it.

50
00:07:31.690 --> 00:07:50.930
Gabor Szabo: Anyway, so you have the code base, and then the mutation testing changes your codebase a little bit. Let's say a plus sign replaces by the minus sign, okay? So everything is the same, except that in the computation, somewhere, a plus is replaced by the minus. And then runs the tests.

51
00:07:50.930 --> 00:07:57.990
Gabor Szabo: And the test… are expected to fail, right? Because now the code base changed. If the tests don't

52
00:07:58.360 --> 00:08:03.770
Gabor Szabo: didn't fail, okay? If the no test failed, that means that the test

53
00:08:03.940 --> 00:08:22.399
Gabor Szabo: are not properly verifying that area of the code. It might be that they are not covering at all, that the test coverage should have found it, okay? Or it might be that, you know, I've seen a codebase or tests where you have a

54
00:08:22.450 --> 00:08:25.580
Gabor Szabo: You run a tab, you run a function, You…

55
00:08:25.730 --> 00:08:41.219
Gabor Szabo: compute the result, but then you're in the test, but then you're not comparing it to some expected, result. Now, how this happens, I have theories, but probably it's just a mistake that the,

56
00:08:41.559 --> 00:08:48.489
Gabor Szabo: developer-made. Sometimes I saw code like this, that people, instead of verifying

57
00:08:48.530 --> 00:09:03.659
Gabor Szabo: with an assert, comparing to an expected result, instead of that printing out the result, and eyeballing it. So, looking at it and saying, okay, it's a good value. And that's how they think tests are.

58
00:09:03.930 --> 00:09:21.049
Gabor Szabo: this is… well, as long as they look at the value, it's good, but it means that they're not really verifying in the test that the value is correct. You have to compare it to some expected result. And so,

59
00:09:21.320 --> 00:09:22.989
Gabor Szabo: the mutation tests.

60
00:09:23.160 --> 00:09:30.960
Gabor Szabo: Can… will find these problems, because they will change the code, the code will still run, it will still generate some result.

61
00:09:31.000 --> 00:09:47.000
Gabor Szabo: And the code and the test will pass, because no one is… because it's not comparing to some expected result, which is different from what is actually generated. And so, if you go further on this page, scroll further down, you will find…

62
00:09:47.360 --> 00:09:55.660
Gabor Szabo: This… let's skip the Rust Digger for now, because I stopped running it. A bunch of projects that I hand… that I…

63
00:09:55.660 --> 00:10:12.010
Gabor Szabo: dealt with, and then you can see the dates when I dealt with on what session was there, and the links to the issues I opened, and the status as I… as I updated it, because it's not automatically updated. So let's say this was a pull request.

64
00:10:12.010 --> 00:10:26.189
Gabor Szabo: adding GitHub workflow, and it was merged. And this was an issue I opened, and it's still waiting for some response. And, there was a PR that I opened, and it was rejected without any comment.

65
00:10:26.190 --> 00:10:42.880
Gabor Szabo: And so, for example, this project, I will probably not contribute anymore, because there were, like, 4 PRs, 3 PRs, and they were all rejected, closed, basically, without any comments. Now, maybe the author implemented the same thing.

66
00:10:43.690 --> 00:10:48.389
Gabor Szabo: Himself, herself, whatever. But,

67
00:10:48.950 --> 00:11:01.769
Gabor Szabo: my PR was sort of rejected, so I would say, okay, I don't want to contribute to this person and to this project anymore, because my contribution is not really valued, okay?

68
00:11:01.810 --> 00:11:07.259
Gabor Szabo: Of course, not every rejection should

69
00:11:07.260 --> 00:11:22.669
Gabor Szabo: generate this reaction, but if you feel… and there's even a video recording of that, okay? Of that session, when I was doing this. So you have plenty of video recordings you might find here.

70
00:11:22.840 --> 00:11:37.720
Gabor Szabo: Anyway, there is a CSV create, which is a very… I don't know if it's popular, but it's, the author is a very well-known author, so it's a very good, create. And so…

71
00:11:37.720 --> 00:11:50.609
Gabor Szabo: I have it locally, and I wanted to show that one. So these were the… because it's a big one, and I'm going to get stuck with this, and so I will just show

72
00:11:50.740 --> 00:11:56.620
Gabor Szabo: That even there, we might have… even… even there, in such a,

73
00:11:57.100 --> 00:12:09.550
Gabor Szabo: well-maintained project, you might find issues. Now, you can see that here, too, I have to check whether they are still waiting, or they were… what happened to these… all the other things.

74
00:12:10.020 --> 00:12:12.280
Gabor Szabo: But you can see the comments I made.

75
00:12:12.840 --> 00:12:21.750
Gabor Szabo: So, I have… here, the Rust CSV project cloned.

76
00:12:22.020 --> 00:12:25.469
Gabor Szabo: So, I can come to this link.

77
00:12:25.740 --> 00:12:28.330
Gabor Szabo: Just to go to a whole,

78
00:12:29.320 --> 00:12:35.080
Gabor Szabo: I can close this one, I can close this one for now. This is the same thing.

79
00:12:35.570 --> 00:12:49.500
Gabor Szabo: This is something that I don't need for now. Oh, and I have the clock. Okay, so this is where the crate is. Andrew Gallant, also known as Burned Sushi.

80
00:12:49.730 --> 00:12:58.430
Gabor Szabo: is the author, or the maintainer, I don't know. I can click on the reposit… on the link to the repository. Most crates have that link.

81
00:12:59.070 --> 00:13:10.720
Gabor Szabo: I think not all of them, but most of them have. And then I took this one, and I cloned it with git clone. And then, so now I have… if I run it Git…

82
00:13:10.750 --> 00:13:27.110
Gabor Szabo: remote minus 3, I have the origin repo, which is from Burn Sushi, so that's the original one, and I have my own fork, this is my username on GitHub. And git pull…

83
00:13:27.140 --> 00:13:37.749
Gabor Szabo: origin master will update from the origin… from the origin. I think it's up to date because I just ran this 5 minutes before we started the session.

84
00:13:37.890 --> 00:13:56.020
Gabor Szabo: And now I can just basically run cargo test here, and it will run the test. And it's already compiled, because I read it earlier. And it was… it's pretty fast. Now, before I continue, I have this other window, and I wanted to run HTOP here.

85
00:13:56.290 --> 00:14:03.729
Gabor Szabo: So, we'll see… how much of my CPU is used?

86
00:14:03.970 --> 00:14:13.320
Gabor Szabo: So I have, like, 16 cores here, and it's not… not that bad. I mean, remember your Zoom used to take up a lot more. I think it's now…

87
00:14:13.570 --> 00:14:28.529
Gabor Szabo: better. So it has a load average of 3, as you can see here, which is not that bad. So I can go now to this page, and

88
00:14:31.290 --> 00:14:39.619
Gabor Szabo: find, try to find, because I never remember the commands, test coverage report using tarpaulin. Okay, so this is the link where

89
00:14:39.770 --> 00:14:52.779
Gabor Szabo: I wrote a blog post, basically, or it's a book, or whatever, there, because I don't remember how the comments are, I just write them in books or blog posts, and then I can find them easier.

90
00:14:52.780 --> 00:15:02.919
Gabor Szabo: So, cargo install, cargo tarpaulin would install this thing, this tool, which generates the test report.

91
00:15:03.570 --> 00:15:18.749
Gabor Szabo: And then you can run it in various flags. So here, this one will run cargo tarpauline HTML, that will generate an HTML report, this is a standard output report, and this will do both.

92
00:15:19.340 --> 00:15:26.129
Gabor Szabo: So… And there is a bug here in the slide. Anyway, I take this one.

93
00:15:26.420 --> 00:15:41.349
Gabor Szabo: and copy it, and run this command, okay? So now what it does, it runs the tests again, but this time it will collect information, which,

94
00:15:42.170 --> 00:15:45.800
Gabor Szabo: Lines of the code base.

95
00:15:46.080 --> 00:15:50.180
Gabor Szabo: were executed while the tests were running, and

96
00:15:52.320 --> 00:16:01.170
Gabor Szabo: Something, oh, broken? Test error? Test failed during run? What does that mean? Is this happening earlier?

97
00:16:01.540 --> 00:16:03.750
Gabor Szabo: Oh, there was a failure already!

98
00:16:03.940 --> 00:16:05.719
Gabor Szabo: Huh, I didn't notice.

99
00:16:08.370 --> 00:16:16.460
Gabor Szabo: Okay, I think it's… it… Okay, now I'm getting confused. I think it was… it was passing earlier, no?

100
00:16:16.600 --> 00:16:17.950
Gabor Szabo: Field, field?

101
00:16:30.130 --> 00:16:34.230
Gabor Szabo: Okay, now I'm sorry, I'll get… I ran the Garbo test again.

102
00:16:37.880 --> 00:16:40.339
Gabor Szabo: Wasn't it passing? Anyone remembers?

103
00:16:40.510 --> 00:16:43.010
Gabor Szabo: This is what happens when you do live coding.

104
00:16:47.910 --> 00:16:52.620
Gabor Szabo: So it… okay, I think I remember… I think specifically…

105
00:16:52.850 --> 00:17:02.049
Gabor Szabo: Some modules… so, okay, the tests are passing, but if I run cargo tarpaulin with this, I hope I pronounced properly.

106
00:17:02.430 --> 00:17:03.639
Gabor Szabo: First of all.

107
00:17:03.850 --> 00:17:15.640
Gabor Szabo: you see that it needs to recompile it, because it's working in a different way, which is a pretty… a little bit annoying. And now we can see that while it's running.

108
00:17:16.730 --> 00:17:30.909
Gabor Szabo: it has all kinds of things that are… get broken, okay? So, I know that tarpaulin doesn't work with every codebase, okay? There are issues. Now, on the other hand.

109
00:17:32.160 --> 00:17:39.490
Gabor Szabo: I think… This might… I mean, I don't know if this worked. I can work… use this one.

110
00:17:39.660 --> 00:17:41.619
Gabor Szabo: Let's try this one as well.

111
00:17:42.730 --> 00:17:46.520
Gabor Szabo: So, I… usually, what I do is I…

112
00:17:46.740 --> 00:18:00.930
Gabor Szabo: run everything inside a Docker container in order to make sure that whatever this random codebase, basically, does, okay, won't impact all of my computer.

113
00:18:00.940 --> 00:18:09.630
Gabor Szabo: Now, what you can see is that I'm running them… running it on… natively on my computer, which is on one hand scary,

114
00:18:09.980 --> 00:18:17.950
Gabor Szabo: On the other hand, it doesn't work either. So, bad example, okay, or one example that,

115
00:18:18.350 --> 00:18:35.279
Gabor Szabo: I can't show you the result for now. I will pick another module, another example, another create another example, but for… while we are… before we do this, I wanted to show one more thing, and this is the mutation testing, so…

116
00:18:37.650 --> 00:18:39.679
Gabor Szabo: No, this, not this one?

117
00:18:41.690 --> 00:18:54.930
Gabor Szabo: This one is my article, okay? So here you can find that this is also… you install it with cargo install… not that tarpaulin… cargo install, cargo mutants, and then you run cargo mutants.

118
00:18:55.180 --> 00:18:58.609
Gabor Szabo: That's it. There… you can find this article, I'll…

119
00:18:59.660 --> 00:19:13.159
Gabor Szabo: Paste it there. So this one now generates the mutants. It found… it generated 758 mutants, meaning in 758 different ways, it changed the codebase.

120
00:19:13.500 --> 00:19:20.509
Gabor Szabo: And it will run the tests on all Run all the tests, 758…

121
00:19:21.470 --> 00:19:35.950
Gabor Szabo: times. If we… if I switch to this screen now, you can see that it's, like, all of the… all of my cores are fully used, almost fully used, okay? The load average is now 12 already.

122
00:19:36.230 --> 00:19:39.450
Gabor Szabo: So… My computer is pretty, pretty busy.

123
00:19:39.570 --> 00:19:43.799
Gabor Szabo: And… when I ran this before the…

124
00:19:44.140 --> 00:19:57.340
Gabor Szabo: with the session, I actually stopped it pretty early, because I found this, what you can see also here. So, I found that there are syncs which are missed.

125
00:19:57.380 --> 00:20:05.139
Gabor Szabo: So, I think when it says missed, it means that it found a case where if you make some changes.

126
00:20:05.620 --> 00:20:09.560
Gabor Szabo: Then the still… the test will pass.

127
00:20:09.920 --> 00:20:16.659
Gabor Szabo: Okay? Unfortunately, I couldn't find how to…

128
00:20:16.860 --> 00:20:30.749
Gabor Szabo: limit the number… how to tell it to stop after the first missed one, okay? If anyone… so I'm going to stop it now, and what I promised is that I share this link as well, in case you…

129
00:20:30.790 --> 00:20:38.099
Gabor Szabo: want to… want to find it easily, but, you can find it from the… from the OSDC page.

130
00:20:38.760 --> 00:20:40.110
Gabor Szabo: Okay, so…

131
00:20:40.270 --> 00:20:48.370
Gabor Szabo: So that's… that's it. Now let me try to find another crate, and hopefully there I will be able to show, better.

132
00:20:49.360 --> 00:20:51.650
Gabor Szabo: what I wanted to… to show.

133
00:20:51.790 --> 00:20:52.940
Gabor Szabo: Damn.

134
00:20:53.790 --> 00:21:08.990
Gabor Szabo: But, wait a second, sorry, before that, I wanted to finish this one, this thought. So I said, let's get the help, and I even piped through less. I couldn't find a way to limit the number of mutants

135
00:21:08.990 --> 00:21:19.790
Gabor Szabo: probably I need to read the documentation for this, or ask ChatGTP, or how to do this, so it will stop the first… on the first missed, mutant.

136
00:21:19.990 --> 00:21:29.200
Gabor Szabo: Git status, mutants out, this is the…

137
00:21:29.390 --> 00:21:49.200
Gabor Szabo: I think it's totally different than what I remember. In any case, in this article that I shared with you, you can find an example that I used on the XMLM rust grid, and we can look at it now, maybe it will work for us now as well, where,

138
00:21:49.820 --> 00:21:58.780
Gabor Szabo: there were quite a few things that were missed, and then I made some improvements, so maybe we can do this again.

139
00:21:59.020 --> 00:22:03.999
Gabor Szabo: X… Let's see what happens here.

140
00:22:08.820 --> 00:22:10.920
Gabor Szabo: Okay, it's not here.

141
00:22:13.620 --> 00:22:16.509
Gabor Szabo: It's here, okay, git status.

142
00:22:16.800 --> 00:22:24.540
Gabor Szabo: Okay, so… I can remove all… everything from here. This is the other project out.

143
00:22:24.940 --> 00:22:27.210
Gabor Szabo: And… Ode?

144
00:22:27.360 --> 00:22:30.540
Gabor Szabo: And the tarpaulin report. Okay, let's clean this.

145
00:22:30.840 --> 00:22:31.710
Gabor Szabo: Git?

146
00:22:33.430 --> 00:22:35.500
Gabor Szabo: Remote?

147
00:22:35.670 --> 00:22:45.000
Gabor Szabo: So here, too, I have two remotes, the origin, which is where it came from, and my fork. So let's see this kit.

148
00:22:45.260 --> 00:22:47.200
Gabor Szabo: Pool Origin.

149
00:22:48.150 --> 00:22:49.740
Gabor Szabo: main here.

150
00:22:51.730 --> 00:22:56.729
Gabor Szabo: Just to make sure that it's up to date. Okay, so it's up to date, and now…

151
00:22:56.990 --> 00:23:07.110
Gabor Szabo: Because, sort of, I say, okay, I trust less this crate, I'm going to show you the one with the docker. So I have this alias.

152
00:23:09.560 --> 00:23:13.059
Gabor Szabo: DRROST, which is basically this command.

153
00:23:13.430 --> 00:23:19.049
Gabor Szabo: And I'm copy-pasting it for you, so you can… in case you would like to use this.

154
00:23:21.470 --> 00:23:27.670
Gabor Szabo: So I'm running this command. What it does, it runs, this, image.

155
00:23:28.210 --> 00:23:33.939
Gabor Szabo: this Docker image, it runs Bash inside it, it maps my directories.

156
00:23:34.050 --> 00:23:37.839
Gabor Szabo: And it also maps various other folders.

157
00:23:37.840 --> 00:23:58.590
Gabor Szabo: Because this Docker image, this is an image I created, this Docker image also contains installations of Copilot, of Codex, of Gemini, Gemini, whatever, how you say it. So if you have a configuration, you can use Dolos, because the clients are installed. So I say DR Rust.

158
00:23:59.480 --> 00:24:05.079
Gabor Szabo: And it copies it because I've removed the previous image.

159
00:24:06.240 --> 00:24:12.690
Gabor Szabo: The idea here is that I can Run inside this,

160
00:24:13.000 --> 00:24:27.709
Gabor Szabo: container, inside this Docker container, I can run any code, and it will only impact the folders that were mapped here, which is still too much, I think, because still these, they are mapping… all these folders are accessible to it.

161
00:24:27.710 --> 00:24:36.500
Gabor Szabo: But, at least it won't be able to access all of my computer, just these few folders, and…

162
00:24:36.790 --> 00:24:39.660
Gabor Szabo: The current folder of the current project.

163
00:24:40.040 --> 00:24:45.600
Gabor Szabo: And that's good, because I can also run any of the…

164
00:24:46.450 --> 00:25:03.819
Gabor Szabo: LM agents, LM coding agents, and I can let them do whatever they want, because they can, again, can't just randomly remove my hard disk or do all kinds of changes. They can only do this. So, okay, we are fine. Inside is this,

165
00:25:03.820 --> 00:25:15.280
Gabor Szabo: container in the slash OPT folder, but if I look at what's here, this is the same project that we saw earlier. Now I can run cargo test.

166
00:25:15.350 --> 00:25:19.460
Gabor Szabo: And, see what's going on, if it's a passive high.

167
00:25:24.600 --> 00:25:30.460
Gabor Szabo: is… is Rust Analyzer also running in the Docker container? So I'm not using it,

168
00:25:30.590 --> 00:25:50.080
Gabor Szabo: I mean, I don't think that I use Rust Analyzer here, and I'm not using an IDE. I think that's what you are going… referring to, that if I use StartCopilot, right, then… not cop… not Copilot, sorry, VS Code, or some IDE, that is using,

169
00:25:50.180 --> 00:25:58.650
Gabor Szabo: the… no, so if I start now VS Code, it's not configured to use the Docker container. That's,

170
00:25:58.770 --> 00:26:08.240
Gabor Szabo: something is lacking from my setup yet, okay? So that would use the… the… Run it on my computer.

171
00:26:08.880 --> 00:26:17.079
Gabor Szabo: Which is problematic, again, because it runs the code. But, yeah, well, whatever.

172
00:26:17.210 --> 00:26:18.670
Gabor Szabo: when I run…

173
00:26:19.010 --> 00:26:29.520
Gabor Szabo: the agents, then I don't even open the IDE, looking at the source code. I'm just telling the agents… I'll show you, probably. Let's see if I can get there, but maybe…

174
00:26:29.820 --> 00:26:36.629
Gabor Szabo: Well, what time is it? Anyway, test… the test passed. I want… I already want to… you to do something, so…

175
00:26:36.760 --> 00:26:44.969
Gabor Szabo: I just… this managed to do, the test passed, okay, cargo test. Now I go back to this,

176
00:26:45.390 --> 00:26:46.930
Gabor Szabo: The tarpauling?

177
00:26:47.460 --> 00:26:49.500
Gabor Szabo: Darba-ulin?

178
00:26:49.800 --> 00:26:51.700
Gabor Szabo: Command, this one…

179
00:26:51.990 --> 00:27:05.689
Gabor Szabo: So, this one is running, the test coverage report, generating the test coverage report. Hopefully, this will work.

180
00:27:06.380 --> 00:27:10.680
Gabor Szabo: In this crit. This is a much, much simpler crit, so…

181
00:27:11.540 --> 00:27:29.830
Gabor Szabo: here… oh, okay, sorry. So this problem is what is described in… on this page, that I get this ASLR disable fail whatever error, and that's where I need to add this one. So basically, the tarpaulin has

182
00:27:29.830 --> 00:27:34.199
Gabor Szabo: two, I think, two engines on how to,

183
00:27:34.620 --> 00:27:42.779
Gabor Szabo: monitor the source code, basically, how to monitor the execution, and one of them doesn't work inside the container. And the other one, which is

184
00:27:44.260 --> 00:27:49.879
Gabor Szabo: probably not so good, or I don't know what that's good, I don't… I'm not sure.

185
00:27:50.360 --> 00:28:08.469
Gabor Szabo: uses this flag, and… and I need to use that when I'm using inside a container. Anyway, now, this, what you can see here, is the out… is the report to the standard output, which says on which… on the source code, which line of code,

186
00:28:10.980 --> 00:28:30.650
Gabor Szabo: what is covered or uncovered, what is not covered. Okay, uncovered? Why is it uncovered? I don't know, it should be not covered, probably. But I'm not an English native, so I don't know. And it also created this file in OPT tarpaulin report. Now, this is, let me open another tab here.

187
00:28:32.400 --> 00:28:51.080
Gabor Szabo: Though it's inside… inside the container, it says slash OPT, tarpaulin report, but the slash OPT is mapped to the outside folder, so if I visit the same folder outside its status, then you can find this page here, and then I can open it

188
00:28:51.680 --> 00:28:56.960
Gabor Szabo: This X is just an alias for me to open a… whatever you open.

189
00:28:58.630 --> 00:29:02.799
Gabor Szabo: And so this is the HTML file that I opened.

190
00:29:03.330 --> 00:29:17.080
Gabor Szabo: And you can see, the source code, and you can see that the display file has a very… a rather good test coverage, 85%. It can be improved, but

191
00:29:17.080 --> 00:29:24.870
Gabor Szabo: It is, as it is, and now you can see which functions were not covered by tests.

192
00:29:26.140 --> 00:29:28.440
Gabor Szabo: Okay? So…

193
00:29:29.530 --> 00:29:37.000
Gabor Szabo: So now, we could go and write a test, or read a little bit more of the source code, and write a test. If I go back here.

194
00:29:37.510 --> 00:29:45.050
Gabor Szabo: back this one, and so there is… the other… other files are way less covered. The yellow… the yellow ones…

195
00:29:45.590 --> 00:29:46.750
Gabor Szabo: I guess…

196
00:29:47.070 --> 00:29:58.489
Gabor Szabo: different ranges, and then there is the select RS, which is really poorly tested, okay? Now, I… I don't know that well this crate, I…

197
00:29:58.560 --> 00:30:03.530
Gabor Szabo: Basically found it in a random, more or less random way,

198
00:30:03.530 --> 00:30:12.659
Gabor Szabo: a couple of months ago, when I dealt with it. But that's one thing that you can… you can do, and you should do.

199
00:30:12.660 --> 00:30:24.670
Gabor Szabo: And I think that's it for now. Let's stop… let me stop here. Maybe I even stop the recording, for now, and then you,

200
00:30:26.370 --> 00:30:45.600
Gabor Szabo: you need to do all these things, okay? So you can also do this, and I will be able to help you. And I'm not recording it so no one needs to worry about the recording. So you don't need to use the Docker container, you can use it locally. Before…

201
00:30:46.340 --> 00:31:03.419
Gabor Szabo: I let you start doing it one more thing. So, either you already have some crate that you want to deal with, okay? You can go with some crate that you want to play with, but the… probably most of you will pick some big crate.

202
00:31:03.520 --> 00:31:18.780
Gabor Szabo: well-known crate. I have another proposition. Go with a semi… here is what the… at the top of the OSDC page. I said, a semi-popular, recently updated Rust project. So if you click on this link.

203
00:31:19.140 --> 00:31:27.670
Gabor Szabo: Well, I have to fix the link because it's an old link. It says it's a… it goes to GitHub, and it searches for

204
00:31:28.030 --> 00:31:39.440
Gabor Szabo: projects where the language is Rust, as you can see. It has between a hundred and a thousand stars. This is an arbitrary

205
00:31:40.380 --> 00:31:47.940
Gabor Szabo: range for saying semi-popular, okay? So it's not very popular, but it's not also not some beginner thing.

206
00:31:47.940 --> 00:31:59.760
Gabor Szabo: And it was changed recently. Now, I updated that file, apparently, in October last year, so it's almost a year ago, so you can come here and change the date and say, okay, let's say

207
00:31:59.760 --> 00:32:08.729
Gabor Szabo: 08, so it's a lot… it was changed in the last months, and let's say you say you want to between 10 and 100 stars.

208
00:32:08.730 --> 00:32:19.450
Gabor Szabo: Okay? So you go here, and then you find a bunch of crates which are not totally… so on one hand, not totally,

209
00:32:23.580 --> 00:32:35.240
Gabor Szabo: uninteresting, or un… or new, or whatever can I say on… so it has some stars, okay? It doesn't have a lot of stars, and the point of recently changed

210
00:32:35.380 --> 00:32:39.270
Gabor Szabo: recently pushed here, is that if there is a

211
00:32:39.470 --> 00:32:48.279
Gabor Szabo: project that hasn't been changed for, I don't know, half a year, that… very unlikely that any changes you make will

212
00:32:48.280 --> 00:32:51.509
Gabor Szabo: Will be accepted and integrated.

213
00:32:51.510 --> 00:33:16.380
Gabor Szabo: So if you are… if you care about the fact also that your work will be accepted, then probably it's a good idea, and you're not… you don't have a specific module that creates that you want to contribute to, then probably it's a good idea to pick one that has been changed recently, because then it… the likelihood that it will… that someone will look at your changes

214
00:33:18.530 --> 00:33:28.459
Gabor Szabo: is higher. So, I can share this link as well, and we can try this one. Okay? So, I'm,

215
00:33:28.980 --> 00:33:34.729
Gabor Szabo: Stopping the share now, and then posing the video, and then giving you…

216
00:33:34.980 --> 00:33:46.419
Gabor Szabo: I don't know, 5 minutes, 10 minutes, maybe, I'll give you to do this, okay? So what your task is, is to find the crate.

217
00:33:46.940 --> 00:33:50.400
Gabor Szabo: Whether you find it a random one, or one that you already

218
00:33:50.570 --> 00:34:01.559
Gabor Szabo: came with, because you want to contribute to, clone it, okay, to your machine, and run the, run the tests, and run the,

219
00:34:01.840 --> 00:34:07.370
Gabor Szabo: test coverage report, and then show it to us, okay? So we can see it.

220
00:34:07.720 --> 00:34:13.979
Gabor Szabo: Emm… So, let me find the button to stop, pause the recording.

221
00:34:14.909 --> 00:34:16.709
Gabor Szabo: Pause recording, okay.

222
00:34:19.100 --> 00:34:33.349
Gabor Szabo: Okay, so now we continue. Hopefully, those who are watching the video might have stopped it and did some of the work already. And I would like to just run on the same codebase, the…

223
00:34:33.510 --> 00:34:46.270
Gabor Szabo: the other tool, the mutation testing that I mentioned. So this is the cargo install mutant, and that's it. I… I don't remember how to run these things, even though they are so simple.

224
00:34:48.199 --> 00:34:50.740
Gabor Szabo: Okay, that's not good.

225
00:34:52.010 --> 00:34:56.419
Gabor Szabo: I thought it was installed in the container… in the image.

226
00:34:56.699 --> 00:35:00.560
Gabor Szabo: But apparently not, so now I'm installing it.

227
00:35:07.160 --> 00:35:10.430
Gabor Szabo: And then, let's use the opportunity.

228
00:35:12.350 --> 00:35:15.700
Gabor Szabo: to go to… GitHub?

229
00:35:17.800 --> 00:35:19.070
Gabor Szabo: Sub-gub?

230
00:35:20.720 --> 00:35:22.590
Gabor Szabo: And, Rust…

231
00:35:27.450 --> 00:35:30.089
Gabor Szabo: Docker on Ubuntu, okay.

232
00:35:31.780 --> 00:35:36.300
Gabor Szabo: Dockerfile, okay, I'm going to enlarge the font, probably, it's a good idea.

233
00:35:36.550 --> 00:35:38.100
Gabor Szabo: Dockerfile…

234
00:35:43.550 --> 00:35:44.510
Gabor Szabo: Utah?

235
00:35:47.810 --> 00:35:49.400
Gabor Szabo: Interesting, you see?

236
00:35:50.020 --> 00:35:51.309
Gabor Szabo: It's not there.

237
00:35:53.350 --> 00:36:04.439
Gabor Szabo: Okay, so this is the file, okay, just to explain you, nothing to do really with the project. This is the file, the Docker file, that generates that Docker image.

238
00:36:09.370 --> 00:36:13.650
Gabor Szabo: And so I'm going to add… this one.

239
00:36:22.670 --> 00:36:24.370
Gabor Szabo: That's it, right? Yeah.

240
00:36:27.450 --> 00:36:37.000
Gabor Szabo: Strange, I thought that I already added it. Okay, so git add docker file, git… edit… mute.

241
00:36:40.320 --> 00:36:42.350
Gabor Szabo: Mutant… okay.

242
00:36:43.770 --> 00:36:47.009
Gabor Szabo: So… that's it, basically. Oh, no.

243
00:36:52.000 --> 00:36:54.460
Gabor Szabo: It probably…

244
00:36:58.100 --> 00:36:59.739
Gabor Szabo: I updated something there.

245
00:37:00.800 --> 00:37:18.530
Gabor Szabo: I can even take a look at it. It was the dependable that updated something in the GitHub Actions, okay? That's why I had to pull it. Anyway, I updated the Docker configuration, so the next time it builds the image, which is now, and I will already have the

246
00:37:18.590 --> 00:37:29.880
Gabor Szabo: this tool also inside, so the mutants, which apparently I didn't have. Okay, so now I'm supposed to be able to run this command as well inside the container.

247
00:37:30.610 --> 00:37:36.020
Gabor Szabo: And because it's a very much smaller crate.

248
00:37:36.020 --> 00:37:51.670
Gabor Szabo: It still found 350 mutants, but the tests are running, I think, much faster. But I still… there are a couple of missed things, so let me try to remember how it helped, how it was. So this is the…

249
00:37:54.020 --> 00:38:01.070
Gabor Szabo: This was when… before I ran the… I contributed, I think. Then I com… after I…

250
00:38:01.490 --> 00:38:07.420
Gabor Szabo: So I found this crate, and then I run these tools, and I found that there are, like.

251
00:38:07.710 --> 00:38:14.159
Gabor Szabo: 600… 165… Mutants that were, failing.

252
00:38:14.330 --> 00:38:23.280
Gabor Szabo: or missed, and that was exactly the story that I told you, that there were these, this,

253
00:38:23.600 --> 00:38:35.659
Gabor Szabo: This is the change, so it had these print statements inside. Doing some calculation, or doing some work, generating this doc, whatever it is, doesn't matter, it's a string.

254
00:38:35.800 --> 00:38:40.960
Gabor Szabo: And then just printing it out. That was the code, that was the test.

255
00:38:41.090 --> 00:38:52.549
Gabor Szabo: And so I… I looked at what it prints out, I said, okay, hopefully, whatever it prints out is the right thing. I mean, I said, I mean, I don't really know, I don't…

256
00:38:52.780 --> 00:39:06.159
Gabor Szabo: really know what it's supposed to be, but let's… let's assume that whatever it is does now works, is the correct one. So, I think I even commented in the… in the comment that I hadn't

257
00:39:06.160 --> 00:39:15.670
Gabor Szabo: verified that whatever we are now testing is correct, I just… testing whatever it… so the regression part of it.

258
00:39:15.840 --> 00:39:22.499
Gabor Szabo: So, I replace the print statement with an assert statement, saying, okay, this is what we are expecting

259
00:39:22.780 --> 00:39:37.749
Gabor Szabo: this dock to be. And that's it. And then I ran the cargo mutants, and it went down from 165 missed to 141 missed, out of the 337 mutants. Now.

260
00:39:37.790 --> 00:39:44.169
Gabor Szabo: I don't know what happened since then. There, I mean, since I made that pull request.

261
00:39:45.370 --> 00:39:51.179
Gabor Szabo: Yeah, I made that pull request somewhere, probably there is a link here to the pull request.

262
00:39:53.030 --> 00:39:56.629
Gabor Szabo: Since now that I run it, it found…

263
00:39:56.880 --> 00:40:12.649
Gabor Szabo: It scripts… 350, mutants. It's saying at the bottom, last row, I don't know if you can see it, I hope that you can see it. It says it's now running the… which mutant it's testing, but it's,

264
00:40:13.400 --> 00:40:17.550
Gabor Szabo: And so far, this is the result. Okay?

265
00:40:25.440 --> 00:40:27.040
Gabor Szabo: So, I mean…

266
00:40:27.150 --> 00:40:40.310
Gabor Szabo: This is not very surprising, okay? I probably… there is no… not a lot of point, not really good reason to run the mutation test on this grade, because we already know that there are so many

267
00:40:40.630 --> 00:40:42.580
Gabor Szabo: There's…

268
00:40:42.660 --> 00:40:52.099
Gabor Szabo: such a huge percentage of the codebase is not tested. That's what the tarpaulin, the tarpaulin, the test coverage report, revealed.

269
00:40:52.110 --> 00:41:01.990
Gabor Szabo: So, obviously, if you make any changes in a code, part of the code that is not tested, the test, it won't impact the tests, okay? The test won't fail. So…

270
00:41:01.990 --> 00:41:13.810
Gabor Szabo: Most of these things that we can see here are probably parts of the code that were never tested. So the first thing that we should do now is write some tests for those parts.

271
00:41:13.960 --> 00:41:23.009
Gabor Szabo: Okay, so I think I'm going to even stop this, because, it's just running and doing…

272
00:41:23.490 --> 00:41:26.109
Gabor Szabo: If you'll give us a report, okay, whatever.

273
00:41:26.300 --> 00:41:33.190
Gabor Szabo: Instead of that, let me try to do some… let me try to do something else. I run…

274
00:41:33.720 --> 00:41:36.539
Gabor Szabo: AGY, which is the…

275
00:41:40.240 --> 00:41:48.040
Gabor Szabo: which is… what is it called? The anti-gravity, okay, so the Gemini tool.

276
00:41:54.040 --> 00:41:57.790
Gabor Szabo: Okay, so there is a question. Some tests in this codebase

277
00:41:57.940 --> 00:42:06.759
Gabor Szabo: are running for too long. Is there a way to ignore specific tests instead of just removing them?

278
00:42:07.190 --> 00:42:25.900
Gabor Szabo: I think… I mean, yeah, running the normal test running, definitely, you have… you can filter out tests. I'm not sure if you're using tarpaulin or this tool, how to do this, but cargo mutants.

279
00:42:26.120 --> 00:42:27.130
Gabor Szabo: Help?

280
00:42:27.630 --> 00:42:28.640
Gabor Szabo: Bless?

281
00:42:28.950 --> 00:42:33.710
Gabor Szabo: Exclude… here it is.

282
00:42:38.600 --> 00:42:39.690
Gabor Szabo: Okay.

283
00:42:41.730 --> 00:42:47.109
Gabor Szabo: I'm not sure if what the… what is it excluding, whether it's excluding the files.

284
00:42:49.240 --> 00:42:58.350
Gabor Szabo: which ones to change, or excluding, tests from… I know, probably the… the…

285
00:43:00.130 --> 00:43:03.479
Gabor Szabo: Wait a second. Time out. Time out.

286
00:43:03.620 --> 00:43:13.720
Gabor Szabo: That I remember. So you can set, what are… what should be the timeout. Minimum test timeout, okay?

287
00:43:16.180 --> 00:43:26.200
Gabor Szabo: or you can set a timeout, whatever, maximum runtime for all cargo commands. Okay, not minimum test time, I don't know. Timeout multiplier…

288
00:43:27.050 --> 00:43:30.809
Gabor Szabo: I don't know, okay? Let's see, cargo test.

289
00:43:45.310 --> 00:43:55.649
Gabor Szabo: So, here's exclude packages, and which test to run. So, with the test, you can configure which ones… you can tell it which one to run and which not.

290
00:43:58.190 --> 00:44:13.890
Gabor Szabo: Yeah, I don't know… I don't know how to tell the other tools which test to run when you're running. And I'm not even sure that that's a good idea. I mean, if test coverage… if you care about test coverage, we want to know what is the general test coverage.

291
00:44:13.890 --> 00:44:22.199
Gabor Szabo: I mean, depending on what you're testing. For example, I have a codebase where I have various subsystems.

292
00:44:22.410 --> 00:44:24.209
Gabor Szabo: And I have passed.

293
00:44:24.360 --> 00:44:31.239
Gabor Szabo: Specifically for those subsystems. And then there are generic, more integration tests, let's say.

294
00:44:31.300 --> 00:44:35.230
Gabor Szabo: And so, sometimes I want to run test coverage report.

295
00:44:35.230 --> 00:44:59.739
Gabor Szabo: running only the tests of the specific subsystem to see how well they are actually covering that subsystem, without the integration tests that are also covering those subsystems, and might cover some parts of it that are the more, like, unit tests are not covering. And let's say I want to make sure that the unit tests are covering. So, in that case, I run

296
00:44:59.740 --> 00:45:05.100
Gabor Szabo: A test coverage report on the test of the subsystem, or whatever.

297
00:45:05.320 --> 00:45:08.210
Gabor Szabo: Crate, or… so part of the code.

298
00:45:09.470 --> 00:45:20.860
Gabor Szabo: So I wanted to do this, I think, run anti-gravity, which is Gemini, and I have a subscription, so hopefully it will, do some…

299
00:45:23.100 --> 00:45:30.640
Gabor Szabo: okay, whatever, it's warning about me, good, whatever, good. So, I would like to,

300
00:45:31.240 --> 00:45:34.929
Gabor Szabo: I don't know exactly the…

301
00:45:36.120 --> 00:45:48.090
Gabor Szabo: we can go back to the report, okay? So, we can go back to this report, and find…

302
00:45:48.200 --> 00:45:49.790
Gabor Szabo: something here.

303
00:45:53.400 --> 00:45:56.120
Gabor Szabo: Here, there is a function here, indent.

304
00:45:56.270 --> 00:45:57.310
Gabor Szabo: Okay?

305
00:45:57.550 --> 00:46:03.450
Gabor Szabo: So, right… one test, Death.

306
00:46:03.580 --> 00:46:07.180
Gabor Szabo: Real… Cool…

307
00:46:13.970 --> 00:46:16.420
Gabor Szabo: It's called SRC Displays.

308
00:46:16.540 --> 00:46:17.430
Gabor Szabo: Right?

309
00:46:18.380 --> 00:46:21.420
Gabor Szabo: The… indent.

310
00:46:23.730 --> 00:46:32.990
Gabor Szabo: Hmmelt… the… what is it? Config indent? Let's say this one, config indent.

311
00:46:34.100 --> 00:46:42.250
Gabor Szabo: function in the SRC… display… File.

312
00:46:43.340 --> 00:46:44.340
Gabor Szabo: Okay?

313
00:46:44.480 --> 00:46:45.870
Gabor Szabo: I, I…

314
00:46:46.110 --> 00:47:00.439
Gabor Szabo: just trying to, minimize it, okay? So I told it what to do. Let me see. We are seeing the test works fine with cargo test, but with cargo turbo, it takes forever. Well,

315
00:47:00.730 --> 00:47:06.890
Gabor Szabo: So, the forever is way too long, obviously, but, it's,

316
00:47:07.120 --> 00:47:19.629
Gabor Szabo: It's definitely a lot slower, because now, while the test is running, it's also recording things, so it added various things to the code base, to the compiled code.

317
00:47:20.440 --> 00:47:25.289
Gabor Szabo: So you have a lot more code, inside your… your…

318
00:47:26.080 --> 00:47:44.390
Gabor Szabo: code, right? Your application, and it's writing somewhere, so it will definitely take longer. Now, forever, it's a bit too much, okay? So, but I did see, like, a 10 times increase in time, and that seems like,

319
00:47:44.750 --> 00:47:50.230
Gabor Szabo: I don't know, normal, shall I call it, but existing, at least. So…

320
00:47:54.010 --> 00:48:08.839
Gabor Szabo: Yeah. Now, on the other hand, yeah, you can… you could take a look, I don't know if what operating system you're running. It still hasn't finished, so I feel it's stopped. Yeah, maybe it stopped. Yeah, that can also happen. But you could also take a look at the…

321
00:48:09.370 --> 00:48:22.489
Gabor Szabo: So probably you can also set some logging on it, so you will see if it is progressing, and… and you could probably take a look at… because you're using Zoom, it might also take up some of your,

322
00:48:24.520 --> 00:48:32.550
Gabor Szabo: CPU power, I had no idea what… you can take a look at that one, depending on your operating system, and the… how…

323
00:48:32.680 --> 00:48:37.510
Gabor Szabo: bigger computer it is. So here it is, okay?

324
00:48:37.950 --> 00:48:44.139
Gabor Szabo: So, there is a… Test for it.

325
00:48:44.660 --> 00:48:53.600
Gabor Szabo: Okay? And now we can go to the… To the outside, git status.

326
00:48:55.060 --> 00:49:07.530
Gabor Szabo: And this is what changed. So these are… this is the tarpaulin report, okay? Probably I should set a gitignore, so it won't be added by meme.

327
00:49:07.530 --> 00:49:16.869
Gabor Szabo: by mistake, but it's not really my project, so… And so this is what the… the agent now changed, bit, diff, S-R-C,

328
00:49:17.080 --> 00:49:18.010
Gabor Szabo: Leap.

329
00:49:19.240 --> 00:49:24.439
Gabor Szabo: And, I don't know. Use create display config?

330
00:49:26.430 --> 00:49:27.490
Gabor Szabo: Okay.

331
00:49:28.380 --> 00:49:37.330
Gabor Szabo: document from, it takes the document, doc, okay? Config default pretty, indent, okay, this is what it does.

332
00:49:37.560 --> 00:49:43.630
Gabor Szabo: So it… it creates some configuration, and then it runs two string prettes config.

333
00:49:43.780 --> 00:49:47.220
Gabor Szabo: Okay, and now it needs to look at, like, this.

334
00:49:48.380 --> 00:49:55.789
Gabor Szabo: Okay, let's, hope that it works, okay? So…

335
00:49:55.970 --> 00:50:00.050
Gabor Szabo: I go back, and now probably,

336
00:50:03.260 --> 00:50:16.079
Gabor Szabo: probably I need to… I should have two Docker sessions open, or I can run here, with an exclamation mark, I think. I can run cargo…

337
00:50:16.550 --> 00:50:21.019
Gabor Szabo: cargo… How did I write a telephone pole in here?

338
00:50:25.970 --> 00:50:27.959
Gabor Szabo: Test coverage, this one.

339
00:50:28.750 --> 00:50:30.740
Gabor Szabo: Cargo tarpaulin?

340
00:50:34.620 --> 00:50:36.959
Gabor Szabo: And then I also need this one.

341
00:50:40.670 --> 00:50:54.260
Gabor Szabo: And before I do that, let me… move the tarpauline… A report, tarpaulin report… 1.

342
00:50:54.510 --> 00:50:57.850
Gabor Szabo: Okay? And now I run this code again.

343
00:50:58.350 --> 00:51:05.509
Gabor Szabo: And so I'm inside the agent, but I'm executing an external code, hopefully.

344
00:51:05.630 --> 00:51:12.659
Gabor Szabo: So it's not the agent doing anything, it's just letting the… I don't need to leave the…

345
00:51:12.940 --> 00:51:18.309
Gabor Szabo: the agent to run this, and I think it finished.

346
00:51:19.010 --> 00:51:25.429
Gabor Szabo: Let me see its status. Okay, so it created a new report.

347
00:51:25.620 --> 00:51:34.890
Gabor Szabo: And I can go back to this page, and now I'm going to reload it, and hopefully this part is going to be green.

348
00:51:36.110 --> 00:51:42.160
Gabor Szabo: this part, okay? Good. So I… now I have a… new test.

349
00:51:43.170 --> 00:51:49.549
Gabor Szabo: And I can get… check out, minus B.

350
00:51:50.410 --> 00:51:51.360
Gabor Szabo: Ed.

351
00:51:51.490 --> 00:51:52.430
Gabor Szabo: Dest?

352
00:51:52.880 --> 00:51:55.589
Gabor Szabo: Give that SRC.

353
00:51:55.930 --> 00:51:56.960
Gabor Szabo: bleep?

354
00:51:59.420 --> 00:52:05.480
Gabor Szabo: Git status, git… And… nest?

355
00:52:22.160 --> 00:52:23.030
Gabor Szabo: Beautiful.

356
00:52:23.360 --> 00:52:24.160
Gabor Szabo: Oof.

357
00:52:24.380 --> 00:52:27.510
Gabor Szabo: Minus… you…

358
00:52:33.840 --> 00:52:34.690
Gabor Szabo: 4.

359
00:52:39.070 --> 00:52:40.480
Gabor Szabo: It's this?

360
00:52:41.300 --> 00:52:52.549
Gabor Szabo: Okay? So this one, you remember Git Remote minus 3. I have, in this project, I have two remotes. I cloned it originally from

361
00:52:52.730 --> 00:53:02.119
Gabor Szabo: So I don't work the same way as many other people do. I don't… when I encounter an open source project, I don't fork and clone.

362
00:53:02.440 --> 00:53:07.549
Gabor Szabo: from my own fork. Instead, I clone from the original one.

363
00:53:08.590 --> 00:53:20.160
Gabor Szabo: that I'm going to contribute to, and then I play with it, and then if I really know how to contribute, then I create a fork, and I create a remote which is called fork.

364
00:53:20.170 --> 00:53:34.910
Gabor Szabo: This makes more sense to me, okay? Instead of the other way around that many people recommend. And so now, I created a branch, and pushed out that branch to my fork.

365
00:53:35.160 --> 00:53:43.289
Gabor Szabo: And I can go to… It's SubGov XML, okay?

366
00:53:43.590 --> 00:53:48.619
Gabor Szabo: GitHub subgroup XMLEM, okay?

367
00:53:48.790 --> 00:53:57.470
Gabor Szabo: So this is the fork, and it's here, and I think the actions are Running.

368
00:53:58.000 --> 00:54:04.310
Gabor Szabo: Yeah, and the test passed, okay, so this was one minute ago, so this is the GitHub action.

369
00:54:04.510 --> 00:54:07.059
Gabor Szabo: And I can go back to this one.

370
00:54:07.240 --> 00:54:11.919
Gabor Szabo: And, send a pull request.

371
00:54:12.630 --> 00:54:14.080
Gabor Szabo: Create a pull request.

372
00:54:18.000 --> 00:54:19.049
Gabor Szabo: And that's it!

373
00:54:20.320 --> 00:54:30.130
Gabor Szabo: Okay? Now, I'm going to… I think that's it. If you have any questions, any more comments, then…

374
00:54:30.260 --> 00:54:43.430
Gabor Szabo: do it, then ask it now. If not, I can stop the recording, so people who are watching the video can click on the like button and follow the channel, okay, remember that one.

375
00:54:44.200 --> 00:54:51.200
Gabor Szabo: And, and you can stay around and do a little bit more work, if you…

376
00:54:51.810 --> 00:54:56.019
Gabor Szabo: would like to, those people who are around, and I can still help you.

377
00:54:56.460 --> 00:55:04.400
Gabor Szabo: So, it seems… can you share the YouTube channel? Yes, yes, I don't… It's,

378
00:55:07.770 --> 00:55:10.220
Gabor Szabo: How do I share it?

379
00:55:10.360 --> 00:55:11.370
Gabor Szabo: Properly.

380
00:55:11.810 --> 00:55:18.310
Gabor Szabo: Find new window… Without sharing all the… everything.

381
00:55:18.560 --> 00:55:26.050
Gabor Szabo: YouTube at Google. Actually, I think if I just visit that URL, then it will only show those

382
00:55:26.290 --> 00:55:33.889
Gabor Szabo: Not all of my… This is the YouTube channel. Okay, I put it in the chat.

383
00:55:34.750 --> 00:55:35.860
Gabor Szabo: Boom.

384
00:55:37.690 --> 00:55:49.630
Gabor Szabo: Surprisingly, it's called Code Mailer. Anyway, so thank you for, for being here, and asking questions and discussing things.

385
00:55:49.630 --> 00:56:04.759
Gabor Szabo: And thank you for everyone who's watching the video, for watching it, and follow the channel. And below the video, you will find links to… I guess, to the video, and to the… to notes, and to all the things that I shared in the chat.

386
00:56:06.410 --> 00:56:19.070
Gabor Szabo: And, sign up for upcoming events, both Rust and other languages that I also mentioned. And that's it for now. Thank you very much. Bye-bye.


