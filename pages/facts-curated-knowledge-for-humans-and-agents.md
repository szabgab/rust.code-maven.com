---
title: "Facts: Curated Knowledge for Humans and Agents"
timestamp: 2026-09-08T09:30:01
tags:
published: true
author:
archive: true
---

## Description

[Facts](https://alnewkirk.com/projects/facts/) is like Git for knowledge. It's an open-source protocol, SDK, and CLI for managing shared knowledge between humans and AI agents. Instead of treating everything written or remembered as true, Facts lets participants propose, review, revise, accept, and reject claims, creating a trusted, versioned record of what a team actually knows.

## Bio

[Al Newkirk](https://www.linkedin.com/in/alnewkirk/) is a technology leader with more than 30 years in software engineering. He builds systems and frameworks for improving how people and AI agents work, make decisions, and share knowledge.


{% youtube id="eL5HQLdmht8" file="2026-09-06-facts-curated-knowledge-for-humans-and-agents-with-al-newkirk.mp4" %}


## Transcript

1
00:00:01.970 --> 00:00:15.599
Gabor Szabo: So, hello, and welcome to the Code Maven YouTube channel, and to this presentation by Al Newkirk. I'm really happy that you… I hope that you pronounced your name correctly, but anyway, I'm really happy that you… that… that we…

2
00:00:15.650 --> 00:00:24.389
Gabor Szabo: I've seen each other the first time, I think, even though we have been… we had some conversations for, I don't know, 10, 20 years? I don't remember.

3
00:00:24.390 --> 00:00:25.120
Al Newkirk: Yeah.

4
00:00:25.120 --> 00:00:31.339
Gabor Szabo: My name is Gabor Szabo, I'm organizing these, these sessions, these online sessions.

5
00:00:31.340 --> 00:00:50.790
Gabor Szabo: We have some guests who might ask some questions, so it might become a conversation. That's the perks for those people who actually joined the event, that they can ask questions, and we had a nice conversation before the video, and we might have another one after then, if time permits.

6
00:00:52.060 --> 00:01:07.459
Gabor Szabo: And if you're watching the video, then please like it, and follow the channel, and below the video, you will find links to the links that all provides, and to the future events. And I think that's it for now, and

7
00:01:07.680 --> 00:01:12.579
Gabor Szabo: The stage is yours, yeah? Okay. Go ahead.

8
00:01:12.580 --> 00:01:13.310
Al Newkirk: Hello, everyone.

9
00:01:13.310 --> 00:01:14.440
Gabor Szabo: Thanks so much for coming on.

10
00:01:14.810 --> 00:01:31.389
Al Newkirk: No problem at all. Hello, everyone. My name is Al Newkirk, and, been in the tech industry for about 30 years, since the mid-90s. My, I guess my first language was Perl. I love Perl. I still write a ton of Perl. But,

11
00:01:31.600 --> 00:01:40.870
Al Newkirk: I've also spent most of my career in software as a service, and web, technologies, and,

12
00:01:42.050 --> 00:01:53.180
Al Newkirk: After, sort of, spending half my career as an engineer, I switched into leadership, And consulting, managing projects.

13
00:01:53.400 --> 00:01:55.019
Al Newkirk: And that is…

14
00:01:56.290 --> 00:02:02.950
Al Newkirk: a great, like, segue into this particular project. So, I'm now working on a project that I call

15
00:02:03.560 --> 00:02:08.590
Al Newkirk: A, C, T, S, And,

16
00:02:08.699 --> 00:02:11.430
Al Newkirk: And the plural, so the…

17
00:02:11.850 --> 00:02:18.239
Al Newkirk: I want to have a more, like, a kind of a conversation about what led me to the…

18
00:02:18.970 --> 00:02:22.640
Al Newkirk: project that I'm working on, and why I think it's important.

19
00:02:22.860 --> 00:02:29.250
Al Newkirk: And then I'll sort of do, like, a demo of the current implementation of the project, and…

20
00:02:29.540 --> 00:02:34.210
Al Newkirk: you know, happy to answer any questions, but I want to first start

21
00:02:34.350 --> 00:02:41.970
Al Newkirk: talking about how I got here, and I'll be brief, and feel free to ask questions or interrupt.

22
00:02:42.350 --> 00:02:44.279
Al Newkirk: If, if you need to.

23
00:02:44.550 --> 00:02:59.570
Al Newkirk: So… I, I launched a software-as-a-service product called AllSign. It's, All Sign, A-L-L… DN dot F.

24
00:02:59.870 --> 00:03:07.650
Al Newkirk: So, Balsign is a product that lets you Elaborate and negotiate on agreements.

25
00:03:07.890 --> 00:03:17.020
Al Newkirk: I am a big fan of the peer review process, so think RFDs, OKRs, PRDs, that kind of thing.

26
00:03:17.240 --> 00:03:23.019
Al Newkirk: ARDs, TDDs, so, you know, documents that,

27
00:03:23.370 --> 00:03:34.640
Al Newkirk: multiple people collaborate on to capture decisions, and to deliberate, and to sign off on. I'm a big fan of the peer review process. I think that

28
00:03:34.800 --> 00:03:41.630
Al Newkirk: I mean… That is… the future.

29
00:03:41.840 --> 00:03:45.239
Al Newkirk: And, you can see a version of that

30
00:03:45.460 --> 00:03:52.929
Al Newkirk: in how people use AI, a lot of people talk about spec-driven development, creating…

31
00:03:53.980 --> 00:04:05.820
Al Newkirk: engineering specifications to give to the AI agent. So these are all forms of… During context and decisions, And…

32
00:04:06.570 --> 00:04:13.440
Al Newkirk: And a project with multiple people, that's gonna be, like, collaborating on those decisions, so the whole peer review process.

33
00:04:13.560 --> 00:04:19.600
Al Newkirk: So anyway, that is all signed, and I'll share my screen just quickly, so that you can…

34
00:04:19.709 --> 00:04:22.679
Al Newkirk: I see all sign, I'm not gonna do a demo,

35
00:04:22.930 --> 00:04:25.929
Al Newkirk: Of all sign, because this conversation is not about all sign.

36
00:04:26.460 --> 00:04:31.149
Al Newkirk: But this is, AllSign, and this is what the UI looks like.

37
00:04:31.410 --> 00:04:38.500
Al Newkirk: As you can maybe infer from the UI, you can create agreements that represent

38
00:04:39.060 --> 00:04:53.730
Al Newkirk: decisions or projects. You can invite people to negotiate those decisions. These agreements can be legally binding or non-binding. So you can sign off on the agreements and

39
00:04:54.540 --> 00:05:04.260
Al Newkirk: Digitally sign, digitally sign those agreements, and, and they are, like, They have the appropriate…

40
00:05:04.500 --> 00:05:12.409
Al Newkirk: distribution, so they can be used as legal documents if you decided to use it that way.

41
00:05:12.530 --> 00:05:23.340
Al Newkirk: But, I just want to kind of show you some of the screenshots, just to familiarize you with the ideas that led to the project that I want to talk to you about today.

42
00:05:24.180 --> 00:05:31.899
Al Newkirk: So fundamentally, like, what we're talking about is negotiation. So you have multiple people, and…

43
00:05:32.200 --> 00:05:37.800
Al Newkirk: In my experience, and in companies that I… where I've worked, where there's a healthy peer review process.

44
00:05:38.340 --> 00:05:46.790
Al Newkirk: Typically, we're using something like Google Docs, and people are commenting on, like, maybe a migration project.

45
00:05:46.930 --> 00:05:52.909
Al Newkirk: And… maybe there's, like, a table in the Google Doc where people

46
00:05:53.970 --> 00:06:01.590
Al Newkirk: sign their name, or add pills or labels that represent their sign-off. And that's how you decide when

47
00:06:03.410 --> 00:06:07.340
Al Newkirk: The idea has been sort of settled or signed off on.

48
00:06:07.460 --> 00:06:13.400
Al Newkirk: But because it's Google Docs, and it's not designed for that, workflow.

49
00:06:13.590 --> 00:06:23.880
Al Newkirk: there's no real way to lock an agreement, make it immutable. There's… like, comments commonly get suppressed, you know, if you comment on a particular part of an agreement.

50
00:06:24.220 --> 00:06:29.489
Al Newkirk: And… and someone changes it, your comments get suppressed, etc.

51
00:06:29.600 --> 00:06:37.349
Al Newkirk: So, so that's what… that's how people are using Google Docs and other… and other technologies to

52
00:06:37.660 --> 00:06:41.960
Al Newkirk: facilitate this peer review process. I basically just built a SaaS product to do it.

53
00:06:42.460 --> 00:06:49.410
Al Newkirk: Now, this product actually, even though, the…

54
00:06:50.220 --> 00:06:54.079
Al Newkirk: the documentation for the API and the MCP server are…

55
00:06:54.210 --> 00:07:13.040
Al Newkirk: you have to find them, but it does have an API, it does have an MCP server, you can have an AI agent negotiate agreements for you. I've done it, it's actually quite fun to have two AIs, like, negotiating agreements and disagreeing with each other. So you can do that with this product, but

56
00:07:13.310 --> 00:07:18.170
Al Newkirk: But again, like, this conversation is not about… All sign.

57
00:07:18.570 --> 00:07:24.790
Al Newkirk: what I… what I found when I was developing this, product is that, that…

58
00:07:26.390 --> 00:07:32.960
Al Newkirk: I think that the future will involve Version-controlling knowledge.

59
00:07:33.090 --> 00:07:36.190
Al Newkirk: And… capturing decisions.

60
00:07:36.350 --> 00:07:42.809
Al Newkirk: What a lot of people who are, who are really leveraging AI at a high level.

61
00:07:42.970 --> 00:07:49.549
Al Newkirk: are stumbling upon is the idea that the implementation…

62
00:07:50.400 --> 00:08:03.920
Al Newkirk: matters secondary to the context, the spec, if you will. But it's not just the spec. It's all the context around the spec. It's the thinking, the decisions that went into

63
00:08:06.150 --> 00:08:09.309
Al Newkirk: the thing that you want to produce. And…

64
00:08:09.650 --> 00:08:16.579
Al Newkirk: And it got me to thinking, because Allsign already does this, but Allsign takes a very opinionated approach.

65
00:08:18.200 --> 00:08:26.260
Al Newkirk: Fundamentally, in all sign, you have to create an agreement. Let me maybe go to a product page to show you…

66
00:08:27.970 --> 00:08:31.119
Al Newkirk: other screenshots. So you create an agreement.

67
00:08:31.190 --> 00:08:44.079
Al Newkirk: And an agreement can have sections, and sections can have terms, and sometimes terms can live outside of sections. And so, this is an example of an agreement in its structure.

68
00:08:44.150 --> 00:08:52.700
Al Newkirk: Scope of ownership is a section, it has multiple terms, and those terms have content that can be negotiated.

69
00:08:52.910 --> 00:08:59.590
Al Newkirk: And you can see that here. So this is a particular term

70
00:08:59.670 --> 00:09:16.789
Al Newkirk: weekly migration status inside of a section, payment methods, profile ownership, and obviously you can see the controls where you can accept or reject. If you reject it, you can propose new terms, and you can do this turn-based negotiation until there is consensus.

71
00:09:16.840 --> 00:09:21.850
Al Newkirk: Alright, so I'm spending way too much time here, but here's the point.

72
00:09:24.530 --> 00:09:28.769
Al Newkirk: Every decision is not necessarily an entire agreement.

73
00:09:29.560 --> 00:09:34.460
Al Newkirk: Sometimes, a decision or a bit of knowledge is…

74
00:09:35.130 --> 00:09:41.300
Al Newkirk: just an idea. And so you shouldn't have to force knowledge into

75
00:09:41.790 --> 00:09:48.699
Al Newkirk: The shape of an agreement with sections and terms to capture it and to deliberate on it.

76
00:09:48.930 --> 00:09:55.270
Al Newkirk: So, that led me to… The question, what if the…

77
00:09:55.440 --> 00:10:08.019
Al Newkirk: fundamentals were open core? What if, what if… what if the core concept of… Versioning knowledge and negotiating

78
00:10:08.470 --> 00:10:12.790
Al Newkirk: knowledge were sort of, like, open source. What if there was a Git

79
00:10:13.000 --> 00:10:19.559
Al Newkirk: For knowledge management. So, Git is a software version control system.

80
00:10:19.750 --> 00:10:31.680
Al Newkirk: that is distributed, and I was thinking, what if there was something like that, but for knowledge management? So, that's what led me to what I call the back system. Now,

81
00:10:32.390 --> 00:10:35.519
Al Newkirk: the way… because I decided from the very start.

82
00:10:35.830 --> 00:10:37.910
Al Newkirk: That this should be open source.

83
00:10:39.230 --> 00:10:42.720
Al Newkirk: I split it into the protocol.

84
00:10:42.890 --> 00:10:48.880
Al Newkirk: the SDK and the CLI, so the protocol is, the…

85
00:10:48.980 --> 00:10:56.250
Al Newkirk: reasoning. It is… it is exactly the thing that I talked about earlier when I talked about

86
00:10:58.450 --> 00:11:11.929
Al Newkirk: capturing the thinking and the implementation is secondary. It matters secondarily to the core ideas. So, the core ideas of facts is captured in the protocol. That exists, it's on GitHub.

87
00:11:12.030 --> 00:11:20.089
Al Newkirk: And… The FACT CLI, is a Rust implementation

88
00:11:20.710 --> 00:11:26.829
Al Newkirk: of the fax protocol as an SDK and a CLI that uses that SDK.

89
00:11:27.280 --> 00:11:28.130
Al Newkirk: So…

90
00:11:28.310 --> 00:11:39.690
Al Newkirk: the reason why I went with the name, Fast is because, and I know a little bit about philosophy, but I'm in no way an expert. But in philosophy,

91
00:11:40.060 --> 00:11:45.729
Al Newkirk: You know, a fact is a true proposition, and a proposition is…

92
00:11:46.470 --> 00:11:50.930
Al Newkirk: Statement or a claim that can be evaluated as true or false.

93
00:11:51.060 --> 00:11:55.360
Al Newkirk: And it felt like it just fit Well,

94
00:11:55.490 --> 00:12:04.660
Al Newkirk: So, what we're talking about is… when we're talking about knowledge, you know, we want to…

95
00:12:04.940 --> 00:12:10.940
Al Newkirk: differentiate between, like, a claim and a truth, so…

96
00:12:12.430 --> 00:12:19.360
Al Newkirk: So, for example, like, if you are using, the fax system.

97
00:12:19.980 --> 00:12:23.750
Al Newkirk: And you capture an idea.

98
00:12:23.860 --> 00:12:32.560
Al Newkirk: That idea is not necessarily a fact, and it shouldn't be made available to others that are also participating in this system.

99
00:12:32.930 --> 00:12:45.850
Al Newkirk: maybe even agents. It shouldn't be made available immediately to agents until it becomes a fact. But on the fact system, the way something becomes a fact is, and let me switch to, sort of, like, a,

100
00:12:46.940 --> 00:12:53.120
Al Newkirk: I guess I'll start the demo, because I can illustrate these concepts with the CLI.

101
00:12:53.490 --> 00:12:58.279
Al Newkirk: And… and drive the point home, so let me just sort of share one of my terminals.

102
00:13:00.400 --> 00:13:07.659
Al Newkirk: And, I'm gonna, like, increase the text, and let me know if there's something else I need to do to make this

103
00:13:09.290 --> 00:13:17.300
Al Newkirk: clear. So we have the FACT CLI, and, in a lot of ways, it's meant to

104
00:13:17.630 --> 00:13:34.069
Al Newkirk: be analogous to the Git CLI and the Git version control system. So, in the same way that Git manages repositories, the FAPS system manages ledgers. Ledgers are,

105
00:13:34.790 --> 00:13:45.839
Al Newkirk: repositories of knowledge. They are stores of knowledge. So for example, I'll create a ledger here. So the first thing I'm going to do is I'm gonna initialize this directory.

106
00:13:46.600 --> 00:13:58.219
Al Newkirk: if I… if I didn't do this, then the CLI would use the system-wide Configuration for organizing ledgers.

107
00:13:58.620 --> 00:14:04.540
Al Newkirk: But I want to localize the ledger to this particular directory, and so I'm gonna initialize…

108
00:14:05.110 --> 00:14:07.869
Al Newkirk: The space, and it's gonna create a default ledger.

109
00:14:08.350 --> 00:14:10.060
Al Newkirk: And, if I do…

110
00:14:10.400 --> 00:14:17.609
Al Newkirk: act as, it'll show that I am currently the ledger admin, I can create additional actors.

111
00:14:17.900 --> 00:14:25.300
Al Newkirk: and create an actor that is, Claude or Codex, or I could just have Claude or Codex.

112
00:14:29.090 --> 00:14:34.059
Al Newkirk: That's me. So… and I'm gonna… I'm gonna demonstrate all of that.

113
00:14:34.530 --> 00:14:35.490
Al Newkirk: So…

114
00:14:37.080 --> 00:14:45.120
Al Newkirk: If I do a fact list, obviously it's an empty ledger, so there's no propositions here. And

115
00:14:45.540 --> 00:14:52.780
Al Newkirk: Fact pending is where you would go to see propositions that you've created that have not yet become

116
00:14:53.900 --> 00:15:01.579
Al Newkirk: facts. And if, it's not obvious, Other actors in the system

117
00:15:03.070 --> 00:15:15.389
Al Newkirk: they don't see propositions that haven't become factor. And there's also a protocol for, like, deliberating on facts before there is a

118
00:15:15.560 --> 00:15:19.390
Al Newkirk: To create a consensus. And once a consensus is reached.

119
00:15:20.100 --> 00:15:37.839
Al Newkirk: that fact automatically gets ex… sorry, that proposition automatically gets accepted as a fact. But I'm just gonna, like, do a couple of commands to create… so FactPropose launches the editor. There's other ways to do this, but it launches the editor, and you can start describing a proposition. So you could say…

120
00:15:38.220 --> 00:15:38.920
Al Newkirk: Hmm.

121
00:15:42.950 --> 00:15:48.249
Al Newkirk: Guess I'll just use voice. Brush programming language… And… I'm fine.

122
00:15:49.600 --> 00:15:50.490
Al Newkirk: Alright, let's…

123
00:15:52.540 --> 00:16:00.449
Al Newkirk: And I'm just gonna create that. So that created a, position. If I do, like, fact the list.

124
00:16:01.280 --> 00:16:06.840
Al Newkirk: you won't see the proposition, because it hasn't been accepted yet. If I go Fact Pending…

125
00:16:07.080 --> 00:16:11.519
Al Newkirk: you'll see the proposition, and I can accept it, good.

126
00:16:12.390 --> 00:16:14.630
Al Newkirk: I could accept it by ID,

127
00:16:18.510 --> 00:16:22.579
Al Newkirk: And as you can see, it says accept a proposition, effective revision.

128
00:16:22.950 --> 00:16:31.260
Al Newkirk: And, the current summary. So, it's keeping… it's versioning, like, your knowledge as you're creating these.

129
00:16:31.730 --> 00:16:38.359
Al Newkirk: I could also, and if I do fact list, now that proposition that was created is available as a

130
00:16:38.930 --> 00:16:42.490
Al Newkirk: as a fact to whoever's using the system.

131
00:16:42.780 --> 00:16:46.860
Al Newkirk: So let me, show you what this looks like with,

132
00:16:47.450 --> 00:16:50.439
Al Newkirk: and AI. And… and this is,

133
00:16:51.610 --> 00:16:59.539
Al Newkirk: This is where you really get to see, like, the protocol and the… approach, like, unlocked. So…

134
00:16:59.990 --> 00:17:07.959
Al Newkirk: To make it easy, let me show… Another screen.

135
00:17:17.030 --> 00:17:22.460
Al Newkirk: I'm just sort of, pulling up the, gitHub.

136
00:17:22.869 --> 00:17:25.350
Al Newkirk: Share my screen so you can see that as well.

137
00:17:26.589 --> 00:17:39.990
Al Newkirk: Okay, so this is the GitHub organization. There's several projects in this organization. There's the CLI, the SDK, there's the, particular architecture that the,

138
00:17:40.740 --> 00:17:47.830
Al Newkirk: that the Rust implementation uses. And then there's the spec. This is the actual protocol that,

139
00:17:49.020 --> 00:17:55.200
Al Newkirk: Is the same no matter what implementation, it's, it's implemented with.

140
00:17:55.400 --> 00:18:02.420
Al Newkirk: But skills is also, this is how you use the fact system with the,

141
00:18:03.040 --> 00:18:07.700
Al Newkirk: with your agents. So, I'm gonna open this up, but I also wanna…

142
00:18:07.930 --> 00:18:16.039
Al Newkirk: In order to use it, you need to have the CLI, you can go to releases, and you can download, like, the latest,

143
00:18:16.410 --> 00:18:20.029
Al Newkirk: you can download the latest CLI for your

144
00:18:20.440 --> 00:18:28.450
Al Newkirk: operating system. It's, you know, between 5 to, 8 megabytes, at, at present.

145
00:18:28.640 --> 00:18:31.640
Al Newkirk: So, if you go to skills,

146
00:18:31.860 --> 00:18:35.729
Al Newkirk: It's really easy to install this, I'm gonna show you now,

147
00:18:35.890 --> 00:18:38.790
Al Newkirk: I'm gonna clone this here,

148
00:18:43.530 --> 00:18:46.649
Al Newkirk: And there is an install,

149
00:18:50.110 --> 00:18:56.700
Al Newkirk: Okay, sorry. I usually call it the CLI, I usually call it install, but it's called spills here.

150
00:18:56.830 --> 00:19:01.399
Al Newkirk: So, if you… If you execute the skills, CLI,

151
00:19:02.220 --> 00:19:10.459
Al Newkirk: you know, it's just a Bash CLI, and it has a help menu, and you can, depending on, like, the agent that you have, you can,

152
00:19:10.740 --> 00:19:13.010
Al Newkirk: You can link, you know, Claude.

153
00:19:13.170 --> 00:19:16.430
Al Newkirk: And it skipped it because it's already linked.

154
00:19:16.620 --> 00:19:19.999
Al Newkirk: Because I'm already using it, you could, if you want to do it for codecs.

155
00:19:20.140 --> 00:19:29.349
Al Newkirk: If you're using an agent harness that… supports the… dot agents,

156
00:19:30.030 --> 00:19:35.680
Al Newkirk: directory, then you could just say, like, I think agents, yeah, so…

157
00:19:35.840 --> 00:19:42.769
Al Newkirk: So anyway, that's… that's all you need to do to install it, and it… and now it's linked up, and it just works.

158
00:19:43.170 --> 00:19:47.680
Al Newkirk: Just to… just to verify, like, if you… if you look at, like, let's say…

159
00:19:47.860 --> 00:19:51.219
Al Newkirk: Codex skills, you can see that

160
00:19:51.960 --> 00:19:54.700
Al Newkirk: it basically creates a SIM link to…

161
00:19:55.320 --> 00:20:06.760
Al Newkirk: wherever you ran the install script from. I ran it from a different directory, so it's already sem-linked from a different directory, so I don't need to worry about that. So…

162
00:20:07.120 --> 00:20:12.350
Al Newkirk: Let me, I'm gonna delete that, because it's already sort of configured now.

163
00:20:12.640 --> 00:20:18.360
Al Newkirk: And I'm gonna start… Codex, and then I'll start Claude later.

164
00:20:18.620 --> 00:20:21.700
Al Newkirk: Okay.

165
00:20:23.760 --> 00:20:36.170
Al Newkirk: So… Now, it's… pre-configure, to… Check the FATS system.

166
00:20:37.390 --> 00:20:42.489
Al Newkirk: for memories. So you could… you could do something… oh, I'm not… actually, I'm sorry, I'm not…

167
00:20:42.850 --> 00:20:47.140
Al Newkirk: I thought I was sharing the screen, but I'm not. Let me just share the screen.

168
00:20:51.820 --> 00:20:52.630
Al Newkirk: Hmm.

169
00:20:58.050 --> 00:20:59.689
Al Newkirk: I find the terminal, really.

170
00:21:01.730 --> 00:21:05.189
Al Newkirk: I think this is it. Let me know if,

171
00:21:13.010 --> 00:21:18.059
Al Newkirk: I think I'm… I think I'm sharing it, but let me know if you're having trouble seeing it. But anyway,

172
00:21:20.880 --> 00:21:21.980
Al Newkirk: I just saw the…

173
00:21:22.550 --> 00:21:30.990
Al Newkirk: just all the comments here. So, okay. Alright, so I started, Codex, and now, because,

174
00:21:31.830 --> 00:21:34.740
Al Newkirk: Because the skills are configured to…

175
00:21:34.740 --> 00:21:35.570
Gabor Szabo: Fingers crossed.

176
00:21:36.450 --> 00:21:37.490
Al Newkirk: Good, good.

177
00:21:37.870 --> 00:21:45.300
Al Newkirk: Okay, yeah, because the, because the AI agent is configured to…

178
00:21:45.640 --> 00:21:51.210
Al Newkirk: reference the fact system for memories, I can… I can say something like,

179
00:21:53.380 --> 00:21:56.589
Al Newkirk: What do you recall about the Rust programming language?

180
00:21:58.550 --> 00:22:04.219
Al Newkirk: And… It should check the FATS system, and

181
00:22:07.210 --> 00:22:14.649
Al Newkirk: Okay, well, this did a more, like, generic, this did sort of a more generic…

182
00:22:16.610 --> 00:22:20.199
Al Newkirk: recollection, but I could be more specific and say, okay.

183
00:22:20.580 --> 00:22:26.900
Uri Bruck: from the facts you entered into the system, or from what the agent knows from somewhere else, from its own?

184
00:22:27.690 --> 00:22:36.500
Al Newkirk: Yeah, this came from its own knowledge, what it knows from somewhere else, so I'm going to… I'm going to specifically nudge it to use the facts system, so I'm gonna say, using facts.

185
00:22:37.400 --> 00:22:39.880
Al Newkirk: What do you recall about…

186
00:22:41.010 --> 00:22:51.010
Al Newkirk: the, Rush programming learning. So now it's gonna… with that nudge, it should… typically, you don't need that nudge, but with that nudge, it'll… it'll…

187
00:22:51.590 --> 00:23:05.299
Al Newkirk: As you can see, it's loading the different skills related to the fact system, and now it's going to search for Russ, and it's going to, it found the proposition, there's only one proposition, because the ledger is empty.

188
00:23:05.740 --> 00:23:07.080
Al Newkirk: And,

189
00:23:07.640 --> 00:23:17.490
Al Newkirk: And now it's going to, there's nothing there. So, using FAFTS, I found one accepted proposition, whatever. So, here's what I want to do. I want to say.

190
00:23:19.090 --> 00:23:23.729
Al Newkirk: the generic… explanation you gave of Rust.

191
00:23:24.290 --> 00:23:26.680
Al Newkirk: Distill that into a set of facts.

192
00:23:32.650 --> 00:23:41.819
Al Newkirk: So, actually, let me, I got the language wrong, but, so… so this should…

193
00:23:41.980 --> 00:23:46.829
Al Newkirk: break down what… What it initially replied with.

194
00:23:46.960 --> 00:23:50.790
Al Newkirk: And break that into a set of atomized fat.

195
00:23:51.100 --> 00:23:56.779
Al Newkirk: And commit them, to the, the fax memory system.

196
00:23:57.420 --> 00:24:03.200
Al Newkirk: And then I'm gonna show you… So, distilled fats,

197
00:24:04.680 --> 00:24:08.340
Al Newkirk: Were these committed to the default ledger?

198
00:24:12.530 --> 00:24:19.430
Al Newkirk: And then I'm going to check the ledger by saying list… sorry, I'm gonna say, like, list…

199
00:24:21.610 --> 00:24:26.990
Al Newkirk: ending… though there… so it was not. No, I only distilled them…

200
00:24:27.540 --> 00:24:30.939
Al Newkirk: did not run FactorFlow or whatever, so, okay.

201
00:24:32.720 --> 00:24:35.620
Al Newkirk: Commit these facts to the ledger.

202
00:24:40.400 --> 00:24:49.289
Al Newkirk: Now, it's gonna commit… it's gonna… so, the distillation process is actually a really interesting process, because

203
00:24:49.480 --> 00:24:57.029
Al Newkirk: It does what… It does a crude version of what is considered knowledge compression.

204
00:24:57.280 --> 00:25:02.690
Al Newkirk: And this is something that I think only people in the memory space pay attention to, but it's…

205
00:25:02.810 --> 00:25:09.300
Al Newkirk: How do you take, contents, and distill it down to just the fact.

206
00:25:09.870 --> 00:25:11.180
Al Newkirk: such that…

207
00:25:12.660 --> 00:25:20.799
Al Newkirk: A completely separate process could reconstruct the entire idea from those facts, so that where you haven't lost

208
00:25:20.930 --> 00:25:31.290
Al Newkirk: any… any… any requisite information. But you don't, you know, maybe you don't need all of the filler content, so you're distilling

209
00:25:31.400 --> 00:25:37.950
Al Newkirk: context down to, like, precise atoms of information that…

210
00:25:38.130 --> 00:25:51.860
Al Newkirk: are necessary to reproduce, like, the big idea in another process. So, anyway, that's… that's… that's, distillation slash compression, and

211
00:25:52.120 --> 00:25:58.840
Al Newkirk: That's what's happened here, so… I committed to the… to the ledger, so if I do pending.

212
00:25:58.950 --> 00:26:05.440
Al Newkirk: I think it auto-accepted the facts, which I like, that's good. So if I do fact list…

213
00:26:07.460 --> 00:26:09.780
Al Newkirk: Sorry, give me a second here.

214
00:26:11.180 --> 00:26:13.810
Al Newkirk: I revised… oh, it revised the existing…

215
00:26:15.910 --> 00:26:18.169
Al Newkirk: They revised the existing, fact.

216
00:26:18.460 --> 00:26:23.139
Al Newkirk: Okay, so what I want to show you here is if I do fact show, this…

217
00:26:23.360 --> 00:26:28.109
Al Newkirk: As you can see, right, there are two revisions. One was superseded.

218
00:26:28.510 --> 00:26:40.420
Al Newkirk: And they both have different identifiers, so you could see… so you can see it's being version controlled. And if you say, like, facts open, and you give it the identifier, it'll… it'll launch your editor.

219
00:26:40.580 --> 00:26:50.520
Al Newkirk: with the content. So it basically, like, took the content and distilled it down into… Into this,

220
00:26:52.270 --> 00:26:53.369
Al Newkirk: It's a dispatch.

221
00:26:53.530 --> 00:27:00.150
Al Newkirk: But, so here's the interesting thing that you can do. I'm gonna… I'm gonna open up another terminal, and I'm gonna launch, Claude.

222
00:27:01.620 --> 00:27:06.090
Al Newkirk: And… plot is also configured, because the skills…

223
00:27:06.650 --> 00:27:10.009
Al Newkirk: I installed the skills across Cloud and codec, so…

224
00:27:10.810 --> 00:27:21.840
Al Newkirk: So now they're both sharing the same memory. They're both pointed at the same ledger, and they can recall the same information, and they can sort of, like, add information to the ledger. So I'm gonna say.

225
00:27:21.960 --> 00:27:23.900
Al Newkirk: Okay.

226
00:27:28.330 --> 00:27:32.109
Al Newkirk: What do you recall I've written about the Rust programming language?

227
00:27:35.750 --> 00:27:41.670
Al Newkirk: Alright, so, checking memory,

228
00:27:42.710 --> 00:27:45.699
Al Newkirk: It's saying nothing in my memory, okay. So…

229
00:27:46.020 --> 00:27:58.270
Al Newkirk: This is what's called, like, a bootstrapping problem. So, every… most AI agent harnesses have their own memory system, and it defaults to that, and you have to…

230
00:27:58.390 --> 00:28:07.630
Al Newkirk: push it out of that if you want to use another memory system. And there are different ways to do that. So, I'm gonna say, like, the same that I did before. I'm gonna say, like, using…

231
00:28:08.150 --> 00:28:12.550
Al Newkirk: that's… What do you recall about the respiratory enemies?

232
00:28:13.260 --> 00:28:16.590
Al Newkirk: So now, as you can see, it loaded the skill.

233
00:28:16.730 --> 00:28:23.589
Al Newkirk: And now it's using the fax system, and it's gonna use the same ledger, and the same,

234
00:28:24.080 --> 00:28:26.460
Al Newkirk: And all the same information is in the ledger.

235
00:28:26.790 --> 00:28:27.550
Al Newkirk: I'm sorry.

236
00:28:28.300 --> 00:28:29.220
Al Newkirk: Okay.

237
00:28:32.710 --> 00:28:41.180
Al Newkirk: Okay. So, without reading, like, the particulars, as you can see, it found the proposition.

238
00:28:42.100 --> 00:28:50.910
Al Newkirk: 202A0, and that is the… are accepted…

239
00:28:51.630 --> 00:28:53.690
Al Newkirk: So, I'm gonna jump back over to…

240
00:28:53.960 --> 00:28:56.509
Al Newkirk: Codex. I'm gonna say, like,

241
00:28:59.180 --> 00:29:05.310
Al Newkirk: Create a new proposition… I'm sorry. Actually, because you can use the fact

242
00:29:05.450 --> 00:29:19.019
Al Newkirk: language, but you don't have to. Like, you can… you can use regular common with parlance, talking about, like, memories, or committing things, and it should do the right thing. So, I want to say,

243
00:29:26.760 --> 00:29:33.410
Al Newkirk: add to memory that the Rust programming language Typically results in…

244
00:29:33.750 --> 00:29:40.000
Al Newkirk: Very large workspaces in terms of… Bites on disk.

245
00:29:42.210 --> 00:29:45.510
Al Newkirk: I mean… But I also want to add,

246
00:29:48.720 --> 00:29:50.799
Al Newkirk: Do not accept this stack.

247
00:29:51.830 --> 00:29:55.420
Al Newkirk: Or, actually, you know, I don't want to lead it too much, so I'm just gonna…

248
00:29:56.260 --> 00:30:02.119
Al Newkirk: typically in my… in my day-to-day work, where I'm working on different projects, and

249
00:30:02.460 --> 00:30:07.209
Al Newkirk: Using different, ledgers and memories,

250
00:30:07.350 --> 00:30:16.070
Al Newkirk: I will, like, tell it what I want it to auto-accept and what I want it to leave as pending. But as you can see, it is,

251
00:30:16.470 --> 00:30:25.539
Al Newkirk: It first checked that there's nothing… there's no pre-existing knowledge that it needs to revise, and now it's gonna sort of, like, add a revision, so…

252
00:30:25.670 --> 00:30:30.550
Al Newkirk: It said it created a review, the revision pending

253
00:30:30.690 --> 00:30:36.230
Al Newkirk: So if I go back over here to the terminal, and I go, like, pending…

254
00:30:36.770 --> 00:30:40.129
Al Newkirk: I guess it auto-accepted it. Oh, wait, sorry, give me a sec.

255
00:30:45.190 --> 00:30:49.970
Al Newkirk: Okay, so it added it… it sort of added it to the existing

256
00:30:50.270 --> 00:30:53.720
Al Newkirk: position, but that's actually not what I want, so I'm gonna correct it and say…

257
00:30:55.350 --> 00:31:02.630
Al Newkirk: I didn't want you to revise… the existing fact. Instead.

258
00:31:03.610 --> 00:31:07.880
Al Newkirk: Retract that and add that as a separate fact.

259
00:31:09.330 --> 00:31:15.850
Al Newkirk: Okay. And then I'm gonna come over to… flawed, I'm gonna say, like,

260
00:31:19.820 --> 00:31:21.730
Al Newkirk: Do I have any memories about…

261
00:31:26.090 --> 00:31:29.200
Al Newkirk: Rust workspace, sizes.

262
00:31:31.930 --> 00:31:34.969
Al Newkirk: So… It's still…

263
00:31:35.440 --> 00:31:45.339
Al Newkirk: Codex is still making the changes to the ledger, so it created a new revision because the… all ledgers are append-only.

264
00:31:49.090 --> 00:31:56.070
Al Newkirk: ledgers, literal ledgers, append-only data stores. So it… it sort of…

265
00:31:56.380 --> 00:32:06.279
Al Newkirk: up… it sort of created a new revision, removing that bullet point, adding it to another revision, so… so again, if I go back list, now I see two, facts.

266
00:32:06.810 --> 00:32:16.890
Al Newkirk: And, there are no, pending, pending propositions. So, if I go over to, Flawed, I can say, like,

267
00:32:18.360 --> 00:32:28.199
Al Newkirk: Do I have any memories about Rust workspace sizes? It's now going to… well, it's gonna find it because it didn't create it as pending.

268
00:32:29.310 --> 00:32:39.899
Al Newkirk: So, I think I have another ledger that I sort of play around with that has a bunch of propositions that I'll sort of switch to. But as you can see, it found the other workspace, and

269
00:32:40.580 --> 00:32:54.329
Al Newkirk: Now, the way that I want you to think about this is, imagine that you had different ledgers for different purposes. Maybe you had a ledger about a project, maybe you had a ledger about your, like, engineering practices. So, you don't have to tell the agent

270
00:32:55.270 --> 00:33:01.649
Al Newkirk: every time you work on a new API, that you prefer versioning in the headers and not in the path.

271
00:33:01.790 --> 00:33:07.870
Al Newkirk: Or that you prefer a maximum or path…

272
00:33:08.370 --> 00:33:14.539
Al Newkirk: when constructing… when designing URLs. Like, you can have those preferences as

273
00:33:14.850 --> 00:33:20.379
Al Newkirk: And the agent can just sort of find them, and you can share them across AI agent harnesses.

274
00:33:21.270 --> 00:33:29.989
Al Newkirk: So, let's see, I'm gonna open up another terminal, and I'm just gonna see… I gotta get out of this directory so that I can… so I'm back in the global space.

275
00:33:30.700 --> 00:33:33.170
Al Newkirk: And I'm gonna look at the ledgers that I have.

276
00:33:33.920 --> 00:33:35.420
Al Newkirk: on this machine.

277
00:33:35.620 --> 00:33:45.629
Al Newkirk: D here, what's in the default?

278
00:33:45.800 --> 00:33:50.960
Al Newkirk: So I'm gonna go to Fact Use, so this is how you switch, ledgers, so Fact Use Default.

279
00:33:51.160 --> 00:34:02.799
Al Newkirk: I'm going to look at what's in this ledger. It's a random notes… maybe… And I'm gonna switch to…

280
00:34:04.190 --> 00:34:05.209
Al Newkirk: I said, no.

281
00:34:05.800 --> 00:34:07.280
Al Newkirk: And… okay.

282
00:34:07.800 --> 00:34:11.509
Al Newkirk: Yeah, this is, I think, is like a throwaway,

283
00:34:12.130 --> 00:34:16.920
Al Newkirk: Not throwaway, but this is sort of like a, random…

284
00:34:18.870 --> 00:34:25.270
Al Newkirk: ledger that I, that I'll use sometimes. Now, this actually highlights an interesting

285
00:34:25.790 --> 00:34:42.490
Al Newkirk: parallel with Git. It is a distributed version control system, so it has… it also supports the notion of remotes, and you can push and pull to ledgers on other machines. But this is called default remote because

286
00:34:42.550 --> 00:34:47.580
Al Newkirk: the origin ledger is actually on an iMac that,

287
00:34:47.880 --> 00:34:51.670
Al Newkirk: that I… I connect to, and I can… Think?

288
00:34:51.889 --> 00:34:55.059
Al Newkirk: Let me just see if the remote is wired up.

289
00:34:55.440 --> 00:34:59.579
Al Newkirk: ready, because I think I might be able to push and pull from it, so…

290
00:35:00.210 --> 00:35:03.779
Al Newkirk: So the remote is here… let me try to pull…

291
00:35:04.990 --> 00:35:11.429
Al Newkirk: Okay, well, I'll… you know, but needless to say that you can push and pull to remotes, and

292
00:35:11.860 --> 00:35:16.330
Al Newkirk: That way, the knowledge is not only versioned, but distributed.

293
00:35:16.450 --> 00:35:26.380
Al Newkirk: Okay, so now that I'm… So…

294
00:35:27.240 --> 00:35:33.090
Al Newkirk: So when you do, for example, Sorry.

295
00:35:34.840 --> 00:35:38.299
Al Newkirk: In the normative, like, use of this.

296
00:35:39.080 --> 00:35:44.250
Al Newkirk: It uses a globe, your ledgers are stored in a, like.

297
00:35:44.940 --> 00:35:50.400
Al Newkirk: Neutral place where the current logged-in user And…

298
00:35:51.950 --> 00:35:54.799
Al Newkirk: That's the ledgers from any directory.

299
00:35:55.160 --> 00:35:56.560
Al Newkirk: on the system.

300
00:35:56.920 --> 00:36:00.650
Al Newkirk: So, for example, like, if I'm… if I'm here.

301
00:36:01.480 --> 00:36:04.969
Al Newkirk: I do, like, fact ledgers, ledger lists.

302
00:36:05.180 --> 00:36:11.500
Al Newkirk: I can see the ledgers. If I move into, like, downloads, and I do the same command, I can see the same ledgers.

303
00:36:11.670 --> 00:36:16.399
Al Newkirk: But if I go back to,

304
00:36:16.960 --> 00:36:21.730
Al Newkirk: I'm just gonna create a temporary directory. If I, if I say fact here.

305
00:36:22.010 --> 00:36:32.070
Al Newkirk: it basically creates a localized… same if you did, like, Git init. It creates a localized, directory-specific, notice.

306
00:36:33.240 --> 00:36:37.469
Al Newkirk: workspace for FAST. So if I did, like, FAST ledger list.

307
00:36:38.370 --> 00:36:46.280
Al Newkirk: this thing is empty, so there's nothing here. It's gonna say there's no ledgers, and if I do, like, fact init, it's gonna create the default ledger.

308
00:36:47.470 --> 00:36:53.550
Al Newkirk: And so, you'll just see the one ledger, and it's all, like, very unique to this. And if you inspect the .factor

309
00:36:56.230 --> 00:37:09.830
Al Newkirk: directory, you'll see, like, there's no remotes, but here's the catalog. The C file is your private key, because you… because every event that you cause is signed cryptographically, and

310
00:37:10.010 --> 00:37:12.620
Al Newkirk: That's…

311
00:37:12.820 --> 00:37:27.209
Al Newkirk: how the, end-only ledger is validated, etc, etc. All this stuff is, oh, and the ledgers are in the Rust implementation, SQLite databases.

312
00:37:28.490 --> 00:37:34.239
Al Newkirk: that's the private key. Okay, so…

313
00:37:35.040 --> 00:37:42.249
Al Newkirk: What I want to do, for demonstration purposes is I want to get out of this…

314
00:37:43.080 --> 00:37:47.819
Al Newkirk: directory, I want to use the global, like, stores, I want to switch to…

315
00:37:52.140 --> 00:37:55.829
Al Newkirk: I think, personal is interesting.

316
00:38:01.830 --> 00:38:02.520
Al Newkirk: Huh.

317
00:38:02.680 --> 00:38:05.089
Al Newkirk: Okay, I'll use,

318
00:38:12.890 --> 00:38:20.770
Al Newkirk: Just give me one second. I'm just gonna use memory research. I mean, it's… It's actually a,

319
00:38:22.950 --> 00:38:28.560
Al Newkirk: It's actually a ledger that I do use. It's a real ledger that I use, and…

320
00:38:28.790 --> 00:38:33.389
Al Newkirk: Every time… okay, so this actually might be a good example of…

321
00:38:33.560 --> 00:38:36.519
Al Newkirk: why and how you might use a ledger, so…

322
00:38:36.700 --> 00:38:46.689
Al Newkirk: The Memory Research Ledger is all of the knowledge and research and information that I find important in learning about

323
00:38:49.550 --> 00:38:52.479
Al Newkirk: memory management is being done with AI.

324
00:38:54.290 --> 00:38:56.890
Al Newkirk: And… you know.

325
00:38:57.250 --> 00:39:14.430
Al Newkirk: it's… having it as its own ledger is a way of sort of segmenting that knowledge. I don't want to have to fish through, like, you know, my professional development ideas and, you know, my Pearl ideas and memory management ideas. I want, like.

326
00:39:14.480 --> 00:39:20.790
Al Newkirk: I want a repository of knowledge specific to, like, memory research. So, if I were to, like,

327
00:39:21.550 --> 00:39:25.829
Al Newkirk: I don't know, like, I like Flawed, so let's… if I were to launch Vlad…

328
00:39:26.370 --> 00:39:44.759
Al Newkirk: Now, the difference between, for example, this, this instance of Claude is… this instance of Claude is running an attempt directory where the fax… where, like, a directory-specific version of FACTS was created with, like, fact here.

329
00:39:45.050 --> 00:39:53.499
Al Newkirk: So… So this is only gonna see, like, the ledgers that were created in this directory.

330
00:39:55.110 --> 00:39:58.209
Al Newkirk: But I launched this other process.

331
00:40:00.430 --> 00:40:06.200
Al Newkirk: like, from the home directory. So this has access to all of the…

332
00:40:06.550 --> 00:40:11.250
Al Newkirk: a global ledger. So if I were to do, like, that as…

333
00:40:11.710 --> 00:40:14.270
Al Newkirk: It's gonna… you know, it's…

334
00:40:14.580 --> 00:40:27.050
Al Newkirk: I'm currently… I'm… I am Al Newkirk on the Memory Research Ledger. So, so now, this… so Claude is using the memory research ledger, and if I were to say something like,

335
00:40:32.570 --> 00:40:42.760
Al Newkirk: Provide a simple bullet list of the… companies… that provide… memory for agents.

336
00:40:49.160 --> 00:40:51.920
Al Newkirk: Okay, so I'm gonna say recall and provide.

337
00:40:52.210 --> 00:40:54.010
Al Newkirk: Recall is kind of like a…

338
00:40:54.310 --> 00:41:07.160
Al Newkirk: a trigger word in the memory space, it usually indicates that you're trying to look something up from memory. This should use the fax system, but again, that bootstrapping problem might rear its head.

339
00:41:07.930 --> 00:41:12.930
Al Newkirk: Where it might look to its own memory first. Okay, so look, it used, fact…

340
00:41:13.540 --> 00:41:25.439
Al Newkirk: So it's… it's using the fact system. I didn't… I didn't have to nudge it in this session to do so. So it's gonna look through the… all of the memories in the memory research ledger.

341
00:41:25.560 --> 00:41:33.250
Al Newkirk: And it's gonna, like, it's gonna create a mapping and pull out a list of companies that are…

342
00:41:33.520 --> 00:41:35.769
Al Newkirk: memory management for AI agents.

343
00:41:39.710 --> 00:41:44.879
Al Newkirk: Any questions or thoughts about any of this before I, like, continue? Let's see, I just wanna…

344
00:41:45.720 --> 00:41:51.480
Al Newkirk: Okay, so here's what it came back with, and this is all from the…

345
00:41:53.060 --> 00:42:01.509
Al Newkirk: from the ledger. Like, if I were to… so, Memzero is one, ZEP, Leta, SuperMemory, Cogni is one,

346
00:42:01.880 --> 00:42:15.879
Al Newkirk: open source projects without a company behind them, and the same ledger is useful, beads, membrane, Memento. So these are all, from this, ledger. So if I were to… if I were to open up a,

347
00:42:17.040 --> 00:42:27.829
Al Newkirk: open up a terminal, go to the home directory. You don't have to be in the home directory, but I do just do fax as. Okay, I'm still on the memory research ledger. If I were to do, like, fact search.

348
00:42:28.070 --> 00:42:31.400
Al Newkirk: And I would've do, like.

349
00:42:32.570 --> 00:42:42.480
Al Newkirk: MIM0, so these are all the facts that reference MIM0. It is using vector searching, so… Hmm.

350
00:42:43.150 --> 00:42:48.350
Al Newkirk: And so that's how it's ranking and returning search results.

351
00:42:50.260 --> 00:42:56.579
Al Newkirk: There's also a command called FactFind, and it allows you… it just does kind of, like, it allows…

352
00:42:56.940 --> 00:43:04.149
Al Newkirk: like, piping in a way, where you can do something with the search results. So, for example, say, like, fact find and zero.

353
00:43:04.910 --> 00:43:08.390
Al Newkirk: And then I can, like, pick Number one.

354
00:43:08.900 --> 00:43:18.199
Al Newkirk: And and then I can give it… tell it… tell it where I want to pipe that proposition, so I can say, like, with open. And so it's going to open that first one.

355
00:43:18.590 --> 00:43:24.209
Al Newkirk: And this is what's actually in the content. It's marked down. If I, if I did, like, pick two.

356
00:43:24.340 --> 00:43:28.509
Al Newkirk: It's gonna open up, like, this other thing, but they're, like, big three.

357
00:43:29.570 --> 00:43:33.069
Al Newkirk: It's gonna open up the third result in the editor.

358
00:43:33.550 --> 00:43:42.579
Al Newkirk: Now you can't, because… Because you're forced to go through… A revision and deliberation process.

359
00:43:42.890 --> 00:43:47.990
Al Newkirk: To register a revision, like, changing the text here won't…

360
00:43:48.880 --> 00:43:55.469
Al Newkirk: will not be persistent. So, it's opening the proposition in an editor for viewing purposes only.

361
00:43:55.600 --> 00:44:10.749
Al Newkirk: But I could… you could pipe it to, like, revise. I don't want to, but I'm… you could pipe it to revise as a command, and you would be able to, like, create a revision from it, because revise is one of the commands.

362
00:44:12.520 --> 00:44:26.029
Al Newkirk: So, let's see, one other thing. I want to go back here, and I want to say, like, and this is… this is actually, with this particular ledger, this is stuff that I… I do when I'm not presenting, so I might want to say…

363
00:44:26.290 --> 00:44:30.350
Al Newkirk: I mean… This is a real thing, actually.

364
00:44:33.910 --> 00:44:40.940
Al Newkirk: There were two thefts that contained… blood, artifact.

365
00:44:42.800 --> 00:44:43.820
Al Newkirk: about…

366
00:44:48.250 --> 00:44:51.420
Al Newkirk: memory, product, analysis.

367
00:44:52.080 --> 00:44:54.750
Al Newkirk: cross… providers.

368
00:44:58.060 --> 00:44:59.879
Al Newkirk: Surface those facts.

369
00:45:02.870 --> 00:45:03.650
Al Newkirk: Okay.

370
00:45:03.950 --> 00:45:11.110
Al Newkirk: And so… the voice recognition is not working like it should, so there are two facts that contain

371
00:45:11.480 --> 00:45:13.030
Al Newkirk: flawed.

372
00:45:17.560 --> 00:45:19.200
Al Newkirk: Clawed artifact.

373
00:45:19.670 --> 00:45:23.269
Al Newkirk: About memory product analyses across providers,

374
00:45:23.640 --> 00:45:29.120
Al Newkirk: So this is actually a real thing, and this is an example of, oh, I forgot where…

375
00:45:29.350 --> 00:45:40.459
Al Newkirk: So, if you were using Markdown files, you might have to grep across a bunch of directories, like, and try to remember, like, what specific kinds of text were in

376
00:45:40.950 --> 00:45:43.210
Al Newkirk: the markdown files,

377
00:45:44.420 --> 00:46:01.920
Al Newkirk: But instead of that, you could just, like, put your… your knowledge into ledgers, and you could lean on the vector searching to surface. And you can use AI to automate the vector searching to recall those. So this is actually correct. So there were two artifacts that…

378
00:46:03.580 --> 00:46:13.470
Al Newkirk: what, what, what Claude calls artifacts, I think, I think are, like, HTML pages, That represent, like, some…

379
00:46:13.840 --> 00:46:20.479
Al Newkirk: research or whatever. So these are actually the URLs that I was looking for, and in fact,

380
00:46:20.710 --> 00:46:26.529
Al Newkirk: Let me just, like, share my entire screen, that way you can see me go from the links to…

381
00:46:27.710 --> 00:46:32.919
Al Newkirk: Okay, so this one was, Agent Memory Feature Mat. I was doing some research about

382
00:46:33.280 --> 00:46:37.850
Al Newkirk: What are the things that I'm providing in this open source,

383
00:46:40.690 --> 00:46:47.730
Al Newkirk: software system, and what are some of the features that are provided from these other players that have SaaS products? And so…

384
00:46:49.440 --> 00:46:55.769
Al Newkirk: And so, this is, and so, this was something that Claude did, and…

385
00:46:57.400 --> 00:47:05.670
Al Newkirk: It lists out all the different players, and at the bottom, it's like a feature matrix of the things that it does, and

386
00:47:05.960 --> 00:47:11.010
Al Newkirk: And fax actually does some things that these other

387
00:47:11.250 --> 00:47:17.430
Al Newkirk: solutions don't do. One of the… one of the big things that FACT is based on

388
00:47:17.640 --> 00:47:24.680
Al Newkirk: That these other systems don't really do, they don't actually do, is differentiate between

389
00:47:24.860 --> 00:47:35.819
Al Newkirk: oppositions and facts. They don't differentiate between Like, accepted, consensus-based knowledge, and…

390
00:47:36.560 --> 00:47:50.049
Al Newkirk: something being proposed to become accepted knowledge. So, you know, you might be thinking, if I just had Markdown files in a Git repository, and I had all my agents share that.

391
00:47:50.640 --> 00:47:57.940
Al Newkirk: why wouldn't that suffice? Well, there's a couple of flaws with that strategy, but…

392
00:47:58.720 --> 00:48:04.019
Al Newkirk: A big one is that, you know, just because an agent writes something.

393
00:48:04.600 --> 00:48:23.600
Al Newkirk: or a person writes something, doesn't mean that every other agent using that concept should automatically pick it up and accept it as truth. I mean, this is how you allow, like, poison pill… poison messages to, like… you could… this is an attack vector, it's… you know, you don't want,

394
00:48:23.780 --> 00:48:36.920
Al Newkirk: And even when you have, like, agents writing back to their own knowledge systems, you want to have a process where there's checks and balances, or it just gets accepted and propagated across a fleet of AI agents.

395
00:48:37.010 --> 00:48:44.959
Al Newkirk: And so, most of these systems haven't solved that yet, but that's one of the fundamental underpinnings of the fact system.

396
00:48:45.250 --> 00:48:51.309
Al Newkirk: is the differentiate… differentiating between, propositions and knowledge. Anyway,

397
00:48:52.710 --> 00:48:56.530
Al Newkirk: If I wanted to, like, open this, I could…

398
00:48:56.650 --> 00:49:03.280
Al Newkirk: Let me do it in another terminal, but I'll just say, like, open that identifier…

399
00:49:04.230 --> 00:49:12.030
Al Newkirk: And, this is where it found and pulled the URL when I asked the AI agent to find me the sub.

400
00:49:12.580 --> 00:49:20.050
Al Newkirk: It had an artifact with, like, analyses, so it found, like, analyses, it found, like, term artifact.

401
00:49:20.270 --> 00:49:22.680
Al Newkirk: And that's how I was able to recall.

402
00:49:26.310 --> 00:49:30.780
Al Newkirk: That's how I was able to recall. So I'm gonna, like,

403
00:49:30.970 --> 00:49:33.490
Al Newkirk: I'm gonna pause there and sort of, like.

404
00:49:35.360 --> 00:49:38.580
Al Newkirk: Open it up for questions, if any of you have any

405
00:49:39.670 --> 00:49:49.809
Al Newkirk: demos you want me to try to perform, I'm happy to do that. If you have questions about… or feedback about the project so far, I'd love to hear what you think.

406
00:49:53.270 --> 00:50:01.839
Gabor Szabo: Well, first of all, thank you very much for this presentation. It was very interesting. It seems that the audience is very silent now, today.

407
00:50:02.280 --> 00:50:05.549
Gabor Szabo: And if you have any questions, that you forgot to…

408
00:50:05.550 --> 00:50:11.150
Uri Bruck: Okay. I'm not really familiar with systems like this, but when you chat with… when you…

409
00:50:11.210 --> 00:50:26.280
Uri Bruck: share memory with documents with the AI. The demos I've seen so far were for Notebook LM, where you stick a lot… upload a bunch of documents, and then you have a chat with… you can ask about the AI about what's in the documents.

410
00:50:26.280 --> 00:50:31.689
Uri Bruck: So, how is this similar? How, can you give me sort of, like,

411
00:50:33.060 --> 00:50:36.389
Al Newkirk: Yeah, so, I think that, I don't…

412
00:50:36.540 --> 00:50:43.270
Al Newkirk: I am familiar with, people using Notebook LM, to…

413
00:50:43.820 --> 00:50:47.730
Al Newkirk: To be able to… so, okay.

414
00:50:48.610 --> 00:50:52.049
Al Newkirk: I don't think that… Most people are…

415
00:50:52.300 --> 00:50:58.210
Al Newkirk: connecting their production AI agents to Notebook LM, like…

416
00:50:58.630 --> 00:51:07.030
Al Newkirk: for business purposes, I think that individuals might use, like, Claude locally and connect it to Notebook LM.

417
00:51:07.860 --> 00:51:14.309
Al Newkirk: But the… let me explain, like, what Notebook LM… It's designed to do.

418
00:51:14.550 --> 00:51:25.040
Al Newkirk: And it is useful. It is useful. So, Notebook LM is designed to… silo, the…

419
00:51:27.450 --> 00:51:34.780
Al Newkirk: the inference. So, instead of… so, for example, in the… earlier in this demo, when I asked about Rust.

420
00:51:35.090 --> 00:51:43.170
Al Newkirk: It… and it pulled from… like, it's trained knowledge. Notebook LM is specifically designed to not do that.

421
00:51:43.420 --> 00:51:51.569
Al Newkirk: So it's designed to only reference the information that you give it, so to not pull from

422
00:51:51.800 --> 00:52:05.380
Al Newkirk: it's training data. And so this is good. So this is good because you know that the answers that it's giving you are specific to the source material that you gave it, and that it's not, sort of.

423
00:52:05.540 --> 00:52:16.469
Al Newkirk: hallucinating or, like, pulling from other sources that are not in the set that you gave it. That's the main… that's why Notebook LM, like.

424
00:52:16.630 --> 00:52:24.519
Al Newkirk: exists. And… and so, you would give it a bunch of, information, and…

425
00:52:24.650 --> 00:52:27.290
Al Newkirk: Including, like, PDFs and spreadsheets and…

426
00:52:27.470 --> 00:52:32.589
Al Newkirk: And it will be able to, like, reference that, reference that in…

427
00:52:34.390 --> 00:52:39.770
Al Newkirk: in, like, whatever questions you ask it. This system cannot do that. This system…

428
00:52:40.160 --> 00:52:45.170
Al Newkirk: It's designed to capture, like, atomized knowledge.

429
00:52:45.370 --> 00:52:48.009
Al Newkirk: So you, you wouldn't, you wouldn't point…

430
00:52:48.120 --> 00:53:01.540
Al Newkirk: This system at files that it's going to parse for information, like PDFs, spreadsheets, etc. You would… Oof.

431
00:53:02.380 --> 00:53:09.250
Al Newkirk: This system is designed for contextual… demise.

432
00:53:09.480 --> 00:53:12.620
Al Newkirk: units of information. So, think, for example.

433
00:53:15.940 --> 00:53:27.189
Al Newkirk: AI agents, like, Making conclusions… and then filing them for later reflection. Or humans, like.

434
00:53:28.940 --> 00:53:37.180
Al Newkirk: being very precise about instructions, even in the, even in, like, I mean…

435
00:53:37.530 --> 00:53:42.269
Al Newkirk: what I've created in this memory ledger is indicative of the kinds of

436
00:53:44.440 --> 00:53:50.180
Al Newkirk: like, atomized knowledge that you would want, or… for example, like, if I were to open up this,

437
00:53:50.550 --> 00:53:52.510
Al Newkirk: memory poisoning.

438
00:53:54.090 --> 00:53:58.839
Al Newkirk: You know, this is the kind of thing you would expect in this system.

439
00:53:59.870 --> 00:54:05.670
Al Newkirk: Now, you could use… they're not mutually exclusive, like, if you wanted to… if I, today.

440
00:54:05.880 --> 00:54:11.150
Al Newkirk: being the author of the system. If I wanted to take a… Thank you.

441
00:54:11.880 --> 00:54:13.859
Al Newkirk: under-page PDF.

442
00:54:14.230 --> 00:54:18.239
Al Newkirk: And… I wanted to, like.

443
00:54:18.860 --> 00:54:22.259
Al Newkirk: questions about it, I would use Nobelzella. Like, absolutely.

444
00:54:22.620 --> 00:54:24.419
Al Newkirk: But,

445
00:54:24.920 --> 00:54:30.020
Al Newkirk: But what I will… but what I would absolutely… and I have done this, actually, I can show you examples of it.

446
00:54:30.530 --> 00:54:38.000
Al Newkirk: In fact, I could tell by the formatting, because Nobook LM formats things like this. Like, the bullet points are… the bullet points in the…

447
00:54:38.840 --> 00:54:48.289
Al Newkirk: bolding. But anyway, facts is about, like, like, the decisions and the…

448
00:54:48.740 --> 00:54:51.500
Al Newkirk: Capturing the thinking and the knowledge.

449
00:54:51.740 --> 00:54:57.670
Al Newkirk: So, I wouldn't put the whole PDF into this system. I would put my conclusions

450
00:54:58.000 --> 00:55:01.310
Al Newkirk: about the PDF in this system, if that makes sense.

451
00:55:02.300 --> 00:55:07.959
Uri Bruck: I think it makes sense, yeah, sure. So we're dealing with lots of much smaller units for facts and conclusions.

452
00:55:08.080 --> 00:55:08.580
Uri Bruck: And

453
00:55:08.580 --> 00:55:09.290
Al Newkirk: Yes.

454
00:55:09.520 --> 00:55:11.720
Uri Bruck: Yeah, so it makes influences from that, and…

455
00:55:12.520 --> 00:55:14.969
Al Newkirk: Exactly. That's right, yeah, yeah, that's right.

456
00:55:15.310 --> 00:55:15.860
Uri Bruck: Yeah.

457
00:55:17.180 --> 00:55:25.610
Al Newkirk: So one of the things that I wanted to demonstrate in this demo that I think is very useful is that,

458
00:55:25.860 --> 00:55:30.809
Al Newkirk: You can use… you're not locked into… this is the biggest, unlock.

459
00:55:32.660 --> 00:55:39.529
Al Newkirk: That, if you don't take away anything else, from, from this demo,

460
00:55:41.510 --> 00:55:49.009
Al Newkirk: If you spend a lot of time with an AI agent, You're building up… memory and knowledge.

461
00:55:49.300 --> 00:55:55.940
Al Newkirk: And… And the AI will become more effective

462
00:55:56.050 --> 00:55:58.489
Al Newkirk: Based on how much context it had.

463
00:56:00.640 --> 00:56:06.409
Al Newkirk: But unless you're intentional about using… about having a…

464
00:56:06.540 --> 00:56:11.249
Al Newkirk: vendor-neutral memory system, you're gonna be locked into

465
00:56:12.280 --> 00:56:17.009
Al Newkirk: a particular vendor. This happened to me, this is a real thing that happened to me,

466
00:56:17.870 --> 00:56:24.579
Al Newkirk: I was using, Claude. I started with Claude, and I was… I became a big fan of Claude.

467
00:56:24.950 --> 00:56:30.970
Al Newkirk: And… I wanted to upgrade to a higher-priced plan.

468
00:56:31.180 --> 00:56:39.369
Al Newkirk: And for whatever reason, the fact that I wanted to upgrade to, like, a $200 a month… $200 a month plan.

469
00:56:39.730 --> 00:56:42.760
Al Newkirk: It triggered them to want to…

470
00:56:43.830 --> 00:56:54.850
Al Newkirk: verify my identity. Now, the short story, short story is, I had trouble with my passport, and because I couldn't verify my identity, they actually locked me out of my account.

471
00:56:55.230 --> 00:57:06.250
Al Newkirk: And all of the memories, all of the… like, all of the projects that I had in progress, I couldn't continue working on, because all of the… I could have tried to use another agent.

472
00:57:06.250 --> 00:57:15.450
Al Newkirk: But all of the, like, knowledge of where I was in the project, and what I was planning to do, all that knowledge was trapped in that particular agent.

473
00:57:15.560 --> 00:57:27.940
Al Newkirk: So I ended up, because I couldn't get a hold of Anthropic, I ended up switching to Codex and just starting from scratch, but I guess that also led me on a journey of…

474
00:57:28.060 --> 00:57:32.680
Al Newkirk: Not wanting to be locked into a particular vendor's memory system.

475
00:57:34.890 --> 00:57:36.570
Al Newkirk: So, yeah, you could…

476
00:57:37.120 --> 00:57:40.850
Uri Bruck: You just stop all knowledge, and you can just take it to another agent, and…

477
00:57:41.900 --> 00:57:47.289
Al Newkirk: Yeah, and I'm gonna, I'm gonna do… so I already did that with, with Claude and…

478
00:57:47.650 --> 00:57:57.090
Al Newkirk: Codex. There's another open source, AI agent harness called Pi, and if I, like, launch PI,

479
00:57:58.060 --> 00:58:04.680
Al Newkirk: you know, because I had the skills that work across agents, I could, I could just say,

480
00:58:05.450 --> 00:58:08.350
Al Newkirk: Let's see… so I'm using Pi right now.

481
00:58:08.930 --> 00:58:18.030
Al Newkirk: And for some reason… oh, okay, sorry, give me a second, because I'm still in that folder, so if I go here, and I relaunch Pi…

482
00:58:19.570 --> 00:58:20.900
Al Newkirk: do, like…

483
00:58:21.060 --> 00:58:27.409
Al Newkirk: act as… okay, I'm in the memory research, so now I could ask Papa the same thing. I could say, like,

484
00:58:28.150 --> 00:58:33.980
Al Newkirk: What do I… what did I bid?

485
00:58:34.820 --> 00:58:41.890
Al Newkirk: Okay, yeah, I'll answer the same thing. This is a whole completely separate, like, like, harness.

486
00:58:42.060 --> 00:58:46.979
Al Newkirk: That there are two facts that contained clawed artifacts about memory products, blah blah blah.

487
00:58:47.320 --> 00:58:49.210
Al Newkirk: There's a whole different,

488
00:58:51.290 --> 00:58:55.450
Al Newkirk: And as you can see, it's doing its thing, it's using the fax memory system.

489
00:58:55.850 --> 00:59:04.470
Al Newkirk: It's, it's using the current ledger, which is the memory research ledger, and it found that that was actually pretty quick. I think that was actually quicker than…

490
00:59:04.640 --> 00:59:08.960
Al Newkirk: flawed, but… But it found it, so it went through.

491
00:59:09.110 --> 00:59:11.630
Al Newkirk: So as you can see.

492
00:59:11.830 --> 00:59:14.359
Al Newkirk: Like, I'm using a completely different AI agent.

493
00:59:14.520 --> 00:59:18.860
Al Newkirk: And… it's using the same memory, so the memory is now portable.

494
00:59:20.320 --> 00:59:27.629
Al Newkirk: And if I were to, yeah, if I were to… because it's, distributed, because you can push and pull, like, get…

495
00:59:28.200 --> 00:59:35.699
Al Newkirk: if I were to, like, launch an AI as a product, like, you know, like an API, but an AI agent.

496
00:59:35.880 --> 00:59:45.870
Al Newkirk: I could have it push and pull memory, and… And all of the… Un- unlike…

497
00:59:47.490 --> 00:59:53.879
Al Newkirk: which will require, I guess, like, network-attached storage, or maybe, maybe, maybe?

498
00:59:54.190 --> 01:00:01.129
Al Newkirk: Unlike that, like, different agents would be able to share the… The memories and… yeah, stuff.

499
01:00:02.120 --> 01:00:02.980
Al Newkirk: Yup.

500
01:00:04.150 --> 01:00:04.570
Uri Bruck: Okay.

501
01:00:04.570 --> 01:00:07.859
Al Newkirk: I think we… I think we lost, Paul? Powell?

502
01:00:08.180 --> 01:00:10.200
Uri Bruck: He left at some point, he still left a message on the chat.

503
01:00:10.200 --> 01:00:11.650
Al Newkirk: Okay, okay, okay.

504
01:00:13.230 --> 01:00:18.210
Uri Bruck: Yeah, okay, well, that's great, that's… I'd love to look into that, you know, this, you know…

505
01:00:18.320 --> 01:00:19.850
Uri Bruck: This is a very nice introduction.

506
01:00:20.150 --> 01:00:20.969
Uri Bruck: Yeah, thank you.

507
01:00:21.610 --> 01:00:27.510
Uri Bruck: And I like the philosophy behind it, when you said you didn't… you said you didn't know much about philosophy, but you have a very nice philosophy behind it, so…

508
01:00:27.970 --> 01:00:29.999
Al Newkirk: Oh, thank you, thank you, I appreciate that.

509
01:00:31.990 --> 01:00:38.449
Al Newkirk: I guess, I guess now that we've sort of… oh, should we, should we leave it there, Gabor?

510
01:00:38.450 --> 01:00:43.559
Gabor Szabo: Yeah, I mean, I think we are… we are done. I mean, we can turn off the camera now, if you like.

511
01:00:43.560 --> 01:00:44.080
Al Newkirk: Okay, okay.

512
01:00:44.080 --> 01:00:46.090
Gabor Szabo: I'll continue talking about other things.

513
01:00:46.740 --> 01:00:49.339
Al Newkirk: Yeah, yeah, so I wanted to ask about…

514
01:00:49.680 --> 01:01:05.330
Gabor Szabo: Wait a second, let me just finish, say a few words for those people who are also in the video. So just remember to like the video again, and follow the channel, and below the video, you will find the link to the… to… to…

515
01:01:05.370 --> 01:01:13.620
Gabor Szabo: various things about this presentation and to the future events. And thank you very much again for this presentation, it was very interesting.

516
01:01:13.680 --> 01:01:14.270
Gabor Szabo: Bye-bye.

517
01:01:14.270 --> 01:01:14.970
Al Newkirk: No problem.

