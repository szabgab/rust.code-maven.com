---
title: Miscompilation in Rust 1.97
timestamp: 2026-11-18T12:30:01
author:
published: true
show_related: true
description:
tags:
---

## Other events

* [Code Maven Live](https://live.code-maven.com/)

## Description

I reported a miscompilation in Rust 1.97. In this presentation, I'll go through the juicy details of the bug, walk through the investigation as it [unfolded in the Zulip chat](https://rust-lang.zulipchat.com/#narrow/channel/131828-t-compiler/topic/Miscompilation.20in.201.2E97.2E0.20with.20new.20Option.3A.3ANone.20discrimi.2E.2E.2E/with/609853461) with the Rust team, and share my takeaways and conclusions.

The issue: [segfault on Rust 1.97.0](https://github.com/rust-lang/rust/issues/159035).

## Bio

[Yoni Peleg](https://www.linkedin.com/in/yoni-peleg/) Software engineer with 10 years of experience (6 in Rust).

## Length

45 minutes

## Slides

* [Miscompilation in Rust 1.97](https://docs.google.com/presentation/d/1yoLmFF30WaOKEsJ0brBK089Bj_FbXKsqA13tUw9CBhI/edit?usp=sharing)

## Notes

* [niche optimization](https://niche.rs/)
* [Waterbed theory](https://en.wikipedia.org/wiki/Waterbed_theory)
* The mismatch between Rust and LLVM seems fairly common. [Miscompilation: Equal pointers comparing as unequal](https://github.com/rust-lang/rust/issues/107975) recently.
* Videos of [Jon Gjengset](https://www.youtube.com/c/JonGjengset)
* [Datafusion comet](https://github.com/apache/datafusion-comet)


{% youtube id="-dNsUB7QRk4" file="2026-11-17-Miscompilation-in-Rust-1-97-with-Yoni-Peleg.mp4" %}

## Transcript

1
00:00:02.110 --> 00:00:18.480
Gabor Szabo: So, hello and welcome to this session of the quarterly event session, specifically the Rust channel. Thank you, Yoni, for, accepting the invitation and agreeing to give this presentation, and thank you everyone who…

2
00:00:18.480 --> 00:00:20.070
Yoni Peleg: Thank you for having me.

3
00:00:20.070 --> 00:00:26.689
Gabor Szabo: Yeah, definitely. I, you… So you're not… not the first time you're presenting, and I always loved your presentations.

4
00:00:29.040 --> 00:00:45.900
Gabor Szabo: And thank you, everyone, who joined us here, and you have the opportunity to ask questions live, and use the chat to do that. And then afterwards, you're going to have a free conversation after the video is recorded.

5
00:00:46.150 --> 00:00:50.010
Gabor Szabo: And the… if you're watching this in YouTube, then,

6
00:00:50.270 --> 00:01:02.360
Gabor Szabo: like the video and follow the channel, first of all, the standard thing that I have to say. Then also, below the video, you will find a link to the future events, so check them out and register to future events.

7
00:01:02.360 --> 00:01:12.249
Gabor Szabo: And you will find a link to notes about this presentation, maybe the slides, whatever, various links are going to be below the video.

8
00:01:12.250 --> 00:01:20.319
Gabor Szabo: So, do that as well, and hope to see you in one of our future, events.

9
00:01:20.860 --> 00:01:24.980
Gabor Szabo: That's it for now, I think. Yoni, the stage is yours.

10
00:01:25.450 --> 00:01:37.179
Yoni Peleg: Okay, so I'll share my screen, let's see… It's this one… Yep, this one.

11
00:01:39.060 --> 00:01:45.819
Yoni Peleg: Okay, so, hello everyone, thank you all for joining. Today we're going to talk about.

12
00:01:46.370 --> 00:01:50.619
Yoni Peleg: bug that I reported to, to the Rust team.

13
00:01:50.760 --> 00:02:04.179
Yoni Peleg: A couple of months back, and I discussed this bug with the Rust team, and I learned quite a few interesting stuff from that, and I'm gonna be sharing them with you today.

14
00:02:06.020 --> 00:02:18.800
Yoni Peleg: So, first a little bit about me. I'm Yoni Peleg, I'm married, plus one is a girl. I've been in the industry for about 10 years, 6 of them in Rust. I picked up Rust back in,

15
00:02:19.010 --> 00:02:29.299
Yoni Peleg: mid-2020, and I've worked mainly in small startups, in a variety of different market sections and products.

16
00:02:30.730 --> 00:02:36.180
Yoni Peleg: Yeah, from big data to, like, AI coding tools and whatnot.

17
00:02:36.310 --> 00:02:44.170
Yoni Peleg: My main focuses throughout the years were distributed systems, cross-source binaries, so, like, binaries that you distribute to

18
00:02:44.620 --> 00:02:56.069
Yoni Peleg: To millions of users on a monthly basis, on all kinds of different operating systems and architectures and whatnot, and also web backends, so all that fun stuff.

19
00:02:56.400 --> 00:03:03.239
Yoni Peleg: Yeah, so… About, so, our story began…

20
00:03:03.290 --> 00:03:18.840
Yoni Peleg: Let's say it like that. Back in July this year, I've been working on some cool ROS code, I was happy with my code, I ran git commit, git push, and then I get a segfault in CI. Now I'm sad, I'm no longer happy.

21
00:03:18.880 --> 00:03:24.450
Yoni Peleg: And I, of course, went to investigate why was that happening.

22
00:03:24.780 --> 00:03:36.449
Yoni Peleg: And pretty quickly, it turns out that it has nothing to do with my code or my changes, and it has more to do with, with Rust itself, so I filed an issue to the Rust team.

23
00:03:36.560 --> 00:03:48.650
Yoni Peleg: And, of course, it's been closed, since then, but, yeah, I've been discussing it with the Rust team on Zulip, which is their chat application, and…

24
00:03:49.040 --> 00:03:57.170
Yoni Peleg: And the discussion turned out to be quite fruitful, and I've surely learned quite a bunch, and also the bug itself is

25
00:03:57.360 --> 00:04:09.080
Yoni Peleg: very interesting, in my opinion. Just, like, a case study of how compilers work and what they do behind the scenes. So that's what I'm gonna share with you today.

26
00:04:09.240 --> 00:04:27.249
Yoni Peleg: So, on the agenda today, we have a technical walkthrough of, like, the bug itself, how it happens, all the way from the source code to the generated, assembly instructions, and what exactly went wrong there. Then we answer a couple of questions that I personally had

27
00:04:27.400 --> 00:04:39.959
Yoni Peleg: During this, this whole, like, event. Which was first, how did it reach stable? Like, it's pretty much uncommon that, the seg fault reaches, stable rust.

28
00:04:40.060 --> 00:04:52.719
Yoni Peleg: And why me? How it came about that I was the one out of all the Rust users out there in the world, which is, fortunately, quite a bunch by now.

29
00:04:52.760 --> 00:05:01.489
Yoni Peleg: So how it comes that it was me who caught the bug first and reported it, and finally we'll go over, some conclusions.

30
00:05:01.820 --> 00:05:03.340
Yoni Peleg: Yeah.

31
00:05:04.690 --> 00:05:08.950
Yoni Peleg: So, without further ado… Let's go through the technical walkthrough.

32
00:05:09.780 --> 00:05:23.429
Yoni Peleg: So, that's a minimal reproduction script, or, like, snippet. That's not the one I filed in the issue, that's an even more minimal snippet that someone on the Rust team

33
00:05:23.650 --> 00:05:27.439
Yoni Peleg: Was able to, to, to write.

34
00:05:27.830 --> 00:05:28.710
Yoni Peleg: And…

35
00:05:29.360 --> 00:05:39.550
Yoni Peleg: As we can see, there's an enum here called Repro. It has two variants, X and Y, and there's a run function, which accepts option of repro.

36
00:05:40.030 --> 00:05:44.399
Yoni Peleg: And that's actually an important bit. We'll take a look at that.

37
00:05:44.770 --> 00:05:53.859
Yoni Peleg: And it runs a simple match, and this match basically extracts the second element from the tuple

38
00:05:54.040 --> 00:06:12.940
Yoni Peleg: whatever variant this is. So, like, if it's X, whether it's true or false, it extracts the second element, the U64, that we have in the X variant. If it's Y, we also… we discard the first value, and we extract the second one.

39
00:06:13.800 --> 00:06:25.199
Yoni Peleg: And if it's none, then we return none, okay? So this match, what it's doing is always trying to extract the second U64 element from this enum, whatever variant it is.

40
00:06:25.270 --> 00:06:37.069
Yoni Peleg: Then, we call it with none in the main function, and if you run this exact code as is on Rust 1.97, you actually get a segfault, and it's quite…

41
00:06:37.310 --> 00:06:44.360
Yoni Peleg: a trivial piece of code, like, if you know Rust even a little bit, nothing here, looks weird or funny.

42
00:06:44.440 --> 00:06:55.240
Yoni Peleg: And yet, it does crush your process. So, to understand how this works and exactly where the issue is, we first need to take a look at

43
00:06:55.290 --> 00:07:06.139
Yoni Peleg: how option of repro is represented. And to do that, we need to talk about niche optimization, or niche. Some say niche, I tend to say niche, so,

44
00:07:06.320 --> 00:07:09.569
Yoni Peleg: So yeah, so that's niche, as far as I'm concerned.

45
00:07:09.930 --> 00:07:23.950
Yoni Peleg: Anyway, niche optimization is, basically asking the question, what is the most compact way to represent the option of repro? Okay, we have an option, which is an enum, and we have a repro, which is also an enum.

46
00:07:24.630 --> 00:07:27.880
Yoni Peleg: And both enums have tags, enums in Rust.

47
00:07:28.050 --> 00:07:34.179
Yoni Peleg: They are, like, union types, and they have tags, and the tag is, like…

48
00:07:34.520 --> 00:07:50.690
Yoni Peleg: is what telling the runtime which variant is currently being used? So, 0 will say, we're talking about Wipro. Like, colon, colon X. 1 means we're using the Y variant.

49
00:07:50.880 --> 00:07:54.919
Yoni Peleg: And same goes for option. Option has a sum.

50
00:07:55.100 --> 00:08:11.509
Yoni Peleg: of whatever value it is, variant and the non-variant, so the option tag will have 0 and 1, representing sum and none, respectively. But Rust says, wait a minute, we have an option of repro.

51
00:08:11.540 --> 00:08:21.019
Yoni Peleg: So it means we're supposed to have 2 tags, but actually, the repro tag is a U32, so, like, 4 bytes, and we only use

52
00:08:21.120 --> 00:08:30.540
Yoni Peleg: two, two values from this entire, value space. We are only using 0 and 1, right? So we could just…

53
00:08:31.040 --> 00:08:35.270
Yoni Peleg: Collapse, like, sort of collapsed or smooshed together the

54
00:08:35.270 --> 00:08:51.690
Yoni Peleg: the tag of the option, like the wrapper option enum, into the inner tag, and represent it as another value of the repro tag, right? We can represent none as any other value that's not either 0 or 1. We can use

55
00:08:51.690 --> 00:08:52.510
Yoni Peleg: I don't know…

56
00:08:52.760 --> 00:09:00.060
Yoni Peleg: 4, 2000, whatever it is. As long as it's not 0, 1, we're good to go, and now we've saved…

57
00:09:00.630 --> 00:09:03.260
Yoni Peleg: whole U32 tag, right?

58
00:09:03.450 --> 00:09:09.789
Yoni Peleg: So, this non-variant gets either the value of 2 or negative 1, and we see later

59
00:09:09.900 --> 00:09:18.699
Yoni Peleg: How that matters, and a little bit of a spoiler that really does matter, and specifically the negative 1 value is the one that matters.

60
00:09:19.900 --> 00:09:31.650
Yoni Peleg: But, hold your hoses, we'll get there. Anyway, so let's look at the, like, the memory layout of this option of repo, structure.

61
00:09:31.700 --> 00:09:44.939
Yoni Peleg: And just as a reminder, I pasted here the Repro enum. We have an X variant with a bool and a U64, and Y variant with two U64s, and this entire option of repro

62
00:09:45.170 --> 00:10:01.789
Yoni Peleg: structs will be, or enum, rather, will be represented as a total sum of 24 bytes. Bytes 0 to 4 will contain the tag, which is either 0 for X, 1 for Y, or whatever other value it is for the non-variant of the option.

63
00:10:01.870 --> 00:10:07.349
Yoni Peleg: Bytes 4 to 8 will contain X's bool, which is the first element of X.

64
00:10:07.470 --> 00:10:19.110
Yoni Peleg: Bytes 8 to 16 will represent either X's second U64 or Y's first U64, depending on the tag.

65
00:10:19.450 --> 00:10:33.699
Yoni Peleg: If the tag is 0, these bytes will contain X's second, element, and if the tag is 1, it will contain Y's first element. And bytes 16 to 24 will contain only Y's second, U64.

66
00:10:34.670 --> 00:10:35.750
Yoni Peleg: Okay.

67
00:10:36.720 --> 00:10:45.259
Yoni Peleg: So, a bit of a trigger warning. Before we continue, we're gonna look at some other VMAR and assembly code, and…

68
00:10:45.530 --> 00:11:04.519
Yoni Peleg: A little bit about LLVM IR. Ir stands for Intermediate Representation, and LLVM is, like, in a simple sentence, is like the compiler backend from any, from any natively compiled languages. Rust is, of course, one of them.

69
00:11:05.310 --> 00:11:16.609
Yoni Peleg: Rust compiles not only with LLVM, but LLVM is, like, the most typical backend compiler for Rust. So how this goes is RustC, the compiler.

70
00:11:16.610 --> 00:11:29.960
Yoni Peleg: takes in, text, like .rs files that you write in your whatever, and it outputs LLVMIR, which is the input into LLVM. LLVM takes that

71
00:11:29.960 --> 00:11:35.850
Yoni Peleg: language, and spits out, binary, like, assembly.

72
00:11:36.230 --> 00:11:43.490
Yoni Peleg: So yeah, so we're going to take a look both at the RVMAR and assembly snippets that are relevant to this issue.

73
00:11:43.650 --> 00:11:50.920
Yoni Peleg: So brace yourselves, take a deep breath, and we'll power through it. Maybe before I do that, let's take a look.

74
00:11:51.020 --> 00:11:54.570
Yoni Peleg: The chat… the chat is empty, so we can continue.

75
00:11:55.080 --> 00:12:02.010
Yoni Peleg: Okay, so let's start with the Arab MIR. It looks kind of like assembly, and we don't really need to understand

76
00:12:02.060 --> 00:12:15.869
Yoni Peleg: everything that's going on here. What is important to note is that we load both offsets 8 and 16 unconditionally before we even look at the tag, okay? Now, if we get back here.

77
00:12:15.870 --> 00:12:28.109
Yoni Peleg: offset 8 contains Y's, X's second variant, and offset 16 contains Y's second variant, and that's the values that we're trying to extract.

78
00:12:28.410 --> 00:12:47.540
Yoni Peleg: So we load both of them beforehand, then we run a little select that looks, basically, at the tag's most lower bit, and if it's 0, it means that the entire tag is 0, and we take V1, which contains the value at offset 8,

79
00:12:47.640 --> 00:12:54.120
Yoni Peleg: And if it's 1, we take the value at offset 16, and then we run an outer select.

80
00:12:54.220 --> 00:13:04.350
Yoni Peleg: And we look at whether or not the tag is negative 1, not just the lower bit. Rather, now we look at the entire tag and see what's the value.

81
00:13:04.430 --> 00:13:15.129
Yoni Peleg: And if the tag is negative 1, we return undefined, and so we discard the inner select that we ran just one light beforehand.

82
00:13:15.130 --> 00:13:26.100
Yoni Peleg: And our… and if not, we return the inner, the inner value, whatever it is, according to the unconditional loads that we ran just before.

83
00:13:27.610 --> 00:13:31.509
Yoni Peleg: Now, so far so good, nothing's, problematic here.

84
00:13:33.290 --> 00:13:44.210
Yoni Peleg: what's the problem is in the assembly instructions that are generated. And here as well, we don't have to understand every single instruction here.

85
00:13:44.360 --> 00:13:58.849
Yoni Peleg: What is important to note is that we load the tag from… okay, RDI is the pointer to the beginning of the enum, to the… to those 24 bytes that we looked at, and…

86
00:13:58.980 --> 00:14:05.469
Yoni Peleg: We load the, we load from the pointer into ECX, meaning that we load the tag.

87
00:14:06.240 --> 00:14:09.710
Yoni Peleg: into ECX. Now, ECX is…

88
00:14:09.920 --> 00:14:16.649
Yoni Peleg: half of a register, and most precisely is half of the RCX register.

89
00:14:16.660 --> 00:14:33.150
Yoni Peleg: It's the lower 32-bit of this register. Okay, so we load a value into half of a register. Now, when you do that, and we'll take a look at that, like, in the next slide, but when you do that, it's important to note.

90
00:14:33.150 --> 00:14:46.559
Yoni Peleg: that the move instruction, move long in this case, it zero extends into the entire 64 bit, okay? So if you load some value into the lower half, the lower 32 bit,

91
00:14:46.960 --> 00:14:53.489
Yoni Peleg: the higher 32-bit will always be 0. That's what this instruction is doing. And…

92
00:14:53.740 --> 00:14:56.639
Yoni Peleg: What's also important to note is the,

93
00:14:56.760 --> 00:15:01.169
Yoni Peleg: the other move, instruction here, move Q,

94
00:15:01.340 --> 00:15:10.509
Yoni Peleg: Which uses RCX, which is the entire registers that contains ECX, as an index.

95
00:15:10.760 --> 00:15:22.440
Yoni Peleg: into the enum, meaning that it uses the tag to index into the enum. Okay, that's a clever little trick. What we do is we take the base pointer address, which is

96
00:15:22.830 --> 00:15:32.219
Yoni Peleg: sorry, which is RDI, we add RCX, which is the tag, we multiply it by 8, we add 8, and then we get our value. Now.

97
00:15:32.470 --> 00:15:49.689
Yoni Peleg: This looks kind of obscure, and maybe not really clear. Certainly, it wasn't clear to me, in the first glance how this works, so let's break it down a little bit. Let's move… let's look at this instruction, that indexes into the enum based on the tag.

98
00:15:50.300 --> 00:15:51.740
Yoni Peleg: Now,

99
00:15:52.020 --> 00:16:07.579
Yoni Peleg: When the tag is 0, RCX times 8 gives us 0, then the entire address you load is the base pointer plus 8, which indexes into X's second field, which is what we want. So far, so good.

100
00:16:07.670 --> 00:16:22.650
Yoni Peleg: When the tag is 1, we multiply it by 8 to get 8, we add another 8 to get 16, and we index into a Y's second field. That's also great. But let's look at what happens when the tag is,

101
00:16:22.790 --> 00:16:28.249
Yoni Peleg: in the niche values, like either 2 or negative 1. So when the tag is 2,

102
00:16:28.380 --> 00:16:46.929
Yoni Peleg: we multiply by 8, and we get 16, then we add another 8 to get 24, and we start reading 8 bytes past the 24 bytes we have for the enum. This entire enum is 24 bytes long, and now we read right past this enum. And, most importantly.

103
00:16:47.390 --> 00:16:49.779
Yoni Peleg: When the tag is negative 1,

104
00:16:50.230 --> 00:16:57.549
Yoni Peleg: Now we look at the entire RCX, which is the entire 64-bit, and these 64-bit are now

105
00:16:57.790 --> 00:17:02.540
Yoni Peleg: Representing U32 Max, which is, 4GB.

106
00:17:02.610 --> 00:17:22.140
Yoni Peleg: Okay? And that's the issue, in fact. And so when it's 4GB, you multiply it by 8, and you add 8, you get precisely 32GB, and now you read 32GB ahead of the enum, which is, of course, out of bounds from the memory that's mapped for the process in the most cases.

107
00:17:23.740 --> 00:17:37.260
Yoni Peleg: And that's, in fact, the issue. Rust 1.97 did not change any of the assembly instructions generated. All it changed is the niche tag from 2 to negative 1, and that's what caused the process to read…

108
00:17:37.390 --> 00:17:45.680
Yoni Peleg: Instead of reading, like, 8 bytes right past the enum, which is, of course, not a valid read, like.

109
00:17:45.790 --> 00:17:59.690
Yoni Peleg: like, in terms of correctness of the program, but the kernel isn't going to do anything about it, because usually this memory is mapped to the process, and it's a valid RAID as far as the kernel is aware.

110
00:18:00.070 --> 00:18:17.350
Yoni Peleg: And, as we saw earlier, when the tag is negative 1, we discard the entire load and we return undefined back in the LVMIR snippet, so no one is reading this invalid, invalid value that we just loaded, so this bug was dormant.

111
00:18:17.350 --> 00:18:20.199
Yoni Peleg: For, like, 10 versions for Rust.

112
00:18:20.200 --> 00:18:29.510
Yoni Peleg: Until they changed it to negative 1, and now the same instruction reads 32GB past the enum, which is usually

113
00:18:29.510 --> 00:18:40.820
Yoni Peleg: Reading into, an address that is outside the address space of the process, and that's where the kernel comes in and says no, and you get a segfault.

114
00:18:41.010 --> 00:18:48.140
Yoni Peleg: Yeah, okay, so far so good. That was the end of the technical walkthrough.

115
00:18:48.500 --> 00:18:51.190
Yoni Peleg: Let's see…

116
00:18:52.090 --> 00:18:59.440
Yoni Peleg: So, it was not a bug in Rust, and actually a bug in NLVM assembly process. Yes, actually…

117
00:19:00.260 --> 00:19:04.339
Yoni Peleg: You could say that, that it's a bug in LVM, but…

118
00:19:04.660 --> 00:19:07.880
Yoni Peleg: I think that's not precisely the case.

119
00:19:08.410 --> 00:19:22.619
Yoni Peleg: this is a case of, like, two unrelated pieces of code, not agreeing precisely enough on the contract between them. So, like, the…

120
00:19:23.250 --> 00:19:31.599
Yoni Peleg: The assembly instruction that we saw here is totally valid if the index is not

121
00:19:31.720 --> 00:19:37.949
Yoni Peleg: Is not in the… like, if the tag is not in the niche values, like, this snippet is not aware

122
00:19:37.950 --> 00:19:55.000
Yoni Peleg: of the niche optimization done by the Rust C, and Rust C, like the generated LLVMIR, was not aware that LLVM would do this little trick of using the tag as an index. So, it's like two unrelated pieces of code.

123
00:19:55.000 --> 00:20:04.640
Yoni Peleg: not agreeing precisely on the, on, like, the interface between them, and that was… caused the, the explosion. So, like.

124
00:20:04.670 --> 00:20:07.359
Yoni Peleg: It's a bug in neither side.

125
00:20:07.450 --> 00:20:08.800
Yoni Peleg: like…

126
00:20:08.910 --> 00:20:17.760
Yoni Peleg: It's not completely in either side of the, like, the front end and back end of the compiler, but rather, like, how they talk together.

127
00:20:18.530 --> 00:20:24.580
Yoni Peleg: Yeah, hope this… Clears… Things?

128
00:20:25.210 --> 00:20:35.450
Yoni Peleg: Let's see another question. Why does the assembly need to actually read the memory at all if it's for the non-variant anyway? So, that's exactly,

129
00:20:36.760 --> 00:20:46.419
Yoni Peleg: That's exactly why, why we show the RVMIR snippet, because we unconditionally load the variant.

130
00:20:46.630 --> 00:20:50.269
Yoni Peleg: Before we even look at the tag in this specific case.

131
00:20:50.400 --> 00:21:03.320
Yoni Peleg: Because RustC decides that when it sees a code in this shape and form, it's better to first load for memory, and then decide whether you want to actually use it or not.

132
00:21:03.400 --> 00:21:14.059
Yoni Peleg: It's better and more optimized than first looking at the tag, and only if it's not negative 1, only then I will go and actually load the memory.

133
00:21:14.260 --> 00:21:24.639
Yoni Peleg: So it's like, it's a kind of optimization that's been known for a while, it's… I think it's for decades at this point, people are doing…

134
00:21:24.710 --> 00:21:42.019
Yoni Peleg: this kind of optimizations, like pre-loading things into memory, even if they don't need them, because it's faster in certain scenarios, and here, the Rusty compiler decided that it will indeed be fast, and as a result, the LVM snippet

135
00:21:42.970 --> 00:21:45.559
Yoni Peleg: Sorry, the assembly snippet we saw.

136
00:21:45.750 --> 00:21:57.730
Yoni Peleg: is always loading the… the variants, whatever… no matter what the tag is, because that's what the LVMIR told it to do.

137
00:21:58.240 --> 00:21:59.110
Yoni Peleg: Okay?

138
00:22:02.840 --> 00:22:03.710
Yoni Peleg: seat?

139
00:22:05.060 --> 00:22:07.999
Yoni Peleg: I think I'll take one last question before I move on.

140
00:22:08.800 --> 00:22:12.669
Yoni Peleg: Why do you need to use the match in the first place?

141
00:22:13.240 --> 00:22:16.899
Yoni Peleg: Could your code not simply take the second element directly?

142
00:22:18.850 --> 00:22:30.070
Yoni Peleg: Okay, so this question is about the, the reproduce snippet as it stands, and it's important to notice that it's only reproduce.

143
00:22:30.260 --> 00:22:46.570
Yoni Peleg: snippet, like, it's not… it's been synthesized precisely to hit this… this pattern and produce the bug. It's not a code that's taken directly from, like, from an actual codebase.

144
00:22:47.320 --> 00:22:50.670
Yoni Peleg: And we'll… we'll also say about it.

145
00:22:50.830 --> 00:22:54.200
Yoni Peleg: In the next slides…

146
00:22:54.520 --> 00:23:04.699
Yoni Peleg: I have something to say about it in the couple of next slides, so please hold your thought, until then.

147
00:23:05.330 --> 00:23:23.790
Yoni Peleg: And one last thing I want to say is that my original snippet was making more sense, and it did reproduce the bug, so it just comes to show that it's not the only way to hit the bug, it's just a way to hit the bug that isn't necessarily making sense in a real-world project.

148
00:23:24.220 --> 00:23:30.100
Yoni Peleg: Or code, but the main purpose is just to reproduce the bug.

149
00:23:30.560 --> 00:23:31.410
Yoni Peleg: Okay?

150
00:23:35.890 --> 00:23:54.029
Yoni Peleg: Okay, if there are any more questions about that, I think I'll take them at the end, and we'll move on to the next section, which is, how did we reach stable? That's the first question I had when I discussed it with the Rust team, and it's certainly a question that's worth answering, in my opinion.

151
00:23:54.230 --> 00:24:11.789
Yoni Peleg: Because, as we saw, the reproduce snippet is, like, very trivial, no unsafe stuff, nothing funny going on there, and yet it did somehow reproduce a segfold in production, and on stable rust.

152
00:24:12.120 --> 00:24:19.059
Yoni Peleg: So, there's a couple reasons. First, it's for, it's reproduced in optimized builds only.

153
00:24:19.260 --> 00:24:34.789
Yoni Peleg: And also x86 only, so if you're on Mac, you'll not hit that no matter what. But, anyway, I think the most important part is that it's only for release builds. Debug builds won't hit it. And that…

154
00:24:34.890 --> 00:24:47.919
Yoni Peleg: answers a big part of it, at least in my head, because beta nightly users usually don't build for release. Like, if you build for release, you usually pick the stable channel,

155
00:24:48.470 --> 00:24:51.839
Yoni Peleg: So, this combination of compiling code with both

156
00:24:52.020 --> 00:25:09.100
Yoni Peleg: beta overnightly and release build should be quite rare, and that explains how the Rust 1.97 indeed reached stable, even though it was, like, the same change was introduced

157
00:25:09.320 --> 00:25:16.630
Yoni Peleg: a couple weeks before that in beta and in nightly, because it only reproduces in release builds.

158
00:25:17.940 --> 00:25:30.389
Yoni Peleg: Reason number two, which I also discussed with the team, is Creator that doesn't run in release mode. So Creator is a little tool that the Rust team built, and it's open source, you can look it up.

159
00:25:30.800 --> 00:25:42.710
Yoni Peleg: All it does is sample… it samples some crates from crates.io, and it runs cargo build and cargo test on them, on the provided Rust version that they want to check, so this is how they, like.

160
00:25:42.810 --> 00:25:57.880
Yoni Peleg: They can check whether they're introducing a breaking change, or what have you. It's a tool they're using quite a lot, from what I could tell. And Crater, when it runs cargo build and test, it runs it in debug mode, it doesn't run it in release mode.

161
00:25:57.960 --> 00:26:14.610
Yoni Peleg: So that actually was a suggestion I made to the team, to try and run Creator in release mode and see whether it would have caught the bug. I actually didn't follow up with them since then, so I don't know where it stands.

162
00:26:15.520 --> 00:26:24.969
Yoni Peleg: But they seem kind of inclined to give it a try, at least, so that's another thing that maybe could have caught the bug earlier.

163
00:26:25.080 --> 00:26:37.400
Yoni Peleg: And reason number 3, which I think is the most important, and it answers your question, I can't remember the name, about the, why the snippet looks like it, whatever it looks like.

164
00:26:37.460 --> 00:26:52.420
Yoni Peleg: Simple is not likely. Even though the reproduced snippet we saw is quite simple and trivial, it does not mean that it's expected that this pattern is, like, common out in the wild.

165
00:26:52.700 --> 00:27:05.770
Yoni Peleg: And as we'll see in the next few slides, any weak and small change that you make to the… that you make to the… to the snippet stops the bug from reproducing.

166
00:27:06.120 --> 00:27:12.990
Yoni Peleg: Because it's so fragile, and it's so dependent on the exact shape and form the code takes.

167
00:27:13.130 --> 00:27:29.550
Yoni Peleg: So, for example, the first thing that stops the reproduce is simply switching from two match arms, one for true and one for false. You switch to a wildcard, and now the bug does not reproduce.

168
00:27:29.840 --> 00:27:31.779
Yoni Peleg: And the reason is simply…

169
00:27:32.270 --> 00:27:41.009
Yoni Peleg: the Rust C, the Rust compiler, when it sees this… this other shape, for some reason, I re… I mean…

170
00:27:41.160 --> 00:27:55.349
Yoni Peleg: I'm not that deep into the Rust compiler, so I can't tell you exactly why, but as it turns out, the Rust compiler decides to avoid these two unconditional loads before we look at the tag, and instead it does insert

171
00:27:55.980 --> 00:28:04.109
Yoni Peleg: Like, a check on the tag before it loads the values, so it stops the entire thing from reproducing.

172
00:28:05.350 --> 00:28:20.800
Yoni Peleg: Second example is if you just widen the type a little bit, and you switch the U64s to U128s, and that simply changes the ABI, the application binary interface, and

173
00:28:20.900 --> 00:28:38.439
Yoni Peleg: And now the values no longer fit neatly into registers, because the registers are 64-bit, and when you have larger values, the entire generated assembly is working with indirectional, I mean, with pointers.

174
00:28:38.590 --> 00:28:52.670
Yoni Peleg: Instead of loading directly into the registers, and it can no longer, do the nice little trick, which is using the tag as an index into the address to load it.

175
00:28:52.800 --> 00:28:55.840
Yoni Peleg: So it also stops the reproduce.

176
00:28:56.810 --> 00:29:03.150
Yoni Peleg: Also, if you add another variant, no matter what it is, it also stops the reproduce, because

177
00:29:03.650 --> 00:29:15.810
Yoni Peleg: as we saw in the LVMIR snippet, we used two SELECT instructions, and SELECT is like if-else in a single instruction, sort of like a ternary operation.

178
00:29:15.970 --> 00:29:22.109
Yoni Peleg: So, you can only use the… this binary selection if you have

179
00:29:22.110 --> 00:29:42.239
Yoni Peleg: exactly two variants. Once you have more than two variants, you can't do this little trick with the inner and outer selects, and it also generates a different assembly code down the line that avoids this problematic load based on the tag, and it also stops the reproduce.

180
00:29:44.170 --> 00:29:57.829
Yoni Peleg: And lastly, if you change the tag, the, if you're not very familiar with Rust, this wrapper annotation is what is, the Rust compiler. What, like, what

181
00:29:58.190 --> 00:30:01.770
Yoni Peleg: How wide is the tag of the enum?

182
00:30:01.780 --> 00:30:18.660
Yoni Peleg: So when it's… when it says wrapper U32, it means that the tag value will be 4 bytes, and when it says wrapper U8, it means that the tag value is a single byte, and the Rust compiler specifically blocks the niche optimization,

183
00:30:18.740 --> 00:30:29.279
Yoni Peleg: when the tag is a single byte, because it caused them some issues, that's at least what the maintainers told me on the Zulik chat.

184
00:30:29.280 --> 00:30:46.279
Yoni Peleg: I haven't dug really deep into the details of how exactly it causes issues, but yeah, the fact of the matter is that it does cause them issues, and they stop this niche optimization from taking place, so it also stops the reproduce.

185
00:30:46.700 --> 00:30:59.900
Yoni Peleg: All of this just comes to show that, even though the reproduced snippet is very simple, it doesn't mean that it's very much expected to see it out in the wild, and that's

186
00:30:59.910 --> 00:31:08.379
Yoni Peleg: That, that also explains how comes, this miscompilation reached, reached, stable.

187
00:31:08.790 --> 00:31:14.610
Yoni Peleg: I think that's… Also, okay, the chat is clear so far.

188
00:31:14.910 --> 00:31:17.899
Yoni Peleg: So we can just move on to the next section.

189
00:31:20.060 --> 00:31:28.920
Yoni Peleg: Which is why me? How comes it was me who caught the bug, and not anyone else? So…

190
00:31:29.210 --> 00:31:33.529
Yoni Peleg: It also have… it also has a couple of reasons.

191
00:31:33.790 --> 00:31:44.670
Yoni Peleg: the company I used to work for back in July, so their CI runs in release mode, like, it compiles the Rust part of the product.

192
00:31:44.670 --> 00:31:55.640
Yoni Peleg: in release mode in CI, and that's a conscious decision, simply because the binary in debug mode was too slow for the CI. So, yeah, so they ran it in release mode.

193
00:31:55.670 --> 00:31:57.830
Yoni Peleg: And that's, like.

194
00:31:58.840 --> 00:32:05.849
Yoni Peleg: As we saw, we have to compile in release mode in order to even encounter the bug, because it doesn't reproduce in the bug.

195
00:32:07.310 --> 00:32:18.360
Yoni Peleg: Second, the CI was using the latest table. They did not pin their Rust version for some reason. I honestly can't tell you why, but that was the case.

196
00:32:18.600 --> 00:32:33.450
Yoni Peleg: So every machine that spawn up and took a job, just looked for whatever's the latest thrust, version, like the stable channel, and used it.

197
00:32:34.420 --> 00:32:35.840
Yoni Peleg: And lastly.

198
00:32:36.990 --> 00:32:52.309
Yoni Peleg: it just so happens to be that this unlikely pattern existed in their codebase. So yeah, so I was just the first one to push a commit after Rust 1.97 was released.

199
00:32:52.420 --> 00:32:55.729
Yoni Peleg: And that's why I hate the bug in,

200
00:32:55.850 --> 00:33:00.399
Yoni Peleg: Yeah, in my CI, and hence the meme for the… from the beginning.

201
00:33:00.720 --> 00:33:08.890
Yoni Peleg: So, just, like, a combination of unfortunate events, but yeah, but at least, that explains

202
00:33:09.360 --> 00:33:12.089
Yoni Peleg: how I was the first one to eat it.

203
00:33:13.370 --> 00:33:25.119
Yoni Peleg: Okay, the last section is conclusions. So, I think I'll get done with this section, and then I'll leave some time for questions.

204
00:33:26.130 --> 00:33:29.280
Yoni Peleg: So, the first conclusion is the compilers

205
00:33:29.410 --> 00:33:37.899
Yoni Peleg: are… are wild, honestly, and especially the Rust compiler, with all the additional work he does on the borrow checker,

206
00:33:38.090 --> 00:33:38.850
Yoni Peleg: But…

207
00:33:39.050 --> 00:33:54.430
Yoni Peleg: Even without that, compilers in general are very, very intricate, and so much work is going into every single line of code that you write, and as we saw throughout the presentation, every single

208
00:33:54.900 --> 00:34:03.010
Yoni Peleg: tweak or change that you make to your code produces widely different results.

209
00:34:03.490 --> 00:34:11.570
Yoni Peleg: Because the compilers are built on so many, like, micro decisions they take in order to…

210
00:34:11.989 --> 00:34:26.610
Yoni Peleg: To make your code not only, correct, but also as fast as possible. So yeah, so just, like, that bullet is just an appreciation bullet, just, like, to take a step back and think, I mean.

211
00:34:26.760 --> 00:34:43.919
Yoni Peleg: wow, so much work is going into my code being so fast when I simply write my code and run it, and so much work is being done behind the scenes to make it happen.

212
00:34:44.159 --> 00:34:45.429
Yoni Peleg: So that's one.

213
00:34:45.710 --> 00:35:03.630
Yoni Peleg: Second, I don't think I need to tell this to anyone, but if you're not doing this, then go do it, like, yesterday. Pin your Rust version, and while you're at it, pin all of your external dependencies, and also, while you're at it.

214
00:35:03.830 --> 00:35:13.149
Yoni Peleg: wait for a little bit with every version that you bump, like, don't pull the next version as soon as it comes out,

215
00:35:13.850 --> 00:35:23.020
Yoni Peleg: Because, as we can see, like, I mean, in general, but especially in this day and age, in 2026,

216
00:35:23.460 --> 00:35:30.669
Yoni Peleg: there are many CVs being discovered, and many bugs being discovered, and usually the vast majority of them

217
00:35:30.710 --> 00:35:45.679
Yoni Peleg: is being patched, like, in the next few days after a version is released. So, if you're maintaining, like, production, or sensitive, or any serious kind of product that's written in Rust, but

218
00:35:46.050 --> 00:35:52.419
Yoni Peleg: I think this point is true for any other language, so… In general.

219
00:35:52.750 --> 00:36:04.310
Yoni Peleg: Just wait a couple of days before your bump your versions, to avoid this kind of segfault, reaching your CI, or even worse, your production, and…

220
00:36:05.000 --> 00:36:16.200
Yoni Peleg: By the way, the latest ROS version, 1.100, shipped new feature in Cargo, which lets you set the minimum age

221
00:36:16.600 --> 00:36:19.110
Yoni Peleg: New dependencies before it pulls them.

222
00:36:19.480 --> 00:36:27.360
Yoni Peleg: So that's, like, a really nice feature. You can, in one line, you can take care of all of that in your cargo terminal.

223
00:36:28.780 --> 00:36:37.300
Yoni Peleg: And… I think… this point, run your CI on your release artifacts, it's…

224
00:36:37.860 --> 00:36:51.840
Yoni Peleg: Mostly important if you distribute your binary directly to your customers, like, if you're building CLIs or agents, like, not AI agents necessarily, but agents in, like, the older fashion agents.

225
00:36:51.840 --> 00:37:06.040
Yoni Peleg: like, if you distribute your binary directly to your users, that's more important, this bullet, but, I mean, as opposed to if you run your binary on some sort of cloud or server or whatever it is.

226
00:37:06.040 --> 00:37:12.590
Yoni Peleg: Where you can monitor it, and it's living in your environment and whatnot. But anyway…

227
00:37:12.750 --> 00:37:16.420
Yoni Peleg: This point is saying, in other words, the…

228
00:37:16.960 --> 00:37:30.870
Yoni Peleg: the release build and the debug builds are two completely different binaries, and as we saw in this presentation, you could have a sec fault in release that you don't have in debug. So, yeah, so especially if you're distributing your binaries to users.

229
00:37:30.870 --> 00:37:42.410
Yoni Peleg: You should run the full suite of tests that you have on your release artifacts, and don't simply rely on the fact that you ran CI on the same commit in debug mode.

230
00:37:42.500 --> 00:37:50.500
Yoni Peleg: Because the binary is vastly different, and although it should do, like, the same thing for the most part.

231
00:37:50.810 --> 00:37:59.350
Yoni Peleg: the assembly instructions are different, and you should test the exact same binary that you ship to your users.

232
00:38:00.410 --> 00:38:03.410
Yoni Peleg: So, yeah, so that's for… that's about it.

233
00:38:03.580 --> 00:38:20.449
Yoni Peleg: And the last, conclusion I have is, like, mainly a shout-out to the Rust team. They were very, very, welcoming, and they made sure I'm in the loop, and they took the time to explain all of this

234
00:38:20.460 --> 00:38:24.480
Yoni Peleg: Like, all the context to me, and, and they…

235
00:38:24.740 --> 00:38:41.120
Yoni Peleg: They make sure I'm as involved as I wanted in the conversation, and they're very… yeah, they're very nice, they're very professional. So yeah, so if you have something you want to get in touch with the Rust team about, don't hesitate to do that. They're all very nice.

236
00:38:41.300 --> 00:38:47.480
Yoni Peleg: And that's it for me, so thank you very much. I'll be now taking questions.

237
00:38:48.150 --> 00:38:52.499
Yoni Peleg: Let's take a look at the chat.

238
00:38:53.550 --> 00:39:09.840
Yoni Peleg: Yeah, can you also please share at which point you figured out it was indeed a compiler bug and not something else? Yeah, so, so this part was actually quite trivial, because, the trace of the,

239
00:39:10.310 --> 00:39:15.590
Yoni Peleg: The, segfold, like, didn't point…

240
00:39:15.700 --> 00:39:33.809
Yoni Peleg: at any line or even module that I changed in my specific commit. So it was completely unrelated, it wasn't even near any of my changes. So then the next thing I did is ran CI on fresh main.

241
00:39:34.020 --> 00:39:50.200
Yoni Peleg: like, the same commit as in the head of the… of main, and I, in fact, reproduced the bug, and that pretty much nailed it, that it doesn't have anything to do with my change, and…

242
00:39:50.380 --> 00:39:55.670
Yoni Peleg: And then I looked… I looked up what's different, and one of the first things you do is you check

243
00:39:55.960 --> 00:40:10.429
Yoni Peleg: the setup, like your machine, the, like, the CPU, memory, and versions of everything that you're running, and the Rust version was indeed different, so the next

244
00:40:11.100 --> 00:40:16.740
Yoni Peleg: And the next check I ran is to compare it with the previous ROS version, like,

245
00:40:16.740 --> 00:40:33.580
Yoni Peleg: I set… I pinned the Rust version to 1.96, and I re-run CI, and it passed, then I pinned it to 1.97, and I re-run CI and got the segfold. So at this point, I was pretty convinced that it's a bug in Rust, and that's where I went to…

246
00:40:33.640 --> 00:40:35.309
Yoni Peleg: To file an issue.

247
00:40:38.030 --> 00:40:39.749
Yoni Peleg: Yeah, any more questions?

248
00:40:50.360 --> 00:40:53.170
Yoni Peleg: You can also reach out on LinkedIn or,

249
00:40:53.370 --> 00:41:01.060
Yoni Peleg: Gmail, I guess, or if you're in the group chat of the ROS community, you can reach out on WhatsApp, I'm there.

250
00:41:01.330 --> 00:41:05.550
Yoni Peleg: Yeah, another question. How long did it take the Rust team to fix it?

251
00:41:06.300 --> 00:41:19.989
Yoni Peleg: About a weekend, I think I reported it on Thursday, and, like, maybe Saturday they released a patch, or maybe something like that. On the order of 2 or 3 days.

252
00:41:21.420 --> 00:41:23.740
Yoni Peleg: Which is pretty good, I think.

253
00:41:24.450 --> 00:41:30.039
Yoni Peleg: But we can take a look at the GitHub repo, like… the… there's the…

254
00:41:30.930 --> 00:41:41.020
Yoni Peleg: whatever it is, the link to… I'll attach the link to the actual GitHub issue, so you can see how it all,

255
00:41:41.160 --> 00:41:46.560
Yoni Peleg: how it all unrolled there. But I think it was a couple of days, yeah.

256
00:41:50.370 --> 00:41:51.880
Yoni Peleg: Do you happen to know?

257
00:41:52.320 --> 00:41:57.790
Yoni Peleg: If you were the first and only one encountering the bug,

258
00:41:58.060 --> 00:42:08.150
Yoni Peleg: as far as I know, I was the only one. It doesn't mean I was, in fact, the only one. I'm just the only one who reported it.

259
00:42:09.380 --> 00:42:14.420
Yoni Peleg: But yeah, I didn't see anyone else saying, yeah, it happened to me too.

260
00:42:14.540 --> 00:42:17.119
Yoni Peleg: Maybe I missed it, but

261
00:42:17.770 --> 00:42:35.399
Yoni Peleg: Again, as we saw in the presentation, it's quite a rare case, actually, and also the, like, the reproduce I gave them was, like, distilled from the exact codebase that we had there, because it's not exactly

262
00:42:35.530 --> 00:42:39.179
Yoni Peleg: How the reproduce showed up in my, like, in…

263
00:42:39.510 --> 00:42:47.359
Yoni Peleg: in their codebase. It was, like, a combination of our code with some external dependencies.

264
00:42:47.370 --> 00:43:05.150
Yoni Peleg: that play together, it was, like, it was a very, very, very edgy, edge case, so… I wouldn't expect many other users to catch it. And also, I caught it, like, a few hours after the version was released, so I'm probably…

265
00:43:05.220 --> 00:43:14.180
Yoni Peleg: Probably one of the first ones ever to encounter this, this edge case. Yeah, quite exciting in the, in, in the bad way.

266
00:43:14.270 --> 00:43:17.120
Yoni Peleg: But nonetheless, exciting, yeah.

267
00:43:29.450 --> 00:43:35.259
Gabor Szabo: Yeah, it's, it's a valid thing, among, very long, I think…

268
00:43:35.390 --> 00:43:41.080
Gabor Szabo: Among people who are new to programming, then when something is broken.

269
00:43:41.580 --> 00:43:46.020
Gabor Szabo: A compiler error or something, then they say, well, there is a bug in the compiler.

270
00:43:46.210 --> 00:43:49.909
Gabor Szabo: And it's almost never a bug in the compiler.

271
00:43:50.050 --> 00:43:50.840
Yoni Peleg: Yeah.

272
00:43:50.990 --> 00:43:52.610
Gabor Szabo: Except when it is.

273
00:43:53.140 --> 00:43:59.190
Yoni Peleg: Yeah, yeah, it's indeed rare, and… And also, Rust is…

274
00:43:59.370 --> 00:44:16.509
Yoni Peleg: quite mature, and we see all the, like, the biggest companies in the world picking it up quite seriously, which is great, and I love that very much. But nonetheless, it's still quite new in terms of languages.

275
00:44:17.770 --> 00:44:29.610
Yoni Peleg: And it's still very much evolving, and if you look at, like, the backlog of the Rust team, and what they plan to do, and what they're releasing, like, in the latest versions nowadays.

276
00:44:29.760 --> 00:44:48.380
Yoni Peleg: They're working on pretty big changes and pretty massive features, so it's expected that, the situation is still a little bit hairy, because, yeah, it's not as matured as, say, C or CPP, right? It didn't have…

277
00:44:48.380 --> 00:44:51.580
Yoni Peleg: Like, 5 or 6 decades to,

278
00:44:51.650 --> 00:44:56.449
Yoni Peleg: To get, to get as stable.

279
00:44:57.040 --> 00:45:01.829
Yoni Peleg: But yeah, it's not, it's not to say that the,

280
00:45:02.080 --> 00:45:08.130
Yoni Peleg: Though there are many bugs like that, of course, it's still… it's very rare, and the Rust team is doing a great job.

281
00:45:08.280 --> 00:45:22.650
Yoni Peleg: to, to keep a very high standard of, like… I still have very high confidence that my code will work when I update my Rust version, let's say like that, so yeah,

282
00:45:23.230 --> 00:45:26.099
Yoni Peleg: So yeah, it's rare, but it happens.

283
00:45:26.970 --> 00:45:41.249
Gabor Szabo: By the way, I have another comment that I just remembered earlier. You recommend people to fix the CR… fix everything, everything to a version.

284
00:45:41.630 --> 00:45:44.159
Gabor Szabo: I also have a recommendation.

285
00:45:44.300 --> 00:45:48.649
Gabor Szabo: that… I'm not sure if it's right, especially these days, but…

286
00:45:48.880 --> 00:46:01.489
Gabor Szabo: is to have a sort of a branch, a part of the CI that will also run your tests, everything, on the latest of everything.

287
00:46:02.250 --> 00:46:04.550
Yoni Peleg: Yeah, so actually, I think…

288
00:46:04.750 --> 00:46:11.180
Yoni Peleg: I had this in my mind, and I forgot to plug it in when I went through the conclusions, but yeah,

289
00:46:11.580 --> 00:46:26.749
Yoni Peleg: Some of it, and that's certainly what I did in many of the startups I worked for, is, like, to set up a scheduled job that pulls in latest versions of whatever, and open… simply open a PR with the update.

290
00:46:26.870 --> 00:46:38.719
Yoni Peleg: There's, like, tools for that nowadays. There's Dependabot, and for the rest versions, it's, like, a couple of lines of, GitHub Actions YAML file.

291
00:46:39.430 --> 00:46:44.700
Yoni Peleg: And simply open a PR and let CI run on the latest versions.

292
00:46:44.880 --> 00:46:58.189
Yoni Peleg: And yeah, that's how you can, like, monitor which changes are safe and which aren't. But one thing I'll say is, even… still, even if you have these scheduled jobs.

293
00:46:58.240 --> 00:47:12.500
Yoni Peleg: don't pull the, like, the latest versions as soon as they come out. Wait for a couple days even before you open a PR with them, because some of the vulnerabilities are… and actually,

294
00:47:12.920 --> 00:47:28.739
Yoni Peleg: quite significant portions of the vulnerabilities and CVEs that I see lately are targeting CI machines, so, so you're still, like, susceptible to the attack, even if you just open a PR. So, yeah, so…

295
00:47:28.800 --> 00:47:39.199
Yoni Peleg: First line of defense is simply wait a couple days, then open a PR, let it run on your CI, and then, if it's safe and you decide you want it, so, put it in, yeah.

296
00:47:41.820 --> 00:47:44.030
Gabor Szabo: Yeah, yes, exactly.

297
00:47:48.720 --> 00:47:57.640
Yoni Peleg: Yeah, anyway, I don't see any, newer questions, and as I said before, you can reach out to me, however you want, if you want to discuss it.

298
00:47:57.820 --> 00:48:02.549
Yoni Peleg: So yeah, so once again, thank you all very, very much.

299
00:48:03.400 --> 00:48:04.210
Gabor Szabo: That's interesting.

300
00:48:05.220 --> 00:48:13.599
Gabor Szabo: Yeah, thank you very much for the presentation, and thanks everyone who asked questions, and thanks everyone who didn't ask questions.

301
00:48:14.290 --> 00:48:22.380
Gabor Szabo: As I mentioned, you can, after we stop the recording, you can stay on, and we can keep the conversation.

302
00:48:22.750 --> 00:48:27.539
Gabor Szabo: without… off the record, exactly.

303
00:48:27.730 --> 00:48:31.429
Gabor Szabo: And everyone who is watching the video, please,

304
00:48:31.500 --> 00:48:48.650
Gabor Szabo: like it, and follow the channel, and below the video, find the link to the few… both the links about the… about the presentation, and to the future events, and… and register there, and I hope to see you at one of our future events.

305
00:48:48.650 --> 00:48:50.370
Yoni Peleg: Sure, bye-bye, YouTube.

306
00:48:50.370 --> 00:48:50.890
Gabor Szabo: Bye-bye.

307
00:48:50.890 --> 00:48:53.110
Yoni Peleg: After the recording.



