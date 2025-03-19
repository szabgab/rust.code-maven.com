---
title: "crum: Complex Numbers and Complex Matrices in Rust with Frans Slabber"
timestamp: 2025-03-19T07:30:01
author: szabgab
published: true
show_related: true
description:
tags:
todo:
---

[crum](https://github.com/fransslabber/crum/) ia a Rust crate for implementing complex numbers and matrices with a large focus on complex matrices and their use in numerical analysis.

[Frans Slabber](https://www.linkedin.com/in/frans-slabber/)

![Frans Slabber](images/frans-slabber.jpeg)

{% youtube id="AjgWnYuTfXc" file="2025-03-18-crum-complex-numbers-and-complex-matrices-in-rust.mp4" %}


## Transcript

1
00:00:00.570 --> 00:00:02.170
Frans: This meeting is being recorded

2
00:00:02.170 --> 00:00:15.080
Gabor Szabo: So Hi, and welcome to the codeme events meetings, the meet up group. And if you're watching the Youtube video, then to the codeme events, Youtube Channel. And my name is Gabor Sabo. I

3
00:00:15.270 --> 00:00:23.110
Gabor Szabo: I'm a self-employed. I help companies introduce rust and python, help them with training, help them with testing.

4
00:00:23.220 --> 00:00:35.710
Gabor Szabo: And I also organize events, both in person meetings in Tel, Aviv, and in Python for Python and and Rust, and also these online events in English.

5
00:00:36.290 --> 00:00:50.500
Gabor Szabo: in order to share knowledge between people all around the world. And I think it's a very good way to learn about all kind of things that we can't learn or ourselves

6
00:00:50.660 --> 00:01:08.890
Gabor Szabo: this time. We have Franz, who agreed to give this presentation, and you're going to introduce yourself, or he's going to introduce himself in a bit. I'm really happy that you agreed to give this presentation, and I thank everyone who joined us in this meeting

7
00:01:08.890 --> 00:01:25.200
Gabor Szabo: feel free to ask questions after the recording, we can stay around. So if there are things that you don't want to talk about during the recording and in the Youtube, video, that's fine, we can stay around after the meeting and have a little chat as well. That's the

8
00:01:25.230 --> 00:01:28.480
Gabor Szabo: advantage of actually being in the in the meeting.

9
00:01:28.710 --> 00:01:31.369
Gabor Szabo: And also you can ask questions.

10
00:01:32.320 --> 00:01:47.709
Gabor Szabo: and if you're watching the video, then please click the like button and follow the channel. So you get notified and also join our the call me events meet up group, so you will be able to see when we have a new meetings.

11
00:01:48.430 --> 00:01:53.809
Gabor Szabo: Franz. Welcome to this meeting. Please introduce yourself, and it's yours.

12
00:01:54.540 --> 00:01:57.089
Gabor Szabo: Share your screen, if you like, and whatever

13
00:01:57.090 --> 00:02:01.689
Frans: Yeah, thanks. Thanks. Kabor. And thanks for the opportunity.

14
00:02:02.283 --> 00:02:11.110
Frans: As as I just mentioned to you. Unfortunately, I've I've I haven't got a formal presentation. But I will talk through

15
00:02:11.230 --> 00:02:24.920
Frans: this sort of process that went into what I'm doing. So 1st of all, my name is Franz Bresler slabber. That's the full spectrum of names. And I'm a programmer living in South Africa.

16
00:02:25.370 --> 00:02:30.309
Frans: I've been a programmer since I can't remember

17
00:02:30.570 --> 00:02:49.090
Frans: since 94 mainly and and oh, okay, I work for myself as well. So I do various contracts, developing mainly in the technical sort of programming space solving.

18
00:02:49.620 --> 00:02:53.009
Frans: modeling problems, etc, stuff like that.

19
00:02:54.810 --> 00:03:00.240
Frans: So my main expertise has always been c plus plus.

20
00:03:00.930 --> 00:03:19.600
Frans: Recently I had moved into Golang a bit, and then I sort of out of interest started in rust. So rust is almost I haven't. How can I say? Used it commercially, but it's it's an interest, and it's a fascinating way for me to.

21
00:03:19.810 --> 00:03:23.493
Frans: I've got a strong interest in mathematics.

22
00:03:24.270 --> 00:03:34.070
Frans: especially how it relates to a numerical computing for use in in machine learning and stuff like that

23
00:03:34.230 --> 00:03:35.530
Frans: stuff like that

24
00:03:37.020 --> 00:03:48.970
Frans: very technical term. Okay, so 1st order of business, what I did is I'm gonna share screen, because I think that's the easiest way to to go about this. That's okay. Gabor.

25
00:03:49.230 --> 00:03:50.300
Frans: obviously.

26
00:03:50.300 --> 00:03:52.039
Gabor Szabo: Sure. Sure. Sure. Go ahead.

27
00:03:52.240 --> 00:03:55.820
Frans: Okay. So I'm not too familiar.

28
00:03:58.220 --> 00:04:00.240
Gabor Szabo: Should be a big green button

29
00:04:00.240 --> 00:04:01.989
Frans: Yeah, yeah, I've got it.

30
00:04:02.870 --> 00:04:10.210
Frans: Share. Oh, yeah. Oh, not seeing it. Select window. So I'm gonna select.

31
00:04:11.310 --> 00:04:16.779
Frans: Not that one know what that doing open store.

32
00:04:17.060 --> 00:04:25.299
Frans: allow visual studio. So I've got a very big screen. So please tell me if the resolution is okay.

33
00:04:25.300 --> 00:04:30.200
Gabor Szabo: But way too small the letters. So if you can enlarge the fonts, that would be nice

34
00:04:30.380 --> 00:04:34.192
Frans: Yeah, I can just zoom in. I think that works as well.

35
00:04:34.990 --> 00:04:36.410
Frans: Just a moment.

36
00:04:37.850 --> 00:04:39.590
Frans: No, it is.

37
00:04:41.280 --> 00:04:45.380
Frans: There is a yes, somebody wants to say something

38
00:04:45.380 --> 00:04:49.949
Gabor Szabo: No, no, I just going to turn it off it. It beeps when someone joins

39
00:04:50.150 --> 00:04:51.100
Frans: All right.

40
00:04:53.500 --> 00:04:55.160
Frans: Just a moment.

41
00:05:02.250 --> 00:05:05.009
Frans: I've done this before. Sorry guys.

42
00:05:05.010 --> 00:05:06.980
Gabor Szabo: So is it. A windows

43
00:05:07.418 --> 00:05:10.929
Frans: There we go. No, it's it's a Linux

44
00:05:11.355 --> 00:05:14.759
Gabor Szabo: For me. It's control shift plus you usually

45
00:05:14.760 --> 00:05:16.020
Frans: Yeah, there we go.

46
00:05:16.260 --> 00:05:17.920
Frans: That's slightly better.

47
00:05:17.920 --> 00:05:21.180
Gabor Szabo: A little bit more, a little bit more.

48
00:05:22.180 --> 00:05:25.310
Gabor Szabo: Yeah, more, more.

49
00:05:25.310 --> 00:05:26.210
Gabor Szabo: One more sure

50
00:05:27.545 --> 00:05:28.360
Frans: Wow!

51
00:05:29.330 --> 00:05:38.159
Gabor Szabo: Yeah, okay, thanks. That's now. No, at least I can see. I don't know if anyone else still needs larger. Please let us know. For me. It's it's fine

52
00:05:38.920 --> 00:05:46.420
Frans: Okay, good stuff. So basically, let's go. Let's start at the beginning.

53
00:05:46.770 --> 00:05:57.899
Frans: So the 1st thing I started playing with is complex numbers, and the idea is to write A. What in C would be a template class

54
00:06:00.670 --> 00:06:24.770
Frans: to write the equivalent in rust, and to actually have limiting it by means of traits. Okay, so sorry. Let me just take a step back here. The whole idea that I started out with as well is that I wanted to write something. One of the main ideas is that it's completely safe code.

55
00:06:24.820 --> 00:06:35.120
Frans: So I don't call into any under the covers, call into any unsafe libraries, any C or C plus plus libraries.

56
00:06:35.360 --> 00:06:50.489
Frans: Of course, that comes with a performance penalty because you gain quite a lot by. For example, if to do your matrix, arithmetic or matrix manipulation. To do that with

57
00:06:50.630 --> 00:06:56.999
Frans: pointer arithmetic is is generally much faster, but

58
00:06:57.150 --> 00:07:14.019
Frans: I try. I stayed away from it on purpose. So this is all safe for us. Code number one. And also I. Then obviously the closest I could come in terms of performance is by using iterators extensively.

59
00:07:17.140 --> 00:07:25.969
Frans: So, okay, having said that, I started out with the basic ideas of a generic complex number class.

60
00:07:26.310 --> 00:07:37.120
Frans: and there are there are libraries out here that out there that do it, and probably do it much better. So I've got no claim that I'm doing wonderful stuff.

61
00:07:37.540 --> 00:07:43.790
Frans: and as you can see, this will look quite a bit outstanding. But okay, sorry. Let me just go to the top.

62
00:07:44.337 --> 00:07:52.080
Frans: So what I started up is just a basic structural and imaginary, with a type of generic type T.

63
00:07:52.420 --> 00:08:03.800
Frans: And because I wanted to make this compliant with num traits float and integer actually specifically

64
00:08:05.580 --> 00:08:12.919
Frans: I had to define all the traits now, some of them I haven't completed yet, because I didn't

65
00:08:13.530 --> 00:08:21.930
Frans: use the so if we go down here you will see there's some to do's in. There.

66
00:08:22.470 --> 00:08:23.499
Frans: there we go.

67
00:08:25.446 --> 00:08:26.940
Frans: Which is

68
00:08:27.110 --> 00:08:36.119
Frans: probably needs to be fleshed out at some point. So again, just reminding you that this is purely a fun project for me.

69
00:08:36.270 --> 00:08:54.100
Frans: I'm not beholden to anyone to have a finished, polished product. Of course I would like to move to that point, but hey? It's all dependent on when I have time or not. So that's my last excuse I'm going to make

70
00:08:54.480 --> 00:09:03.099
Frans: from us this evening. Okay, so complex numbers, not a big thing. There are things out there pretty much

71
00:09:04.980 --> 00:09:12.970
Frans: And I mean, I'm not going to dwell on this too too much. This is, you can imagine, this is very basic kind of coding.

72
00:09:13.370 --> 00:09:20.389
Frans: right? So, of course, that the aim of that was to start moving into matrices.

73
00:09:20.690 --> 00:09:23.740
Frans: So then

74
00:09:26.980 --> 00:09:34.670
Frans: I start. Okay, sorry. I just want to get you the matrix declaration apologies for this scrolling around.

75
00:09:34.890 --> 00:09:36.490
Frans: Oh, boom!

76
00:09:39.800 --> 00:09:48.610
Frans: So what I decided on in terms of looking towards the future

77
00:09:48.840 --> 00:09:53.970
Frans: to store the all the data in a flat vector.

78
00:09:54.490 --> 00:10:04.109
Frans: generic vector and to make it row dominant. So what that means is that if you if you look at a matrix, and

79
00:10:04.240 --> 00:10:11.780
Frans: you look at the 1st row that would be the first.st So say you've got a a

80
00:10:14.170 --> 00:10:17.920
Frans: in rows. Sorry in columns. So the 1st

81
00:10:18.150 --> 00:10:33.629
Frans: N entries in the vector is the 1st row and then the second N entries that follow onto that is the second row, so I don't know if that creates a easy visualization. But that's for those that don't.

82
00:10:34.240 --> 00:10:36.849
Frans: I haven't heard about road dominance, anyway.

83
00:10:37.090 --> 00:10:50.160
Frans: So and that was sort of then it started getting interesting. So the normal things like adding a matrix subtracting a matrix, extensive use of of just

84
00:10:50.380 --> 00:10:54.589
Frans: iterator and all the the goodies that it has with it.

85
00:10:58.050 --> 00:11:03.670
Frans: So that was pretty so interesting, but pretty straightforward.

86
00:11:03.830 --> 00:11:11.610
Frans: And then I added, stuff like, get the diagonal as a vector, etc, that kind of thing and getting a skew diagonal.

87
00:11:11.920 --> 00:11:21.890
Frans: Also, I find that I wanted to. If you have a matrix of floats, you want to round them off to some decimal place which is very nice

88
00:11:22.040 --> 00:11:48.269
Frans: for later manipulation. I find I needed upper triangular matrices. So if any of these things, these are just different forms of matrices, so upper triangular is all the elements above. The diagonal is non-zero and the one below are 0 kind of thing, and as that is, and lower triangle is triangular is the opposite.

89
00:11:48.770 --> 00:11:53.205
Frans: And we have the famous identity matrix. So

90
00:11:54.660 --> 00:12:07.240
Frans: if this is so, I I'm assuming some level of linear algebra background here. Assumption.

91
00:12:08.370 --> 00:12:16.309
Frans: Then oh, checking if it is an identity, that kind of thing. Okay, so that's pretty much

92
00:12:16.410 --> 00:12:24.000
Frans: sort of what you would think would it would have as a standard library then? Now.

93
00:12:24.200 --> 00:12:27.420
Frans: my! Oh, in this transpose.

94
00:12:30.900 --> 00:12:35.089
Frans: of course. Now this raises very interesting

95
00:12:35.640 --> 00:12:50.880
Frans: when you write this because you have to keep in mind that you have a flat vector storing the stuff. So if you have to do a transpose, you have to carefully work with what they call strides.

96
00:12:51.610 --> 00:12:53.380
Frans: In other words,

97
00:12:55.690 --> 00:13:05.180
Frans: it is normally the offset that you would get to an element in a specific row and column in a flat vector, considering a flat, vector.

98
00:13:06.510 --> 00:13:10.689
Frans: we will talk more about strides when we get to tenses.

99
00:13:11.050 --> 00:13:11.810
Frans: Okay.

100
00:13:11.940 --> 00:13:31.359
Frans: so this was sort of sub matrix. And you can see there's some tests here in a way. You know, I wrote a macro for declaring a matrix like that which is pretty straightforward, and then to get the sub matrix sticking to having this inclusive ranges that you can specify

101
00:13:31.430 --> 00:13:43.310
Frans: from this row to that row, and from this column to this this column inclusive, and it'll give you the sub matrix out of this chunk of bigger matrix, anyway. So that's sort of

102
00:13:43.980 --> 00:13:45.730
Frans: what's going on here.

103
00:13:46.890 --> 00:13:52.510
Frans: Of course, this whole process also to keep in mind is that this is me.

104
00:13:53.530 --> 00:13:59.419
Frans: As you work with iterators, more and more it becomes clearer and

105
00:13:59.540 --> 00:14:02.300
Frans: probably less clunky working with it.

106
00:14:02.680 --> 00:14:03.530
Frans: Okay?

107
00:14:06.200 --> 00:14:31.759
Frans: So also doing stuff like inserting a row, inserting a column, etc. Etc. Then, of course, there becomes augmentation as we adding a sub matrix on top as the description is up top here, the top left corner of a larger matrix. So when we when I start writing matrix, how can I say routines? This becomes important.

108
00:14:32.480 --> 00:14:43.590
Frans: determinant over 2 by 2 and then converting a real to a complex matrix. Okay, so now we start seeing why I started with complex. Because I want you to work

109
00:14:43.930 --> 00:14:50.749
Frans: my 1st aim in doing with this, or the 1st sort of goal that I had was working with complex matrices.

110
00:14:51.300 --> 00:15:13.380
Frans: and there was a very specific reason that I wanted to do it, which I'll show later, or just now put it that way. So now with matrices, you have very famous things like Lu decompositions with Gauss elimination. So this was so now it became really interesting. And I mean, I really actually

111
00:15:13.800 --> 00:15:20.900
Frans: love this stuff sadly, I do other things in life as well.

112
00:15:21.230 --> 00:15:28.340
Frans: This a side note. Humorous side note, anyway. So so that was sort of

113
00:15:29.570 --> 00:15:34.759
Frans: this. I think this was my 1st big achievement. Was it lu decomposition?

114
00:15:36.920 --> 00:15:41.400
Frans: With a partial pivot, fascinating stuff.

115
00:15:41.520 --> 00:16:03.260
Frans: So what was also what I really liked about this is I did, applied math degree long ago. But I you know you completely forget things, because if you don't work with it or not completely forget, but it's so far removed, and what was very nice is, I had to go and sort of read up on how these things work, especially

116
00:16:03.400 --> 00:16:22.099
Frans: from a numerical computing point of view, because that is different to a sort of straightforward, academic, theoretical application of matrices. Because, you know, computing isn't exact. So you always have rounding errors, and you always have to work to a certain precision.

117
00:16:22.330 --> 00:16:25.190
Frans: Anyway, that was linear solving.

118
00:16:25.620 --> 00:16:33.020
Frans: Have I got too complex yet? Sorry sorry about that. I just want to go up this.

119
00:16:33.570 --> 00:16:37.899
Frans: And we saw a real square matrix. No, we're still unreal.

120
00:16:38.410 --> 00:16:40.819
Frans: Okay? So at some point.

121
00:16:48.230 --> 00:16:50.729
Frans: sorry, it must be completely boring

122
00:16:51.480 --> 00:16:55.720
Frans: sort of looking at what I did, quite amazed that I wrote it all.

123
00:16:57.770 --> 00:17:05.419
Frans: Okay. So ultimately, the point was what to calculate eigenvalues of a matrix.

124
00:17:06.140 --> 00:17:21.600
Frans: So and here I established then complex matrix specializations, that where your generic item is a complex matrix of

125
00:17:21.829 --> 00:17:26.040
Frans: T, which you know satisfies numb traits.

126
00:17:30.390 --> 00:17:38.150
Frans: So there was just some things here, and then I think the ultimate is where I do.

127
00:17:39.371 --> 00:17:44.489
Frans: There's a random, complex matrix. By the way, kind of things is where I actually

128
00:17:45.960 --> 00:17:48.319
Frans: own a complex conjugate.

129
00:17:49.440 --> 00:17:52.770
Frans: So this is all to do with complex matrices. And

130
00:17:53.680 --> 00:18:01.819
Frans: this was also beautiful to do complex matrix. QR decomposition using householder transforms.

131
00:18:02.160 --> 00:18:08.325
Frans: This was fantastic. So, by the way, all of this has been

132
00:18:09.260 --> 00:18:13.869
Frans: how can I say ratified against Matlab

133
00:18:15.560 --> 00:18:20.830
Frans: because I wanted to make sure that I'm not putting nonsense out there.

134
00:18:28.210 --> 00:18:31.269
Frans: this is QR complex.

135
00:18:32.990 --> 00:18:44.350
Frans: QR decomposition with householder transforms. So this is all used when you're trying to find the eigenvalues of a matrix. I just want to go right down where I actually do the

136
00:18:46.520 --> 00:18:50.909
Frans: that's the household that transform scar, duck, decomposition

137
00:18:56.570 --> 00:18:58.500
Gabor Szabo: Yeah, I have a question. Franz

138
00:18:58.500 --> 00:18:59.380
Frans: Yes.

139
00:18:59.380 --> 00:19:00.393
Gabor Szabo: Do you?

140
00:19:01.680 --> 00:19:07.819
Gabor Szabo: So, besides writing the the crate, did you make any use of it?

141
00:19:07.820 --> 00:19:15.380
Frans: No, no, no. So so the that's for yourself or yeah. So I'm hoping

142
00:19:16.220 --> 00:19:28.349
Frans: to use it at some point. But at the moment this was purely, as I said in the beginning, for myself, as an entertainment, as something fascinating for me to do. And and

143
00:19:28.450 --> 00:19:43.880
Frans: I enjoy mathematics. And I enjoy solving complex software problems. And a lot of this coding is is

144
00:19:44.490 --> 00:19:54.709
Frans: I. Maybe I'm just old and slow. But I found it very involved and and quite a lot of pencil sketches had to be done

145
00:19:55.260 --> 00:19:58.510
Frans: to figure out how to do this.

146
00:19:58.830 --> 00:20:06.440
Gabor Szabo: I mean, I we use generics and all kind of interesting parts of of rust.

147
00:20:06.600 --> 00:20:13.509
Gabor Szabo: Yes. How did you? Did did you implement anything similar in in CC plus earlier

148
00:20:14.060 --> 00:20:15.890
Frans: No, I've either.

149
00:20:15.890 --> 00:20:20.959
Gabor Szabo: How do you compare what the feeling of of writing rush? That's what I'm trying to ask

150
00:20:21.190 --> 00:20:33.960
Frans: Much, much more difficult. And look, I mean, obviously, I've written C plus plus for what 20 years, you know. So and template programming template libraries is is

151
00:20:34.350 --> 00:20:41.179
Frans: is pretty because you have all these these restrictions, these these things.

152
00:20:41.360 --> 00:21:01.450
Frans: you know making. And if you so, for example, if you are doing an eigen eigenvalue decomposition you are using previous QR or Lu decompositions that you've written to actually facilitate that finding that eigenvalue and

153
00:21:01.700 --> 00:21:24.719
Frans: you have to be so. How can I say? Careful to have the right traits from the bottom up? Otherwise you can't, you know, when you get to the top one. You don't satisfy the traits required to to do it. Kind of thing. If if that makes some sort of sense, I just find it very.

154
00:21:25.200 --> 00:21:49.570
Frans: I mean, I appreciate what it's trying to do here in terms of forcing you to be very, very strict. What you can do. And obviously that is the selling point of rust. Well, one of the selling points of rust is the is safety, and and this really forces you to use it in a very specific way.

155
00:21:50.130 --> 00:22:10.590
Frans: So I do appreciate that. But I may. I mean, look, I'm a youngster in rust if you can put it like that. So maybe that's why I find it difficult, but I found it much more involved than than C plus plus. And of course, and I had to learn the whole iterator setup.

156
00:22:10.860 --> 00:22:15.950
Frans: You know what you can do with iterators, what it actually means. And and

157
00:22:16.330 --> 00:22:26.499
Frans: you know, in in c plus plus, you would have just done as I said. Pointer. Arithmetic, you know, on, on dynamic arrays, or something or static arrays doesn't matter, but

158
00:22:26.660 --> 00:22:30.999
Frans: I think it would be pretty much more straightforward.

159
00:22:32.640 --> 00:22:37.470
Frans: So so this? Why, this was challenging for me to be honest, you know

160
00:22:37.710 --> 00:22:43.045
Gabor Szabo: Do you want to go over the the details of one of the functions to see? So we can. We can,

161
00:22:43.300 --> 00:22:47.157
Frans: Well, I'm I'm just trying to get to

162
00:22:47.640 --> 00:22:51.220
Gabor Szabo: For a simple function, and then, maybe later on, a more complex one.

163
00:22:51.630 --> 00:22:52.110
Frans: Okay.

164
00:22:52.110 --> 00:22:54.929
Gabor Szabo: So so we can have a feeling of of a

165
00:22:54.930 --> 00:22:55.630
Frans: Yeah.

166
00:22:55.630 --> 00:22:56.880
Gabor Szabo: How this code looks like

167
00:22:57.360 --> 00:23:01.159
Frans: Mother, okay. So we out there. So I just want to

168
00:23:01.160 --> 00:23:04.340
Gabor Szabo: I suppose I don't know. Difficult

169
00:23:04.340 --> 00:23:08.539
Frans: Okay, yeah, I transpose this shit.

170
00:23:18.680 --> 00:23:25.519
Frans: There we go. Okay, okay, so this is

171
00:23:26.400 --> 00:23:32.560
Frans: so remember, vector matrix is a flat vector

172
00:23:32.830 --> 00:23:40.880
Frans: so basically, what I do is when I come into this function, I just declare a mutable vector

173
00:23:41.050 --> 00:23:51.370
Frans: a generic vector of type T, which is what the original type is, and it's declared with the capacity of the one that I'm entering in. So

174
00:23:53.280 --> 00:24:03.450
Frans: just to go back to something I said previously is that I was learning as well. So there may be some way of doing this in place.

175
00:24:03.640 --> 00:24:16.490
Frans: But this I just declared a new vector, kind of a result vector, which you can see. I later wrap it into the the matrix that I return, if you like.

176
00:24:17.110 --> 00:24:23.100
Frans: So and then so for each row

177
00:24:25.310 --> 00:24:31.320
Frans: index in the number of columns. So we have number of columns. So this is for each item in the row.

178
00:24:37.140 --> 00:24:39.670
Frans: I have to really scratch my brain here.

179
00:24:44.560 --> 00:25:00.700
Frans: Oh, that's right. I use the skip iter to actually skip the actual offset so that I don't want to actually use this example because this doesn't really show using

180
00:25:00.830 --> 00:25:03.370
Frans: skip, and

181
00:25:03.720 --> 00:25:12.029
Frans: the the iterator function skip and step by which I actually use quite a lot. So anyway. But we can continue with this.

182
00:25:12.340 --> 00:25:15.819
Frans: Then we have. We just

183
00:25:16.680 --> 00:25:23.319
Frans: use a skip it iterator to get to the correct one. Because, remember, if you have.

184
00:25:23.570 --> 00:25:30.109
Frans: where is the example? Here, there we go. So we want to transpose that. So that is the row

185
00:25:30.620 --> 00:25:32.929
Gabor Szabo: And we want to make the row

186
00:25:33.070 --> 00:25:39.100
Frans: The 1st column, right? So you've got to go through the 1st row and write the 1st column.

187
00:25:39.540 --> 00:26:08.629
Frans: But the problem being that you're writing it into a row dominant, vector which is what this result vector, is. So you've got to go and say, if I transpose this, you've got 1, 2, 7 going down and 3, 4, 8 going down. Okay, so you have one and 3 being the 1st row now. So one and 3 will be the 1st 2 elements of the new vector

188
00:26:11.640 --> 00:26:13.690
Frans: here that I'm writing.

189
00:26:16.230 --> 00:26:26.350
Frans: So I I. So you can see in the result. Vector sorry I'll be with you. Now, we actually, we actually just build it up, row by row.

190
00:26:28.200 --> 00:26:28.820
Gabor Szabo: Right.

191
00:26:31.090 --> 00:26:32.489
Frans: Yes, go ask a question

192
00:26:32.490 --> 00:26:39.690
Gabor Szabo: No, I was. I was just wondering if if if the transpose is not just taking the

193
00:26:40.250 --> 00:26:44.599
Gabor Szabo: Xy element in the matrix and put it in. Put it in the YX

194
00:26:44.600 --> 00:26:52.729
Frans: Right. Yes, you can. So if you which, if you have a way of accessing the elements XY,

195
00:26:53.000 --> 00:26:56.640
Frans: which I do have up here, but I'm trying to

196
00:26:56.640 --> 00:26:59.800
Gabor Szabo: An array. Right? The whole thing is just one longer

197
00:26:59.800 --> 00:27:06.580
Frans: Well, well, it's it's resembling an array, but it is stored as a flat vector that that

198
00:27:06.580 --> 00:27:07.880
Gabor Szabo: Vector, sorry. Correct.

199
00:27:07.880 --> 00:27:25.889
Frans: Yeah, flat. Vector so the thing is that I can write this by just saying, for each row, for each column, swap, go get element Xy, and make it element. Yx, and I have the accessors.

200
00:27:26.390 --> 00:27:30.819
Frans: I think I've got it. Sorry. Let me just find it up here.

201
00:27:33.410 --> 00:27:35.800
Frans: Where is? Oh, there index.

202
00:27:36.430 --> 00:27:37.559
Frans: So I've got

203
00:27:37.790 --> 00:27:51.299
Frans: index and index mutable, which will give me access to the the actual XY coordinate, if you like, or the row column coordinate, and then just make just using index to set it to

204
00:27:51.470 --> 00:27:53.630
Frans: to Yx, if you like.

205
00:27:54.050 --> 00:27:58.930
Frans: But that means that you are calling this routines many times.

206
00:27:59.260 --> 00:28:01.090
Frans: Okay in a loop.

207
00:28:01.420 --> 00:28:11.079
Frans: So I would rather, for the sake of efficiency and speed. When I look for which I've lost now.

208
00:28:20.070 --> 00:28:41.300
Frans: I would rather do it directly on the vector and pull chunks out of it than actually. So I'm saving a little bit. Okay, not much, I guess. But this to me, is more efficient than just sitting in 2 for loops and and swapping rows and columns around row and column items around.

209
00:28:42.530 --> 00:28:46.300
Frans: I could be wrong, but that was just my thought on this

210
00:28:50.230 --> 00:28:52.450
Frans: sort of happy Gabor

211
00:28:53.320 --> 00:28:58.680
Gabor Szabo: I'm trying to understand. Yeah, the skipper. What? What is that

212
00:29:00.180 --> 00:29:03.020
Frans: This is, I defined it up here.

213
00:29:11.810 --> 00:29:30.510
Frans: So that is just a way of defining an iterator that skips a certain step size through so you could. Instead of using this. And as I said this, I used in the beginning, and later on I use the iterative actual iter functions step by and skip.

214
00:29:30.690 --> 00:29:33.580
Frans: which is, which would do the same thing as this

215
00:29:35.330 --> 00:29:36.120
Gabor Szabo: Okay.

216
00:29:38.630 --> 00:29:45.700
Frans: So what I'm actually saying subtly is that this would probably needs to be rewritten.

217
00:29:48.700 --> 00:30:04.450
Frans: But for now it works okay. Because and it will become clear. Why, I just sort of went on without really being too fussed about this. Okay, so I don't.

218
00:30:04.640 --> 00:30:06.050
Frans: Okay, hang on.

219
00:30:06.450 --> 00:30:10.980
Frans: Let's get some order here, go!

220
00:30:11.350 --> 00:30:12.679
Frans: Where was that again?

221
00:30:13.250 --> 00:30:14.770
Frans: Can't remember. There we go.

222
00:30:15.100 --> 00:30:16.799
Frans: Okay. So

223
00:30:17.570 --> 00:30:28.179
Frans: that's a very easy function, which, as you rightly say, probably needs to be rewritten. I'm just trying to think if there's not something else that is probably better.

224
00:30:29.950 --> 00:30:33.139
Frans: Sub matrix. You see. Yeah.

225
00:30:33.260 --> 00:30:40.280
Frans: okay. So again, the idea is, you want to grab a chunk out of this. Okay, that's not.

226
00:30:40.660 --> 00:30:42.920
Frans: You want to grab

227
00:30:42.920 --> 00:30:43.450
Gabor Szabo: Rectangle.

228
00:30:43.450 --> 00:30:57.590
Frans: From here here. Yeah, you want to grab a little block out of it, and the way you specify it, you say inclusive. I want to from column 3 to column 7, and from Sorry row.

229
00:30:57.720 --> 00:31:06.859
Frans: row, row column. Always work, row column. So row 3, 2, including row 7, column 2. So you can. It's a little slice out of there.

230
00:31:10.720 --> 00:31:14.199
Frans: So basically, we just check that. It's the correct rows.

231
00:31:14.540 --> 00:31:15.530
Frans: Right?

232
00:31:15.690 --> 00:31:19.100
Frans: Iterate. Get an index going enumerate.

233
00:31:19.230 --> 00:31:22.759
Frans: Then we will skip.

234
00:31:26.070 --> 00:31:42.990
Frans: So we want to position ourselves. So if you imagine just visually, if you want to extract a chunk out of here and remember that it's stored row dominant. You want to 1st get to this row, and you can just strip out

235
00:31:43.160 --> 00:31:47.740
Frans: the number of row elements if you like.

236
00:31:47.860 --> 00:31:57.340
Frans: because they would, they would be adjacent in the to each other in the storage. Vector so this is this will be straightforward

237
00:31:57.470 --> 00:31:58.409
Frans: if you like.

238
00:31:58.810 --> 00:32:01.041
Frans: So you would

239
00:32:02.670 --> 00:32:18.170
Frans: remember this all up here before you get to this element here. This all will be in a flat. Vector so it's a row. Then the next row next row. So you've got to skip the number of columns times up to this row.

240
00:32:18.410 --> 00:32:22.830
Frans: and then you have to skip what the index is that you want to extract.

241
00:32:23.210 --> 00:32:26.400
Frans: So you want to skip 3 rows

242
00:32:26.540 --> 00:32:29.400
Frans: times how many columns is in a row.

243
00:32:29.550 --> 00:32:41.559
Frans: so that, and then you want to add on another little piece until you get here. So that positions you here. So I think you can see that exactly what I've done here.

244
00:32:42.140 --> 00:32:46.410
Frans: the problem, not the problem. But something that

245
00:32:46.710 --> 00:32:56.590
Frans: probably could be changed is that I'm using 0 index. Sorry one indexing for use. But

246
00:32:56.690 --> 00:33:00.270
Frans: obviously it starts at 0. It's 0 indexed.

247
00:33:00.520 --> 00:33:05.830
Frans: So that's why I always have these minus one nonsense, and it gets a bit

248
00:33:06.520 --> 00:33:18.489
Frans: tedious. But anyway, Beta estimate the number of columns times the number of rows above the element. And then, plus where the the column is that we want to extract.

249
00:33:20.020 --> 00:33:24.030
Frans: you can see calls. Start is the 1st one here in the range.

250
00:33:24.950 --> 00:33:28.020
Frans: Okay, so is, is that sort of reasonably clear

251
00:33:28.360 --> 00:33:29.700
Gabor Szabo: Yeah, I think so.

252
00:33:30.450 --> 00:33:33.470
Frans: What I'm doing right? So then.

253
00:33:33.470 --> 00:33:36.420
Gabor Szabo: We're using assert to to verify the

254
00:33:37.510 --> 00:33:38.200
Frans: Yes.

255
00:33:38.420 --> 00:33:51.300
Frans: Well, this is just to make sure that I'm not specifying in my rows and columns something that would sit outside. You know that there isn't enough rows for it to sit in here.

256
00:33:51.540 --> 00:33:52.429
Frans: if you like.

257
00:33:54.990 --> 00:34:03.689
Frans: In other words, I'm not. I'm not wanting to extract this submatrix that is larger in some dimension than the matrix. I'm given

258
00:34:04.520 --> 00:34:05.230
Gabor Szabo: Right.

259
00:34:05.800 --> 00:34:08.346
Frans: Alright. So that's all the sort of

260
00:34:09.250 --> 00:34:12.260
Frans: stuff that I this says that I do there.

261
00:34:14.860 --> 00:34:23.259
Frans: So then, the step by jeez, I'm trying to think what I did here.

262
00:34:26.800 --> 00:34:33.600
Frans: I know I extract little vectors, and then I flatten them. Obviously, this is what's happening here.

263
00:34:34.132 --> 00:34:35.098
Frans: Sorry about that.

264
00:34:35.560 --> 00:34:42.690
Frans: Step by. So I get to the right position. Then I start iterating.

265
00:34:51.400 --> 00:34:57.200
Frans: just thinking, why am I not iterating by one stepping by 1 1st of all?

266
00:35:03.860 --> 00:35:09.729
Frans: Oh, well, guys, sorry I should have spent more time prepping it so I can give better answers

267
00:35:11.410 --> 00:35:16.200
Gabor Szabo: It's fun reading code, even if it even if it's yours.

268
00:35:16.730 --> 00:35:23.520
Frans: Yeah, it's it's it's as I said, I haven't spent time on it, and I've been so in deep in

269
00:35:24.050 --> 00:35:29.630
Frans: trying to find a memory leak on a big golang server. And it's

270
00:35:29.810 --> 00:35:33.599
Frans: it's yeah. My mind has just not been on this at all.

271
00:35:35.780 --> 00:35:57.839
Frans: okay, anyway, if there's real interest. We can come back to this at some point later, when I will make time from the beginning of April, I have more time again, so maybe we can make another arrangement. I can look at this so maybe see this as a sort of a high level view to start off with.

272
00:35:57.980 --> 00:36:03.260
Frans: anyway. So the moving on from this.

273
00:36:05.090 --> 00:36:10.300
Frans: which I hadn't really considered in the beginning. But it became obvious

274
00:36:10.560 --> 00:36:30.679
Frans: that if I wanted to something that would be actually useful in terms of machine learning, I would have to move into tensors. So tensors is just a higher dimensional form of a matrix or a higher dimensional form. A matrix is a higher dimensional form of a vector

275
00:36:30.960 --> 00:36:40.019
Frans: and tensor is then a step up from that so generic N dimensions or rank.

276
00:36:43.790 --> 00:36:45.740
Frans: So this is where

277
00:36:46.500 --> 00:36:54.380
Frans: I actually did the last, my last work, and it's also probably the most from a rust perspective, the most mature.

278
00:36:56.310 --> 00:37:04.740
Frans: So again, I declare, a tensor object generic tensor object using a flat data structure.

279
00:37:07.785 --> 00:37:16.550
Frans: I have the shape of the matrix. Okay, now, I just want to actually hold there and and interrupt myself again and say that

280
00:37:17.310 --> 00:37:19.400
Frans: what I actually

281
00:37:19.630 --> 00:37:31.460
Frans: was trying to do, and I actually had to go back and change the names a bit here because I had the right idea. But I had different names for this. I think I had dims here instead of shape.

282
00:37:31.780 --> 00:37:40.189
Frans: But then I've done quite a bit of work in with tensorflow in, in python, etc. And

283
00:37:40.590 --> 00:37:44.839
Frans: and in the array, and numpy sorry.

284
00:37:45.210 --> 00:38:00.589
Frans: So I had the back of my mind. It suddenly came to me. I want to write something in rust that somebody with a python machining background would feel comfortable in well, reasonably comfortable in using.

285
00:38:00.720 --> 00:38:05.880
Frans: I don't know if that's a pipe dream. But that's sort of where I was going with this.

286
00:38:06.020 --> 00:38:08.569
Frans: So that's why I changed this to shape

287
00:38:09.390 --> 00:38:13.459
Frans: and strides I had from the beginning. So

288
00:38:14.780 --> 00:38:30.030
Frans: let's the go. So again, I'm sitting with row dominance. But whereas previously in a matrix, you have a row, followed by a row, followed by a row. You now have a more complex setup, where you can have

289
00:38:30.540 --> 00:38:35.619
Frans: in dimensions nonspecific. In other words.

290
00:38:36.060 --> 00:38:45.810
Frans: you. You can access the the way that you access an element becomes very much more obtuse

291
00:38:46.090 --> 00:38:55.260
Frans: if I can put it like that. And so that's where why, in the beginning, when I establish the struct I calculate strides

292
00:38:55.520 --> 00:39:05.800
Frans: now, strides is a way that you can use to calculate offset in the vector to get to an element

293
00:39:06.030 --> 00:39:14.780
Frans: of a certain coordinate. Given a coordinate, you can use strides to get to that element, I will demonstrate it shortly. How this works.

294
00:39:16.395 --> 00:39:17.370
Frans: So

295
00:39:18.050 --> 00:39:36.500
Frans: that was pretty straightforward, and in the next line comes to exactly what I've been talking about. So if you have an object declared like this with when you declare a new one, it automatically calculates the strides because it has got the dimensions, the shape, if you like.

296
00:39:37.070 --> 00:39:51.790
Frans: So basically, a stride, is the index of a coordinate. And you can see the coordinate you specify as a, vector because it can be of any dimension

297
00:39:52.030 --> 00:40:06.499
Frans: up to say, 1520, 30. Whatever pick a number. So obviously a good way is to present it as a vector you could probably find other ways of doing this. So here I specify, coordinate as a. Vector

298
00:40:06.640 --> 00:40:07.770
Frans: and

299
00:40:07.950 --> 00:40:19.079
Frans: we can. We can look at an example. I see I don't have an example up here. But yeah, we can look an example. So. But more importantly, you can see that I take the data, which is a vector.

300
00:40:19.240 --> 00:40:27.120
Frans: And I stepped through the coordinates that I were given the spectre. I stepped through it.

301
00:40:27.310 --> 00:40:46.150
Frans: and at the same time I'm stepping through the strides because the number of strides. This vector equals the number of coordinates. And I just do accumulated multiplication. So strides are set up that I can actually get exactly to the offset

302
00:40:46.260 --> 00:40:48.999
Frans: in the vector for this coordinate.

303
00:40:51.230 --> 00:41:01.619
Frans: it takes a bit of thinking, maybe to visual right? You can't really visualize because you're starting to work in. You know, you don't have 2 dimensions as in the matrix.

304
00:41:02.020 --> 00:41:10.629
Frans: But this is, if you think about it long enough, it will become obvious. Well, not obvious, but

305
00:41:10.830 --> 00:41:11.990
Frans: what I'm doing.

306
00:41:14.830 --> 00:41:23.189
Frans: So so there's just a mutable setting that you can assign values back.

307
00:41:25.205 --> 00:41:26.690
Frans: Right?

308
00:41:27.000 --> 00:41:29.150
Frans: So here is the sort of

309
00:41:29.410 --> 00:41:36.050
Frans: generic implementation of tensors where you have stuff like.

310
00:41:37.735 --> 00:41:40.570
Frans: Oh, no. Sorry. My apologies, that's not

311
00:41:44.750 --> 00:41:45.927
Frans: yeah. That is

312
00:41:46.850 --> 00:41:58.639
Frans: what am I saying now, here, standard functions. So anyway, this is just what I wrote, because I got tired of. I wanted to display tenses properly.

313
00:41:58.930 --> 00:42:02.689
Frans: in other words, like python would.

314
00:42:05.095 --> 00:42:08.398
Frans: So this is the sort of

315
00:42:09.370 --> 00:42:15.580
Frans: display that I that I wrote formatting it properly.

316
00:42:16.730 --> 00:42:19.170
Frans: We'll get to that in a moment as well.

317
00:42:19.480 --> 00:42:24.209
Frans: I'll actually write. We can do a real time example of of using this.

318
00:42:25.200 --> 00:42:31.139
Frans: I'm much more comfortable talking about tenses, because it's the last thing I did. So it's probably more recent in my mind.

319
00:42:31.980 --> 00:42:44.889
Frans: recent being a month and a bit ago. Okay, so these are the general functions new declaring it. So you specify a shape, you specify basically all the dimensions in the vector and then you give it the data. Vector

320
00:42:45.840 --> 00:42:54.400
Frans: so the data vector is flat, but the shape gives it its tensor characteristics if you like.

321
00:42:55.330 --> 00:43:12.430
Frans: So you can see when you declare new, you, I can easily calculate the strides, because I have the dimensions and the strides are simply just an accumulated multiplication of

322
00:43:16.820 --> 00:43:22.610
Frans: for each one. You calculate one less, I think.

323
00:43:27.770 --> 00:43:31.860
Frans: and you put it into the strides. So this is what I'm doing here.

324
00:43:32.070 --> 00:43:36.640
Frans: Yeah. So I'm starting at one where index is one.

325
00:43:36.840 --> 00:43:41.920
Frans: So this the zeroth stride is skipping. 0.

326
00:43:43.750 --> 00:43:47.540
Frans: Sorry one. Yeah. Started one skip one

327
00:43:49.450 --> 00:43:54.700
Frans: and then accumulate everyone sort of familiar with this syntax.

328
00:43:54.840 --> 00:43:57.849
Frans: the fold, the initial value, and the closure.

329
00:44:01.400 --> 00:44:04.499
Frans: What I do find very.

330
00:44:05.570 --> 00:44:14.409
Frans: I don't want to use the word annoying, but in a way it is is this sort of dereferencing

331
00:44:14.820 --> 00:44:17.629
Frans: and and and cloning

332
00:44:19.590 --> 00:44:37.580
Frans: And again. I know that's that's the that's the rust. Safety guarantees that things, you know will disappear when they need to disappear or go out of scope. But still writing the code, for it can sometimes

333
00:44:37.710 --> 00:44:42.070
Frans: get very, very challenging, I find.

334
00:44:43.925 --> 00:44:44.600
Frans: Anyway.

335
00:44:44.600 --> 00:44:45.260
Gabor Szabo: Awesome.

336
00:44:45.582 --> 00:44:46.870
Frans: Sorry. That's my dog.

337
00:44:47.640 --> 00:44:48.150
Gabor Szabo: No problem.

338
00:44:48.470 --> 00:44:57.150
Frans: Getting excited there, anyway. So this is just setting up the new new tensor. And then so you have shape strides and the actual database.

339
00:44:57.400 --> 00:44:58.120
Frans: Yeah.

340
00:45:04.170 --> 00:45:07.480
Frans: You can also. Okay. So

341
00:45:07.780 --> 00:45:11.829
Frans: just now, to sort of add on to the discussion about

342
00:45:12.000 --> 00:45:23.659
Frans: find writing something that would start to be slightly, vaguely, vaguely familiar to python users. So there's a tensor library in python called Tensory.

343
00:45:24.965 --> 00:45:29.320
Frans: I don't know if anybody has looked at. I mean go you you

344
00:45:29.320 --> 00:45:29.760
Gabor Szabo: Good.

345
00:45:29.760 --> 00:45:31.930
Frans: Do some python as well. Yeah.

346
00:45:32.690 --> 00:45:40.680
Frans: so tensely is quite an advanced library in terms of using tenses for numeric computing.

347
00:45:40.940 --> 00:45:58.699
Frans: So it's not not like tensorflow, where you specifically, maybe, you know, writing a neural network, and you're training it, etc. That kind of thing. It's more you can do what they call tensor contraction, which we'll get to in a moment.

348
00:45:58.750 --> 00:46:28.699
Frans: and you can do factorized tensor into more as the sum of more usable tensors or even usable matrices. You can sort of change it and and make it. They're all sort of numerical numerical tricks to to make it easier to work with tensors and to manipulate them, and to understand what you're actually trying to do. So

349
00:46:28.800 --> 00:46:35.730
Frans: having said that, I've never really worked with it directly myself.

350
00:46:40.690 --> 00:46:44.200
Frans: Okay, so we get to subtensor.

351
00:46:44.958 --> 00:46:47.650
Frans: Right? So one of the

352
00:46:51.680 --> 00:46:56.339
Frans: One of the big things that I discovered right in the beginning

353
00:46:56.580 --> 00:47:06.300
Frans: is that because you are in matrices. It's very easy. You are working with 2 dimensions, and there's only 2 dimensions, and that's always 2 dimensions. No problem.

354
00:47:07.120 --> 00:47:10.840
Frans: If you are writing a generic tensor that can be anything.

355
00:47:11.680 --> 00:47:19.669
Frans: Then it almost forces you to start using at nested functions within to do things.

356
00:47:19.880 --> 00:47:23.070
Frans: So this also was like a very

357
00:47:23.190 --> 00:47:28.600
Frans: tricky concept to get used to, because

358
00:47:29.150 --> 00:47:36.559
Frans: you could almost think as a tensor where you are. If you want to get to an element you have to.

359
00:47:36.710 --> 00:47:43.979
Frans: How can I put it visually? You can. You must drill down the dimensions until you get to the actual element.

360
00:47:44.130 --> 00:47:54.729
Frans: So that's sort of very strange may sound very strange. But if you write the code, one starts to understand that's sort of what is happening.

361
00:47:54.820 --> 00:48:14.720
Frans: So, for example, here, if you want to extract, get a subtensor of a tensor. You 1st have to. You have to work on all the dimension levels, and at each one you've got to drill down to the required dimension and extract an element

362
00:48:15.180 --> 00:48:21.509
Frans: that sounds very strange now that I say it out loud. But that's really the thought process that's gone on here

363
00:48:21.880 --> 00:48:27.420
Gabor Szabo: Yeah, after 3 dimensions. It's it's getting a bit hairy.

364
00:48:27.600 --> 00:48:33.549
Frans: Yeah. Well, 3 dimensions you could probably still visualize. Yes, with you know. XYZ

365
00:48:33.700 --> 00:48:39.060
Gabor Szabo: Or ZI don't know what you guys call Z in South Africa. We call it Z.

366
00:48:41.500 --> 00:49:01.749
Frans: Zeros standard stuff, you know to this is, if you just want to. So this is all. What happened is then I started looking at tensely. And I say, what did what does it have? And I want to do the same stuff. So Zeros is just to create

367
00:49:01.910 --> 00:49:24.670
Frans: a tensor of this, these dimensions or shape, if you like, so that's probably shape would be a better thing to put in there, and they all just filled with zeros. These are all filled with ones. These are filled with a certain value. T, and this is filled with a certain value range that starts at T

368
00:49:24.840 --> 00:49:25.930
Frans: and go up.

369
00:49:28.240 --> 00:49:29.130
Frans: Yeah.

370
00:49:29.130 --> 00:49:34.309
Gabor Szabo: Yeah, I think dimensions would would say that like it has 3 dimensions, or 4, or whatever right

371
00:49:34.310 --> 00:49:36.919
Frans: Yeah. Yeah. But it's just it's

372
00:49:36.920 --> 00:49:39.309
Gabor Szabo: Described, actually

373
00:49:39.430 --> 00:49:45.379
Frans: It's it's just that in in python and and tensory and and even, you know, numpy, they use shape.

374
00:49:45.560 --> 00:50:09.049
Frans: So I sort of you know there's if if I wanted to make a product that I sort of can really, commercially market or sell or punt, you know, not even commercially, but in seriousness. I would have to go through this and make it all and standardize it, and and and make it much more. How can I say rigorous?

375
00:50:09.510 --> 00:50:16.890
Frans: That's the one thing. The other thing is, I'm sure that a lot of optimization can be done which I'm

376
00:50:17.070 --> 00:50:30.078
Frans: if in a rust perspective, not I haven't gone down that road at all, and which probably needs to be gone down. So I'm not making any. I mean, I've done no benchmarking against anything else. So

377
00:50:30.730 --> 00:50:37.939
Frans: you know, there's probably there's a lot of work that needs to be done for this to be a robust, rigorous library, for sure.

378
00:50:39.420 --> 00:50:44.680
Gabor Szabo: I mean, we are getting close to the, to the, to the 1 h due to

379
00:50:45.290 --> 00:50:50.820
Frans: Do I am. Oh, I'm rambling on endlessly. My apologies, anyway, so

380
00:50:50.820 --> 00:50:52.819
Gabor Szabo: Yeah, I have to wrap up soon, I guess.

381
00:50:52.820 --> 00:51:04.640
Frans: Yeah, so so random and stuff like that. And I think the biggest and the the most difficult thing that I've done up to date was contracting a tensor.

382
00:51:04.960 --> 00:51:05.930
Frans: and

383
00:51:06.120 --> 00:51:25.789
Frans: that so using Einstein's summation notation, I don't know if anybody's familiar with that. So you have basically 2 tenses. And you basically want to contract a tensor ijkl with Mkni, which means you're taking the 3rd dimension

384
00:51:25.980 --> 00:51:28.730
Frans: and contracting it with the second dimension.

385
00:51:29.170 --> 00:51:49.360
Frans: And you're taking the 1st dimension and of the left hand tensor. And you're contracting with the right hand. And what you get basically contraction, you are shrinking the dimensions you are Melding. If I can use that word, adding these 2

386
00:51:49.760 --> 00:51:56.929
Frans: tensors together, and you will end up with a tensor of dimensions. J.

387
00:51:57.300 --> 00:51:59.120
Frans: LMN.

388
00:51:59.550 --> 00:52:12.779
Frans: Right so, and contraction was extremely difficult until it the penny drops. So yes, okay. So, Einstein, notation is just a way of writing it out understandably that that

389
00:52:12.930 --> 00:52:16.550
Frans: tensor algebraus use.

390
00:52:16.820 --> 00:52:22.980
Frans: But the actual underlying routine is the contraction routine, and that

391
00:52:24.120 --> 00:52:33.350
Frans: once you realize that a tensor you have to drill down through all the dimensions, and at the end you end up with a matrix

392
00:52:34.350 --> 00:52:45.949
Frans: at the bottom of drilling down, and once you have that sort of idea in your head, then it's just a question of rearranging

393
00:52:48.410 --> 00:52:51.100
Frans: finding the the correct

394
00:52:51.430 --> 00:52:58.419
Frans: going through the dimensional drill down to find the correct element in the matrix underneath.

395
00:52:58.870 --> 00:53:14.579
Frans: That doesn't sound very nice. But anyway, that's what it happens, anyway. So so that's basically is is sort of where I'm at. As I said, contraction was the biggest thing. Oh,

396
00:53:15.260 --> 00:53:21.029
Frans: Almost the 1st thing I did with tensors is I wanted to do a creation. Macro

397
00:53:22.050 --> 00:53:26.990
Frans: and I had to resort to.

398
00:53:27.660 --> 00:53:40.330
Frans: This was very interesting. I had to resort to writing a separate crate that contains a what's this tensor? What's this? Macro called again.

399
00:53:41.170 --> 00:53:51.880
Frans: Somebody help me here. It's it's not, you know your normal macros, or you can write it in your in your crate. But this is a specific execution, Macro. Process, Macro, sorry.

400
00:53:52.030 --> 00:53:52.700
Frans: Yeah.

401
00:53:53.470 --> 00:53:59.840
Frans: And that's the 1st time that I had to use a nested function.

402
00:54:00.180 --> 00:54:06.249
Frans: Think, here, here is it here

403
00:54:07.460 --> 00:54:11.730
Frans: flatten array? Yes, I used flatten array.

404
00:54:25.880 --> 00:54:37.260
Frans: It's the 1st time I had to use it. So what I was trying to achieve here, and this is so. If we go back to the tensor library, which uses that macro in that crate

405
00:54:37.550 --> 00:54:41.440
Frans: is that you can establish tenses by doing this.

406
00:54:41.790 --> 00:54:43.620
Frans: And that is fantastic.

407
00:54:44.000 --> 00:54:53.240
Frans: I love that because and your community, I mean, this is a like a 12 dimensional tensor.

408
00:54:53.380 --> 00:54:57.829
Frans: you know, which you can establish like. So this is just a stupid example.

409
00:54:57.990 --> 00:55:22.109
Frans: But you can see, and that establishes the shape of that and the strides of that. And that's the actual day test of the flat. Vector anyway, that's about it. So I would love to. You know, if you guys at any point want to chat more about how I did it, I'm happy to have future interaction

410
00:55:22.610 --> 00:55:30.729
Frans: on a 1 to one or one to in a group session, but only from April onwards, because at the moment things are hectic on my side.

411
00:55:31.150 --> 00:55:33.399
Frans: that's it. That's a wrap for me.

412
00:55:33.400 --> 00:55:38.660
Gabor Szabo: Well, thank you. Thank you. April. Is not that far? It's it's like 2 weeks, right? So

413
00:55:38.930 --> 00:55:51.300
Frans: Yes, no, exactly. But that's why it's a bit tense at the moment, because there's lots of pressure, and and there's a there's a substantial bonus involved. If I sort things out so you can understand

414
00:55:52.591 --> 00:56:12.020
Gabor Szabo: Good. So so thank you very much for your for your presentation. By the way, your your information. I I put the link to the to the git repository in the chat, and that if you'll be under the video as well, you will find that and the link to your. I think your Linkedin page. So people will be able to contact you if if interested.

415
00:56:12.110 --> 00:56:25.790
Gabor Szabo: So thank you very much for your presentation and for everyone who who listened, and for everyone who watched the video, and just remember to like the video and follow the channel. Thank you very much. Bye, bye.

416
00:56:25.790 --> 00:56:28.479
Frans: Thanks. Gabor, thanks, guys. Cheers. Bye-bye.

