---
title: "Rust and the GPU from Native to Web: An Introduction to `wgpu` by Liam Rust"
timestamp: 2026-10-23T11:30:01
author:
published: true
show_related: true
description:
tags:
---

## Description

The GPU can be one of the most powerful pieces of hardware in a device, and they are in almost every personal device with a screen. Despite this, it's usually quite difficult to actually utilize them, requiring platform and hardware-specific considerations. Enter `wgpu`: Rust's low-level, yet platform-agnostic and safe GPU library. This talk will cover what `wgpu` is and does, how you can use it for everything from parallel computations to rendering, and walk you through how to use it starting from scratch. Additionally, we'll cover how you can compile it for use in the browser!

## Bio

[Liam Rust](https://www.linkedin.com/in/liam-rust/) is a recent masters graduate of computer science from Stevens Institute of Technology. Liam is currently an embedded software engineering intern at Persistent Systems. Liam has many interests, but include all things Rust, full-stack web development, free and open source software, and Linux. Outside of programming, Liam enjoys singing, music, design, urbanism, and modeling.

## Length

30 min


{% youtube id="LnxuQKoxvFM" file="2026-10-22-rust-and-the-gpu-from-native-to-web-an-introduction-to-wgpu-by-liam-rust.mp4" %}


## Transcription

00:00:01.000 --> 00:00:08.000
So, hello and welcome to the Code Maven YouTube channel, if you are watching it on YouTube, and if you are present here, then welcome to the…

00:00:08.000 --> 00:00:14.000
Rust Maven meeting. Thank you, Liam, for agreeing to give this presentation.

00:00:14.000 --> 00:00:23.000
My name is Gabor. I organize these events in various… for various programming languages and various technologies,

00:00:23.000 --> 00:00:28.000
If you're watching the video, then you will find below the video, you will find a link

00:00:28.000 --> 00:00:35.000
to details about this presentation specifically, maybe even to the slides. We'll see what can be shared.

00:00:35.000 --> 00:00:43.000
And anything Leanne wants to share will be linked there. Also, to the future events, it will be there

00:00:43.000 --> 00:00:46.000
And, um, everyone who is here, thank you for…

00:00:46.000 --> 00:00:54.000
Joining us, as I mentioned, you're free to ask questions in the chat, and then…

00:00:54.000 --> 00:01:02.000
Depending on the arm, I can read out those questions during the presentation, or we can leave it to the end. What would you prefer?

00:01:02.000 --> 00:01:04.000
Um, I think…

00:01:04.000 --> 00:01:07.000
Uh, during the end will be easier.

00:01:07.000 --> 00:01:12.000
And, and, okay, so after you freeze the presentation, we go… we can go over the questions,

00:01:12.000 --> 00:01:15.000
And then you can selectively answer whatever you prefer.

00:01:15.000 --> 00:01:16.000
Yeah.

00:01:16.000 --> 00:01:22.000
And then once we are done with that, then we shut down the video recording, then we can… everyone can stay around.

00:01:22.000 --> 00:01:25.000
And, uh, we can have a free chat, uh…

00:01:25.000 --> 00:01:30.000
We… of the record, exactly, okay? So…

00:01:30.000 --> 00:01:36.000
Uh, that's it. Leon, again, thank you for agreeing to give this presentation. Now the stage is yours, you can…

00:01:36.000 --> 00:01:38.000
Introduce yourself, and…

00:01:38.000 --> 00:01:39.000
Go ahead.

00:01:39.000 --> 00:01:44.000
All right, thank you. Yeah. Um, so, hi, uh, my name is Liam Rust, um,

00:01:44.000 --> 00:01:48.000
funnily enough, my last name is actually Rust, um…

00:01:48.000 --> 00:01:54.000
And this is a talk that I gave back in August at the Rust NYC meetup.

00:01:54.000 --> 00:01:57.000
And I'm doing it again for you guys, and it's…

00:01:57.000 --> 00:02:00.000
An introduction to WGPU.

00:02:00.000 --> 00:02:04.000
And because it's rust, I have…

00:02:04.000 --> 00:02:06.000
speaks, like, obligatory crap.

00:02:06.000 --> 00:02:12.000
Okay, so, uh, who am I? So, I am A Things. Um, there's Liam.

00:02:12.000 --> 00:02:20.000
I use any pronouns, non-binary. Um, I am an enjoyer of software, of coding, and of using it.

00:02:20.000 --> 00:02:29.000
Love doing some systems programming, doing some web programming, doing some DevOps, containerization web server stuff, um, I have a little home lab at home.

00:02:29.000 --> 00:02:34.000
Um, I enjoy singing and listening to music, um…

00:02:34.000 --> 00:02:40.000
My favorite artist right now is Jimmy Page, uh, kind of in Jokoid stuff. And I also love design!

00:02:40.000 --> 00:02:44.000
Um, things like UI design, or even, like, fonts, UX, all that.

00:02:44.000 --> 00:02:47.000
Um, I'm telling you all this because…

00:02:47.000 --> 00:02:51.000
Um, I am not quite an expert in cargo rating.

00:02:51.000 --> 00:02:57.000
But that's the good news, because it means that you're not… if you're not an expert, you can still do this.

00:02:57.000 --> 00:02:59.000
Um, and…

00:02:59.000 --> 00:03:02.000
Yeah, it was a really fun time. So…

00:03:02.000 --> 00:03:05.000
Let's dive into it.

00:03:05.000 --> 00:03:09.000
So, first of all, we have to, um, answer, what is a GPU?

00:03:09.000 --> 00:03:12.000
Uh, because we want to actually use a GPU, what is it?

00:03:12.000 --> 00:03:17.000
It is a parallel computation device. It is structured…

00:03:17.000 --> 00:03:19.000
quite differently to a CPU. A CPU?

00:03:19.000 --> 00:03:21.000
is optimized for

00:03:21.000 --> 00:03:24.000
being able to process…

00:03:24.000 --> 00:03:30.000
a sequence of instructions, one after another. It can handle more complex instructions.

00:03:30.000 --> 00:03:33.000
But it can't parallelize as well.

00:03:33.000 --> 00:03:36.000
you know, like, you can have a certain amount of cores or threads,

00:03:36.000 --> 00:03:42.000
But it does not scale up past a couple hundred maximum, like, the highest-end CPUs.

00:03:42.000 --> 00:03:48.000
A GP, on the other hand, is structured for pure parallel operations.

00:03:48.000 --> 00:03:54.000
Um, essentially, when you have a GPU, like, the bottleneck is memory access.

00:03:54.000 --> 00:03:58.000
Um, it is made of tens of thousands

00:03:58.000 --> 00:04:00.000
of, like, a small course I can do.

00:04:00.000 --> 00:04:03.000
SIP operations. Um, they're…

00:04:03.000 --> 00:04:05.000
broken up into little blocks, um…

00:04:05.000 --> 00:04:08.000
I know, I, uh…

00:04:08.000 --> 00:04:13.000
NVA, I'll call it, like, a warp. There's, like, um, they basically…

00:04:13.000 --> 00:04:19.000
You'll have a small group of cores that will be operating in lockstep.

00:04:19.000 --> 00:04:22.000
And they can do simple operations, um…

00:04:22.000 --> 00:04:29.000
You know, they can't do super complex things. They don't have the fancy instructions like a…

00:04:29.000 --> 00:04:32.000
a CPU does. It's a lot more simple.

00:04:32.000 --> 00:04:35.000
But their strength comes in their numbers.

00:04:35.000 --> 00:04:38.000
And the fact that they can process

00:04:38.000 --> 00:04:40.000
gigabytes of information.

00:04:40.000 --> 00:04:46.000
so much faster than a CPU that has to go through and iterate through the entire…

00:04:46.000 --> 00:04:52.000
So… now that I've gotten out of the way, why is using a GPU so hard?

00:04:52.000 --> 00:04:58.000
Because CPUs are pretty standard at this point, you know? You can easily make a program that runs on

00:04:58.000 --> 00:05:03.000
And it compiles down to any CPU, you know, ARM, or…

00:05:03.000 --> 00:05:06.000
X86, even RISC-V if you want to do that.

00:05:06.000 --> 00:05:10.000
Um, but GPUs, there's not really a…

00:05:10.000 --> 00:05:13.000
super great all the time, like…

00:05:13.000 --> 00:05:15.000
unified API.

00:05:15.000 --> 00:05:21.000
You have, I mean, the part of history, you have, like, multiple manufacturers, like NVIDIA, AMD, Intel are the main ones.

00:05:21.000 --> 00:05:26.000
that kind of have their own competing interests, so they often will want to sell their own…

00:05:26.000 --> 00:05:29.000
proprietary APIs, we'll get into that a bit.

00:05:29.000 --> 00:05:34.000
You also have the issue of platforms having their own APIs. You have…

00:05:34.000 --> 00:05:36.000
graphics APIs of DirectX,

00:05:36.000 --> 00:05:43.000
Vulcan, Metal, OpenGL, Direct Access Windows only, um, Vulkan is…

00:05:43.000 --> 00:05:47.000
Windows, Linux, Android, everywhere except for Apple.

00:05:47.000 --> 00:05:50.000
Metal is ONLY on Apple devices.

00:05:50.000 --> 00:05:55.000
And OpenGL is technically universal, but it is

00:05:55.000 --> 00:05:57.000
Uh, quite outdated at this point.

00:05:57.000 --> 00:06:01.000
Um, I know on macOS, it's deprecated as far as I know.

00:06:01.000 --> 00:06:07.000
Um, and most things that use OpenGL are shifting away from it these days.

00:06:07.000 --> 00:06:10.000
Um, for doing compute on the GPU.

00:06:10.000 --> 00:06:15.000
there are different competing APIs. There's CUDA, which is NVIDIA's.

00:06:15.000 --> 00:06:18.000
Um, there's Rock'em, which is AMD's.

00:06:18.000 --> 00:06:21.000
Uh, one API, I believe, is Intel's.

00:06:21.000 --> 00:06:26.000
And OpenCL is an open one, but is…

00:06:26.000 --> 00:06:30.000
not the best, it's kind of, like, the so-so in terms of, like, sport and…

00:06:30.000 --> 00:06:33.000
performance. And even on the web!

00:06:33.000 --> 00:06:40.000
Um, of course, we have two competing APIs now, like, well, really it's more one's playing the other.

00:06:40.000 --> 00:06:43.000
Um, we have WebGL, which is the older standard, and

00:06:43.000 --> 00:06:46.000
Hi, sorry, lights turned off. Um…

00:06:46.000 --> 00:06:51.000
There we go. And we have WebGPU, which is the newer standard.

00:06:51.000 --> 00:06:55.000
Um, which will actually become quite relevant, because…

00:06:55.000 --> 00:06:57.000
WGPU is…

00:06:57.000 --> 00:07:00.000
a cross-platform RESH GPU library.

00:07:00.000 --> 00:07:04.000
kind of helps hide all of that complexity we talked about.

00:07:04.000 --> 00:07:08.000
And it implements the WebGPU standard that I just talked about.

00:07:08.000 --> 00:07:13.000
Um, which is, like, the newest standard in cross-platform GPU.

00:07:13.000 --> 00:07:16.000
usage. Um…

00:07:16.000 --> 00:07:19.000
There's none other standard is honestly pretty close to Vulkan.

00:07:19.000 --> 00:07:22.000
Vulcan is, again, it's basically the…

00:07:22.000 --> 00:07:26.000
cross-platform, except for Apple, um, it's a newer…

00:07:26.000 --> 00:07:29.000
APIs, and…

00:07:29.000 --> 00:07:33.000
compared to OpenGL, which was quite high level,

00:07:33.000 --> 00:07:35.000
And you kind of, like, you described…

00:07:35.000 --> 00:07:39.000
a little more like, you know, what you wanted, and it did a lot of magic for you behind the scenes.

00:07:39.000 --> 00:07:45.000
Vulcan and WebGPU are a lot more… you have to actually specify what you want. You have to…

00:07:45.000 --> 00:07:48.000
get quite low level in terms of describing

00:07:48.000 --> 00:07:53.000
Um, if you haven't have the device usage and the pipelines and how you render.

00:07:53.000 --> 00:07:56.000
And, um, this…

00:07:56.000 --> 00:07:59.000
does kind of, you know, like a bit of a developer cost at first.

00:07:59.000 --> 00:08:05.000
But it allows you to optimize and have a lot finer… more finer-grained control.

00:08:05.000 --> 00:08:09.000
over the GPU than something that OpenGL does.

00:08:09.000 --> 00:08:15.000
Um, it has its own trading language, which is the WGSL shading language.

00:08:15.000 --> 00:08:17.000
Um, of course, uh…

00:08:17.000 --> 00:08:20.000
It's similar, actually, to Rust in terms of…

00:08:20.000 --> 00:08:25.000
syntax and, um, bit of a design of it.

00:08:25.000 --> 00:08:28.000
you know, we'll walk in it a bit later.

00:08:28.000 --> 00:08:32.000
And it's not to be confused with WebGL or GLSL, which is the…

00:08:32.000 --> 00:08:36.000
WebGL shading language, um, even though it's…

00:08:36.000 --> 00:08:42.000
very, very close to those names. That's because promos are bad at names. Um, just a general rule.

00:08:42.000 --> 00:08:46.000
Um, and…

00:08:46.000 --> 00:08:52.000
WGPU, um, will… so yeah, it wants WebGPU standard, you know, it takes in…

00:08:52.000 --> 00:08:56.000
these, like, WGSL shaders and all that, and describing pipelines.

00:08:56.000 --> 00:09:02.000
And what it does is that it will take those shaders and all the interruptions you get, and it will compile down

00:09:02.000 --> 00:09:04.000
to the native API,

00:09:04.000 --> 00:09:08.000
at runtime. So, this kind of depends on what platform you're on.

00:09:08.000 --> 00:09:11.000
If you are on…

00:09:11.000 --> 00:09:14.000
on Wix, it'll compile down to Vulkan.

00:09:14.000 --> 00:09:17.000
If you are, um, on the web,

00:09:17.000 --> 00:09:25.000
it might compile down to either WebVPU itself, or it'll compile down to

00:09:25.000 --> 00:09:27.000
um, WebGL.

00:09:27.000 --> 00:09:29.000
Uh, if you're on macOS, it'll compile it in a model.

00:09:29.000 --> 00:09:33.000
Um, but it kind of… it's a very nice layer that…

00:09:33.000 --> 00:09:39.000
hides all of that incompatibility and different seating platforms for you.

00:09:39.000 --> 00:09:42.000
Well, saving you a lot of control and a lot of performance.

00:09:42.000 --> 00:09:45.000
Um, because as I said, it's very low level. You can describe…

00:09:45.000 --> 00:09:48.000
very close to… to…

00:09:48.000 --> 00:09:52.000
the actual APIs.

00:09:52.000 --> 00:09:54.000
So,

00:09:54.000 --> 00:09:58.000
How do we use WGPU?

00:09:58.000 --> 00:10:02.000
That's a little complicated, because getting to a triangle is hard.

00:10:02.000 --> 00:10:05.000
Here is the actual code.

00:10:05.000 --> 00:10:08.000
to get to rendering a triangle.

00:10:08.000 --> 00:10:13.000
NWGPU. It is 354 lines long.

00:10:13.000 --> 00:10:19.000
We're gonna look at this more after I explain some stuff, um, and we're not gonna go look at every single line in detail.

00:10:19.000 --> 00:10:23.000
Um, but…

00:10:23.000 --> 00:10:31.000
Suffice to say, as I said, it's low level, very close to Vulcan, that means you gotta do a lot of setup and a lot of configuration of how you get things running before you can get to…

00:10:31.000 --> 00:10:34.000
things like drawing a triangle.

00:10:34.000 --> 00:10:38.000
So, first, let's look at a more simple example.

00:10:38.000 --> 00:10:46.000
Um, is it a simple pipeline? Um, I believe this is a… just a simple compute pipeline, so there's not any rendering.

00:10:46.000 --> 00:10:49.000
It's just doing an actual computation.

00:10:49.000 --> 00:10:51.000
on the GPU, and…

00:10:51.000 --> 00:10:54.000
Um, uh, returning as a result.

00:10:54.000 --> 00:10:57.000
So, we're gonna do a little lawsuit with code here.

00:10:57.000 --> 00:11:03.000
Um, as you see, WGPU, you get an instance of it.

00:11:03.000 --> 00:11:05.000
And then you request an adapter.

00:11:05.000 --> 00:11:13.000
Um, which I believe the adapter that actually adapts it to, um, your local light, like, your native…

00:11:13.000 --> 00:11:15.000
API.

00:11:15.000 --> 00:11:20.000
And then… you get from that adapter, you request a device.

00:11:20.000 --> 00:11:26.000
Um, a… that means you're actually requesting the actual GPU that you want to use for this computation.

00:11:26.000 --> 00:11:33.000
Um, you can specify some options here, we'll see that later, um, to kind of say, oh, I need something that can do

00:11:33.000 --> 00:11:37.000
this level of competition that has this feature and that feature, um…

00:11:37.000 --> 00:11:41.000
But here, I think we just do a default, because it'd be a simple device.

00:11:41.000 --> 00:11:45.000
Um, and you get the device, uh…

00:11:45.000 --> 00:11:49.000
shop itself, and also a queue that you can queue operations on.

00:11:49.000 --> 00:11:51.000
Um…

00:11:51.000 --> 00:11:54.000
So then here, we…

00:11:54.000 --> 00:11:58.000
reload the shader, um, we'll look at the shader at the next slide.

00:11:58.000 --> 00:12:00.000
Um, I guess it's a little compute shader.

00:12:00.000 --> 00:12:04.000
Um, and it's called shaders because I think it kind of came from

00:12:04.000 --> 00:12:07.000
the graphics wrong, where, you know, you actually are…

00:12:07.000 --> 00:12:09.000
shading pixels and stuff, but…

00:12:09.000 --> 00:12:13.000
You can, these days, and in WebGPU,

00:12:13.000 --> 00:12:16.000
You can do what is called a compute shader.

00:12:16.000 --> 00:12:21.000
Which does something quite similar to things like CUDA, where you have a block model,

00:12:21.000 --> 00:12:27.000
And you can do arbitrary computations on a buffer.

00:12:27.000 --> 00:12:30.000
So…

00:12:30.000 --> 00:12:32.000
Sorry, scrolling a little bit ago. Um, next…

00:12:32.000 --> 00:12:35.000
Uh, we actually make the compute pipeline.

00:12:35.000 --> 00:12:38.000
It's basically describing what the actual, um…

00:12:38.000 --> 00:12:40.000
opportunity we want to do, yeah, we attach…

00:12:40.000 --> 00:12:47.000
Um, things to where we had to have buffers later, and we attach…

00:12:47.000 --> 00:12:51.000
the actual shader you want to use. As you see here, it has a module shader.

00:12:51.000 --> 00:12:55.000
giving it a label and some other default options.

00:12:55.000 --> 00:12:58.000
Um, here, we just make a buffer of…

00:12:58.000 --> 00:13:01.000
10,000 elements, um…

00:13:01.000 --> 00:13:06.000
We then actually make a buffer on the device.

00:13:06.000 --> 00:13:10.000
Where we take this data and put it onto the actual GPU.

00:13:10.000 --> 00:13:13.000
Um, you can see here it's a bite muck.

00:13:13.000 --> 00:13:16.000
Uh, which basically does some, uh, safe…

00:13:16.000 --> 00:13:20.000
casting, uh, to actually give it the raw data.

00:13:20.000 --> 00:13:24.000
Um, we then create an output buffer.

00:13:24.000 --> 00:13:26.000
um… that.

00:13:26.000 --> 00:13:29.000
Basically, the actual result of the computation will go.

00:13:29.000 --> 00:13:31.000
Um, and…

00:13:31.000 --> 00:13:35.000
Yes, you can see here it's a…

00:13:35.000 --> 00:13:37.000
Yeah, copy source.

00:13:37.000 --> 00:13:40.000
Yes, yes, yes. So, I might have mentioned that. Yes, the copy source designation.

00:13:40.000 --> 00:13:48.000
Um, you're basically telling it which way the data's gonna flow, um, so copy destination here for the, for the input means that we're going to

00:13:48.000 --> 00:13:51.000
copy from the host to the device.

00:13:51.000 --> 00:13:55.000
And then this one as a source, means we'll copy from the device to the host.

00:13:55.000 --> 00:13:59.000
Um, and yes, arbitrary storage, so it's called storage.

00:13:59.000 --> 00:14:04.000
Um, there's also a temporary buffer that we use for the…

00:14:04.000 --> 00:14:06.000
computation, I believe, and…

00:14:06.000 --> 00:14:09.000
that is, um…

00:14:09.000 --> 00:14:12.000
Okay, well, copy to it, I guess, and yeah, it's a…

00:14:12.000 --> 00:14:15.000
thing they'll… they'll read from.

00:14:15.000 --> 00:14:16.000
Um…

00:14:16.000 --> 00:14:19.000
Then, we create the buying group. Buying group…

00:14:19.000 --> 00:14:23.000
It's a whole thing where you kind of describe how you actually want to lay out the memory.

00:14:23.000 --> 00:14:27.000
And so that the shader can properly access the memory.

00:14:27.000 --> 00:14:31.000
Um, so I use buffers, you attach it to the buying group and say, okay,

00:14:31.000 --> 00:14:35.000
So, the layout is this pipeline from the shader up.

00:14:35.000 --> 00:14:38.000
Um, and here's the entries.

00:14:38.000 --> 00:14:43.000
So, the… the first buy, you know, index zero,

00:14:43.000 --> 00:14:48.000
the input buffer, the second bind will be the output buffer,

00:14:48.000 --> 00:14:51.000
And then… here we go, um…

00:14:51.000 --> 00:14:54.000
Yes, now we actually will…

00:14:54.000 --> 00:14:58.000
encode the command to the device.

00:14:58.000 --> 00:15:04.000
Um, as I talked about a little bit earlier, how it kind of shares the similar model to

00:15:04.000 --> 00:15:07.000
kudos, um…

00:15:07.000 --> 00:15:10.000
warps, blocks, threads, um…

00:15:10.000 --> 00:15:12.000
So, uh…

00:15:12.000 --> 00:15:16.000
WVU has a thing called Workgroups.

00:15:16.000 --> 00:15:22.000
And that's basically the amount of threads that are in a kind of lockstep group, conceptually. Um…

00:15:22.000 --> 00:15:28.000
So, we have to specify how many, um, threads per workgroup we want.

00:15:28.000 --> 00:15:31.000
Alright, sorry. We already have to buy that.

00:15:31.000 --> 00:15:34.000
Where was that? I think it was earlier on.

00:15:34.000 --> 00:15:36.000
I might have missed that. Um…

00:15:36.000 --> 00:15:39.000
But we have to actually calculate how many groups we want.

00:15:39.000 --> 00:15:42.000
of 64 threads, so we need to, uh…

00:15:42.000 --> 00:15:46.000
ceiling division, and then we tell it.

00:15:46.000 --> 00:15:49.000
the pipeline, the buying group that goes with it,

00:15:49.000 --> 00:15:52.000
And we need to dispatch the work groups. We tell it, okay.

00:15:52.000 --> 00:15:55.000
I want this many groups of 64 threads.

00:15:55.000 --> 00:16:01.000
Um, these ones, actually, because you can technically do up to, like, 3 dimensions of threads conceptually.

00:16:01.000 --> 00:16:03.000
professionally?

00:16:03.000 --> 00:16:05.000
or I was doing a one-dimensional computation.

00:16:05.000 --> 00:16:08.000
So, we tell it, I just want this amount of groups.

00:16:08.000 --> 00:16:11.000
And then we will…

00:16:11.000 --> 00:16:13.000
copy, buffer to buffer.

00:16:13.000 --> 00:16:16.000
Um…

00:16:16.000 --> 00:16:20.000
Let's see… yes, we are copying the output.

00:16:20.000 --> 00:16:25.000
for the exact arguments here for us to call being a buffer, um, and then we're copying a defense buffer.

00:16:25.000 --> 00:16:27.000
Then we submit it, and…

00:16:27.000 --> 00:16:32.000
And the… there's actually runs the full computation.

00:16:32.000 --> 00:16:37.000
Um, we now need to actually… this is some…

00:16:37.000 --> 00:16:42.000
This is some funky actual map process that we have to, um, map the grizzle, um…

00:16:42.000 --> 00:16:47.000
to our device, I think. Um, this is…

00:16:47.000 --> 00:16:53.000
Yeah, so this account you run on the web, um, we can't really use, like, a nice, like, thread or anything here, we actually have to do it

00:16:53.000 --> 00:16:55.000
single-dotted mode, so it does some…

00:16:55.000 --> 00:16:57.000
wait indefinitely pulling.

00:16:57.000 --> 00:17:00.000
Um, this blocks the thread. Um…

00:17:00.000 --> 00:17:03.000
And then we actually get the data.

00:17:03.000 --> 00:17:07.000
Um, and then we can…

00:17:07.000 --> 00:17:11.000
get the data, copy the output data from the…

00:17:11.000 --> 00:17:14.000
buffer, and then, because they're equal to that, um…

00:17:14.000 --> 00:17:19.000
Yeah, I think it's, yeah, it's just… I think Ashley basically had no offer.

00:17:19.000 --> 00:17:22.000
But we assert that the input was copied to the output.

00:17:22.000 --> 00:17:25.000
Um, and…

00:17:25.000 --> 00:17:31.000
Anyway, you unmap it, and there you go! You've successfully done one computation.

00:17:31.000 --> 00:17:35.000
It's a good, fun, 168 lines.

00:17:35.000 --> 00:17:40.000
Um, so yeah, here's the actual, uh, shader we're looking at a compute shader.

00:17:40.000 --> 00:17:44.000
funnily enough, this is actually more complex than the triangle shader, because…

00:17:44.000 --> 00:17:49.000
this deals with some, like, basically, yeah, dimensions of, like, your actual work groups and threads.

00:17:49.000 --> 00:17:53.000
Um, it seems kind of a lot of kind of, like, annotation-based.

00:17:53.000 --> 00:17:55.000
Um, here you annotate your bindings.

00:17:55.000 --> 00:17:59.000
like, the Group 0, binding 0 as the input.

00:17:59.000 --> 00:18:01.000
Group VR binding 1 is the output.

00:18:01.000 --> 00:18:03.000
It's an array of unsigned ints.

00:18:03.000 --> 00:18:06.000
Um…

00:18:06.000 --> 00:18:10.000
You then annotate the main function here, saying it's a compute function.

00:18:10.000 --> 00:18:14.000
And yes, here it is, yes, the workgroup size is defined here.

00:18:14.000 --> 00:18:19.000
Um, 64. So, that means there's 64 threads in that workgroup.

00:18:19.000 --> 00:18:22.000
And then you have the function main.

00:18:22.000 --> 00:18:25.000
It's built in for the actual, um, the…

00:18:25.000 --> 00:18:32.000
global invocation ID is basically telling it its dimension in this, um…

00:18:32.000 --> 00:18:35.000
Like, if you are… if you were, like, the…

00:18:35.000 --> 00:18:39.000
The third… no. As I do this, the second work group…

00:18:39.000 --> 00:18:44.000
And the first one in that, you would be, like, 129, because that would be 64…

00:18:44.000 --> 00:18:46.000
times 2 plus 1.

00:18:46.000 --> 00:18:49.000
it's basically a whole dimension-based, um…

00:18:49.000 --> 00:18:54.000
indexing. You can look into that more if you want on your own.

00:18:54.000 --> 00:18:57.000
it's kind of someone recruit if you've done before.

00:18:57.000 --> 00:19:00.000
Um, then you…

00:19:00.000 --> 00:19:06.000
Simply, yeah, this just copies it from the input to the output. It's a very simple nob.

00:19:06.000 --> 00:19:12.000
And… yeah, that is… that is how you do a computation. You can tell it either index…

00:19:12.000 --> 00:19:16.000
Um, you guys are… there's an array length function, that tells me how long… so that makes sure you're…

00:19:16.000 --> 00:19:20.000
type-safe and don't, uh, accidentally do an out-of-bounds write.

00:19:20.000 --> 00:19:23.000
And then…

00:19:23.000 --> 00:19:26.000
Yeah. So…

00:19:26.000 --> 00:19:30.000
That's a compute, a very… a quote-unquote simple compute shader.

00:19:30.000 --> 00:19:36.000
But if you want to do some GPU stuff, you probably also want to do some rendering stuff.

00:19:36.000 --> 00:19:38.000
I mean, not always, of course, but…

00:19:38.000 --> 00:19:44.000
a very, very, um, good run of GPUs is obviously when doing graphics.

00:19:44.000 --> 00:19:46.000
So, how do you do that?

00:19:46.000 --> 00:19:52.000
Um, we have to get a window first, and for here, we actually use a crate call to win it.

00:19:52.000 --> 00:20:00.000
Um, I just… this is, like, they're actually their icon on GitHub, like, their, like, organization icon. It's a stock photo, just as a fair…

00:20:00.000 --> 00:20:02.000
edited in there. It's great.

00:20:02.000 --> 00:20:07.000
Um, but when it is kind of like WGP for making windows. It abstracts over…

00:20:07.000 --> 00:20:12.000
all of, like, the platforms and the API for making and managing a window.

00:20:12.000 --> 00:20:16.000
Um, and it even works in the web!

00:20:16.000 --> 00:20:20.000
So, like, basically, you know, um, render a window in a canvas element.

00:20:20.000 --> 00:20:23.000
Um, and…

00:20:23.000 --> 00:20:29.000
Also, if you're on Linux, it'll make a GTK window, I believe, if you're on…

00:20:29.000 --> 00:20:32.000
Windows, or we use a 1.32 API on Mac OS.

00:20:32.000 --> 00:20:35.000
etc., etc. Even on Android, you can do that too.

00:20:35.000 --> 00:20:37.000
Um, so…

00:20:37.000 --> 00:20:43.000
Now we can take a look at that fun 300-something lines of code earlier.

00:20:43.000 --> 00:20:48.000
And… it goes through a lot, because we have to make a window…

00:20:48.000 --> 00:20:51.000
have an event loop for the window, and render to it.

00:20:51.000 --> 00:20:56.000
Again, I'm probably not gonna get into as much detail as I did earlier, because it's just aligned.

00:20:56.000 --> 00:20:59.000
I think I have 10 minutes left right now. Um…

00:20:59.000 --> 00:21:06.000
So, we'll go a little bit faster, but we have some, um, configuration target stuff for combined WebAssembly.

00:21:06.000 --> 00:21:08.000
get to that in a bit. Um…

00:21:08.000 --> 00:21:11.000
Here, we actually store our entire estate.

00:21:11.000 --> 00:21:13.000
of what we're doing, um…

00:21:13.000 --> 00:21:16.000
the actual, like, instance of WHVU.

00:21:16.000 --> 00:21:20.000
our window that we're using, device, queue…

00:21:20.000 --> 00:21:24.000
The surfix reminders you, configuration of that surface.

00:21:24.000 --> 00:21:26.000
the pipeline.

00:21:26.000 --> 00:21:30.000
Um, let's see…

00:21:30.000 --> 00:21:32.000
Um, so…

00:21:32.000 --> 00:21:34.000
We make our app.

00:21:34.000 --> 00:21:37.000
And we essentially…

00:21:37.000 --> 00:21:40.000
Let's see, yes. We…

00:21:40.000 --> 00:21:43.000
make a new little app with an event loop.

00:21:43.000 --> 00:21:49.000
Um, and has some things for, like, resizing the surface when the window size changes.

00:21:49.000 --> 00:21:57.000
Um, it's, again, there's not a lot to this.

00:21:57.000 --> 00:21:58.000
Sorry, scrolling over, annoying. Um…

00:21:58.000 --> 00:22:01.000
Oh, look at that, macOS thing, because apparently, uh…

00:22:01.000 --> 00:22:07.000
macOS has to manually draw, so, you know, abstracts cover most things, but some things might be over…

00:22:07.000 --> 00:22:11.000
Because, of course, it is no abstraction is perfect. They're often leaky.

00:22:11.000 --> 00:22:15.000
Um… here is…

00:22:15.000 --> 00:22:21.000
some stuff for, like, when basically the app is resumed, like, I typically was minimized, or then unminimized.

00:22:21.000 --> 00:22:24.000
Um…

00:22:24.000 --> 00:22:26.000
this has some fun…

00:22:26.000 --> 00:22:28.000
JavaScript interop,

00:22:28.000 --> 00:22:32.000
Again, lots of… actually, handling the window.

00:22:32.000 --> 00:22:35.000
Um, I'm not gonna explain it all, honestly, it's been a

00:22:35.000 --> 00:22:37.000
The purpose of the idea of this, and…

00:22:37.000 --> 00:22:43.000
I just know that. This stuff. Basically, we'll get into working and working in the way that you expect a window to, you know.

00:22:43.000 --> 00:22:47.000
When you resize things, when you render, it should actually…

00:22:47.000 --> 00:22:49.000
work as you expect. Um…

00:22:49.000 --> 00:22:51.000
Okay. Here we go.

00:22:51.000 --> 00:22:54.000
Ha! Here it is. So this is actually the part where…

00:22:54.000 --> 00:22:58.000
We, um, we're getting some instance and service and adapter stuff.

00:22:58.000 --> 00:23:00.000
So, the instance, similar to earlier,

00:23:00.000 --> 00:23:04.000
Um, we get a lot of WGP, for instance, but this one, we actually give it…

00:23:04.000 --> 00:23:08.000
a, um, display handle.

00:23:08.000 --> 00:23:10.000
Because we want to actually get a display.

00:23:10.000 --> 00:23:13.000
And so then we get a surface…

00:23:13.000 --> 00:23:17.000
from the instance, officially to get it from the window.

00:23:17.000 --> 00:23:20.000
And we request an adapter from the instance.

00:23:20.000 --> 00:23:22.000
And here…

00:23:22.000 --> 00:23:24.000
Yep, we have, like, some things of saying, okay.

00:23:24.000 --> 00:23:31.000
I want something that is, like, default power preference, and you can specify a higher or a lower if you want to do power saving.

00:23:31.000 --> 00:23:35.000
um… giving you a surface…

00:23:35.000 --> 00:23:37.000
Then you get the descriptor,

00:23:37.000 --> 00:23:41.000
Which… or device, I mean, you can imagine descriptor that you're saying,

00:23:41.000 --> 00:23:44.000
I need certain features.

00:23:44.000 --> 00:23:51.000
So this one, like, it doesn't require any special features here, but it has some limits, where you want to…

00:23:51.000 --> 00:23:53.000
This requirement says that

00:23:53.000 --> 00:23:59.000
you want to be able to basically have the same limits as the WebGL2 standard.

00:23:59.000 --> 00:24:03.000
Um, and also with the resolution of…

00:24:03.000 --> 00:24:07.000
the adapter. It'll be your surface resolution.

00:24:07.000 --> 00:24:11.000
Um, I actually got bit by this, I think, because I forgot to add this once.

00:24:11.000 --> 00:24:14.000
And, um…

00:24:14.000 --> 00:24:21.000
my app would break, because if I made my window too large, it would be like, oh, I can't do anything over 2048.

00:24:21.000 --> 00:24:26.000
And that's because when I did WebGL default is 2048 for the maximum texture size.

00:24:26.000 --> 00:24:30.000
Um, so you have to tell it that you have to have a resolution, too.

00:24:30.000 --> 00:24:32.000
Um…

00:24:32.000 --> 00:24:34.000
Again, a couple other options you pass here.

00:24:34.000 --> 00:24:36.000
And then, you need a device…

00:24:36.000 --> 00:24:39.000
Similar to earlier, you give it a shader, um…

00:24:39.000 --> 00:24:44.000
Which we'll look at, uh, down here. Um, we'll get to that later.

00:24:44.000 --> 00:24:49.000
discovered a pipeline, just like earlier. Um, there's no buying groups here. We're not actually passing memory.

00:24:49.000 --> 00:24:52.000
to and from the device. We're doing a simple shader that renders

00:24:52.000 --> 00:24:55.000
Directly. Um, there's no…

00:24:55.000 --> 00:24:58.000
memory sharing going on here, um, which actually makes things a little simpler.

00:24:58.000 --> 00:25:00.000
Um…

00:25:00.000 --> 00:25:04.000
There is, okay, a swap chain is for… sorry.

00:25:04.000 --> 00:25:09.000
A swap chain, I believe it's for actually doing some rendering, basically to go swap buffers back and forth to…

00:25:09.000 --> 00:25:13.000
Um, ensure that you're not writing to the buffer that's currently being rendered.

00:25:13.000 --> 00:25:15.000
Um, like, someone like VSync and stuff.

00:25:15.000 --> 00:25:19.000
Um… here, render pipeline…

00:25:19.000 --> 00:25:23.000
similar earlier, right, via compute pipeline earlier, right now, we have a render pipeline.

00:25:23.000 --> 00:25:26.000
Um…

00:25:26.000 --> 00:25:29.000
We are giving it a layout of the pipeline, we have…

00:25:29.000 --> 00:25:38.000
vertexes, um, vertex and fragment shaders, um, inside of our shader, we'll look at that in a second.

00:25:38.000 --> 00:25:42.000
And that basically will actually render things.

00:25:42.000 --> 00:25:45.000
get the surface configuration,

00:25:45.000 --> 00:25:48.000
configure it with the device and the…

00:25:48.000 --> 00:25:52.000
computers we want to, and then we send the event to actually…

00:25:52.000 --> 00:25:56.000
um, writer.

00:25:56.000 --> 00:25:59.000
And then, let's see…

00:25:59.000 --> 00:26:04.000
Oh yeah, here is just a thing for handling any user events on the window.

00:26:04.000 --> 00:26:07.000
Um, oh, window, but, um…

00:26:07.000 --> 00:26:11.000
Whenever I actually find the handling things, like, when the wind is resized,

00:26:11.000 --> 00:26:16.000
when there's a redraw request, basically whenever you want to draw a frame.

00:26:16.000 --> 00:26:17.000
Um, that will…

00:26:17.000 --> 00:26:22.000
Okay, here we go, it matches what having on the GPU, if it's had a success, please give it the frame.

00:26:22.000 --> 00:26:25.000
If it's a timeout, we try again later.

00:26:25.000 --> 00:26:32.000
Um, there's a suboptimal issue, we drop it and try again, and then we reconfigure things.

00:26:32.000 --> 00:26:37.000
updated, so I won't let me do that, too.

00:26:37.000 --> 00:26:41.000
Sorry, the scrolling's jumping around a lot here. Um, I…

00:26:41.000 --> 00:26:44.000
Here, um, yeah, things are lost, try again.

00:26:44.000 --> 00:26:48.000
Uh, dental goods, honestly.

00:26:48.000 --> 00:26:51.000
And yeah, here it goes, we have a…

00:26:51.000 --> 00:26:53.000
Texture. We…

00:26:53.000 --> 00:26:57.000
encode the… here we… actually, here's the company, yeah, encode the commands here.

00:26:57.000 --> 00:26:59.000
on the actual, like, render pass, um…

00:26:59.000 --> 00:27:04.000
We encode the command, we begin the render pass.

00:27:04.000 --> 00:27:07.000
attached to some things are going high, like, what…

00:27:07.000 --> 00:27:10.000
how we wanted the colors to work, um…

00:27:10.000 --> 00:27:14.000
Like, you can tell, obviously, you want to actually clear things or not, um…

00:27:14.000 --> 00:27:18.000
like, basically, it's not like you can retain the renderings, you can…

00:27:18.000 --> 00:27:24.000
Um, they're having to draw the entire frame every window, you can just, like, draw whatever new stuff you want.

00:27:24.000 --> 00:27:31.000
It's gonna be useful sometimes, but something also can get weird, like, if you ever played, um…

00:27:31.000 --> 00:27:38.000
any source games, like Portal. If you go out of bounds, you'll get a VR trippy effect where things will start, like, drawing across the screen.

00:27:38.000 --> 00:27:40.000
Because it doesn't actually clear the rendering.

00:27:40.000 --> 00:27:43.000
But here, we're going to clear it with green.

00:27:43.000 --> 00:27:45.000
Research Pipeline.

00:27:45.000 --> 00:27:48.000
And you draw 3 vertices.

00:27:48.000 --> 00:27:52.000
We submit that, and we present the frame.

00:27:52.000 --> 00:27:57.000
And then… here's the actual main, where we actually get our event loop built.

00:27:57.000 --> 00:28:01.000
And we, uh, start our app.

00:28:01.000 --> 00:28:04.000
And there you go, it has basically an infinite-infant loop.

00:28:04.000 --> 00:28:07.000
Um, here's the actual shader down here.

00:28:07.000 --> 00:28:11.000
It's simpler than earlier, have two things, um…

00:28:11.000 --> 00:28:14.000
vertex and Fragment, um…

00:28:14.000 --> 00:28:19.000
the vertex shader takes in the vertexes for the triangle.

00:28:19.000 --> 00:28:21.000
and actually render them.

00:28:21.000 --> 00:28:24.000
or EZ returns the opposition to them.

00:28:24.000 --> 00:28:26.000
Fragment…

00:28:26.000 --> 00:28:31.000
I forget exactly what this does, but it's a simple no-op here, we're not doing anything fancy with it.

00:28:31.000 --> 00:28:36.000
Um, and…

00:28:36.000 --> 00:28:41.000
That's it. Are you ready to see a triangle, guys?

00:28:41.000 --> 00:28:46.000
There we go! That is all of our work, all 37 lines of it.

00:28:46.000 --> 00:28:48.000
to get a red triangle,

00:28:48.000 --> 00:28:54.000
green background. That's it. The fragment, that's it. The fragment is actually… that's red. This color's red.

00:28:54.000 --> 00:28:56.000
So it's coloring the fragment red. That's what that is.

00:28:56.000 --> 00:28:59.000
I never know. Sorry, been a little bit since I did this.

00:28:59.000 --> 00:29:04.000
Um, so that's… that's our triangle. We write it at one triangle.

00:29:04.000 --> 00:29:08.000
So, um, running out of time here, I have, like, a minute left, I think running a little fast.

00:29:08.000 --> 00:29:12.000
Um, if you want to compile the web, there is a tool Wasm pack.

00:29:12.000 --> 00:29:15.000
Um, compiles your cargo project of WebAssembly?

00:29:15.000 --> 00:29:19.000
Um, your product needs to be a library for this, so you have to include this.

00:29:19.000 --> 00:29:21.000
in your cargo tunnel?

00:29:21.000 --> 00:29:27.000
Um, and this will compile… basically, you can run WASMP pack build,

00:29:27.000 --> 00:29:31.000
And it will build it for the web. It'll give you OpAssembly, um…

00:29:31.000 --> 00:29:35.000
artifact. It'll give you the JS bindings for it, too.

00:29:35.000 --> 00:29:38.000
Um, other web perks for assembly.

00:29:38.000 --> 00:29:41.000
Um, you can't have threads on the web, at least not yet.

00:29:41.000 --> 00:29:45.000
Um… WebGPU support is

00:29:45.000 --> 00:29:48.000
pretty, uh, widespread.

00:29:48.000 --> 00:29:54.000
Um, however, not entirely. I know that on Linux, it's still experimental. I had to turn it on.

00:29:54.000 --> 00:29:58.000
for both Firefox and Chrome, and, like, they're… and they're, like…

00:29:58.000 --> 00:30:00.000
hidden settings, basically. Um…

00:30:00.000 --> 00:30:06.000
And on Firefox, it's a little bit buggy. I usually use Firefox, but I'm using Chrome.

00:30:06.000 --> 00:30:10.000
for this presentation, because sometimes Firefox would crash when I reset the window.

00:30:10.000 --> 00:30:13.000
Um,

00:30:13.000 --> 00:30:17.000
They can have quite large binaries. I know that my project, when I do this, um…

00:30:17.000 --> 00:30:20.000
ended up having a binary, I think, like, the WebAssembly binary.

00:30:20.000 --> 00:30:23.000
was, like, I think 5 megabytes?

00:30:23.000 --> 00:30:26.000
Which is a lot for the web. Um…

00:30:26.000 --> 00:30:31.000
And single-page applications can be a little bit painful sometimes. On my website.

00:30:31.000 --> 00:30:36.000
If anyone has a solution to this, you can let me know, um, on my website, I have a… I use Tulk Kit.

00:30:36.000 --> 00:30:39.000
And I have attorney and generation, and we'll look at it in a second.

00:30:39.000 --> 00:30:45.000
Um, and the issue is, if you navigate away from that page, then come back,

00:30:45.000 --> 00:30:51.000
Uh, it fails to render, because it's trying to render on the old canvas,

00:30:51.000 --> 00:30:53.000
And I can't figure out exactly how to terminate it and relaunch it.

00:30:53.000 --> 00:30:57.000
I just added a link there saying, if it fails, reload the page, and it reloads the page.

00:30:57.000 --> 00:31:01.000
Um, so, some quirks with that.

00:31:01.000 --> 00:31:04.000
just generally kind of how JS works.

00:31:04.000 --> 00:31:06.000
But…

00:31:06.000 --> 00:31:09.000
Yeah. Um, more to it, um…

00:31:09.000 --> 00:31:14.000
eagerly, um, is a simple GUI library. It's great for things like graphics demos.

00:31:14.000 --> 00:31:17.000
Um, essentially running on the side here, um…

00:31:17.000 --> 00:31:21.000
This is… this is, like, this website, it's…

00:31:21.000 --> 00:31:23.000
It's… this is, like, running on WebAssembly, it's a Rust…

00:31:23.000 --> 00:31:27.000
library, um, you can integrate it quite nicely with WGPU.

00:31:27.000 --> 00:31:30.000
And… I see, it's… it…

00:31:30.000 --> 00:31:34.000
It works quite nicely, and you can do a lot of nice things.

00:31:34.000 --> 00:31:38.000
For example, here is the terrain generation product I was talking about.

00:31:38.000 --> 00:31:42.000
Um, I made this for a class at Stevens, where I was a student.

00:31:42.000 --> 00:31:45.000
Just a few months ago. Um…

00:31:45.000 --> 00:31:51.000
And it's running… it's WGPU entirely with, uh, eGUI here.

00:31:51.000 --> 00:31:59.000
Um, and you can change the pipelines running here.

00:31:59.000 --> 00:32:01.000
And you can change parameters to change the noise values.

00:32:01.000 --> 00:32:08.000
And, uh, yeah, Eagle is a very nice thing to actually, uh, add the simple UI to you.

00:32:08.000 --> 00:32:13.000
WebGPU project, and just, like, change parameters and stuff, like, I had to wire it all up, but it was…

00:32:13.000 --> 00:32:16.000
Very nice, very convenient. Um…

00:32:16.000 --> 00:32:21.000
So yeah, if there's something you can do with, uh, WebGPU. It's really cool.

00:32:21.000 --> 00:32:24.000
I'm quite proud of this product, personally.

00:32:24.000 --> 00:32:28.000
just a fun rendering project that compute…

00:32:28.000 --> 00:32:31.000
has a couple modes, but this one is, like, just in the vertex, this one…

00:32:31.000 --> 00:32:34.000
computers with, um, a compute group.

00:32:34.000 --> 00:32:39.000
Um, and yeah, that's… that's the project I did.

00:32:39.000 --> 00:32:41.000
So, in summary…

00:32:41.000 --> 00:32:44.000
WGPU is a cross-platform,

00:32:44.000 --> 00:32:46.000
Low-level GPU library for Rust.

00:32:46.000 --> 00:32:48.000
And there was a lot to go through, we went through it.

00:32:48.000 --> 00:32:51.000
Um, quite quickly for some parts.

00:32:51.000 --> 00:32:54.000
Um, but a few resources for you.

00:32:54.000 --> 00:32:59.000
WGPU-RS is the homepage of WGPU.

00:32:59.000 --> 00:33:04.000
Um, it is, like, there's got some good news just there. This is a tutorial.

00:33:04.000 --> 00:33:07.000
um, learn WGPU, a great tutorial that walks you through…

00:33:07.000 --> 00:33:10.000
everything from starting, like, you know, at basic.

00:33:10.000 --> 00:33:18.000
Like, start getting a window up, getting your triangle work, and getting lighting working, even camera movement, all that.

00:33:18.000 --> 00:33:25.000
And then, finally, um, my mom was like, because gotta plug that. Um, and my project is on that website, too.

00:33:25.000 --> 00:33:27.000
Um, so…

00:33:27.000 --> 00:33:31.000
Yeah. That is the talk, so thank you guys!

00:33:31.000 --> 00:33:37.000
For… for listening, and I'll answer any questions, and I saw there's a few chat messages, so…

00:33:37.000 --> 00:33:39.000
We'll go through questions. Thank you.

00:33:39.000 --> 00:33:45.000
Excellent, thank you very much for this presentation. It was really interesting.

00:33:45.000 --> 00:33:50.000
I have a question which is probably, and we can get back to the other questions as well,

00:33:50.000 --> 00:33:53.000
But, uh, obviously. But, um…

00:33:53.000 --> 00:33:59.000
I might be totally misunderstanding things here. So, GPU is basically, originally, was for graphics.

00:33:59.000 --> 00:34:00.000
Yes.

00:34:00.000 --> 00:34:07.000
But these days, these days, it's used a lot for various mathematic operations,

00:34:07.000 --> 00:34:10.000
For the back end of AI or whatnot.

00:34:10.000 --> 00:34:11.000
Right? So…

00:34:11.000 --> 00:34:15.000
Yep, yep, because, yeah, it's because they were designed for graphics, you know, where you have to have a large, like,

00:34:15.000 --> 00:34:16.000
Yeah.

00:34:16.000 --> 00:34:18.000
array of pixels, a bread or two.

00:34:18.000 --> 00:34:22.000
But they realized that in building that, they basically built a general-purpose

00:34:22.000 --> 00:34:27.000
parallel processor, and so they exposed that with different APIs, and now you can just do

00:34:27.000 --> 00:34:31.000
any old computation you want to do on there.

00:34:31.000 --> 00:34:36.000
Yeah, so is this, uh… I don't know, library, create, whatever,

00:34:36.000 --> 00:34:43.000
Is this, uh… would it be good for also to do various non-graphic computations?

00:34:43.000 --> 00:34:44.000
Would that be… make sense?

00:34:44.000 --> 00:34:47.000
Um, yes, yes, you could. As my product here, like, I have, um…

00:34:47.000 --> 00:34:51.000
I get renders it, but it also, like, this is actually a compute.

00:34:51.000 --> 00:34:57.000
shader, so it's doing the computation to run, like, to calculate the terrain height.

00:34:57.000 --> 00:34:58.000
Mm-hmm.

00:34:58.000 --> 00:35:00.000
And then it passes it to a rendering fader.

00:35:00.000 --> 00:35:03.000
Um, so yeah, this is, like, you can very much do, um…

00:35:03.000 --> 00:35:07.000
like, uh, computation on here. It has, you know, it has things like…

00:35:07.000 --> 00:35:10.000
A lot of these concepts similar to things like CUDA, where you have, um…

00:35:10.000 --> 00:35:14.000
you can do some, like, shared memory stuff between work groups.

00:35:14.000 --> 00:35:16.000
You can do some… some caching.

00:35:16.000 --> 00:35:21.000
Um, um, they have, like, the async thread primitive, where you can make it…

00:35:21.000 --> 00:35:25.000
has a lot of those similar types of operations, but you can do it for…

00:35:25.000 --> 00:35:29.000
general-purpose compute. It is very, very capable of that.

00:35:29.000 --> 00:35:34.000
Yeah, so a couple of years ago, I wanted to demonstrate the use of GPU.

00:35:34.000 --> 00:35:38.000
of the advantages of using a GPU for computations,

00:35:38.000 --> 00:35:44.000
And I had some very simple and stupid computation written in Python.

00:35:44.000 --> 00:35:47.000
And it took me, I don't know, a couple of…

00:35:47.000 --> 00:35:52.000
Quite a few hours till I managed to actually show that using the GPU

00:35:52.000 --> 00:35:59.000
made it faster.

00:35:59.000 --> 00:36:00.000
Yes, that is… yep.

00:36:00.000 --> 00:36:08.000
And, of course, the mistake I had is that I didn't have enough data, or I had, uh… so the copying the data, and it didn't have enough computation, so the use of the GPU didn't…

00:36:08.000 --> 00:36:11.000
The cost of using the GPU didn't, uh…

00:36:11.000 --> 00:36:14.000
Um… yeah.

00:36:14.000 --> 00:36:16.000
doesn't force it.

00:36:16.000 --> 00:36:17.000
Yes.

00:36:17.000 --> 00:36:20.000
So I think it would be really nice to see an example where

00:36:20.000 --> 00:36:21.000
But you…

00:36:21.000 --> 00:36:22.000
Yes, actually, I can do it here. I have that here. Um…

00:36:22.000 --> 00:36:24.000
So, if I do a really small one…

00:36:24.000 --> 00:36:29.000
I think, like, yeah, the CPU is faster.

00:36:29.000 --> 00:36:32.000
But, if I do a large one…

00:36:32.000 --> 00:36:35.000
you can see a difference comes in quite quickly.

00:36:35.000 --> 00:36:39.000
Um… you can see, like, I think this, like, a…

00:36:39.000 --> 00:36:41.000
I think that's a little bit, um…

00:36:41.000 --> 00:36:45.000
Oh, you shoulder the… you run it both on CPU and the GPU, and then you show the differences here. Oh, I see that.

00:36:45.000 --> 00:36:49.000
Yes, yes, here, yes, sorry, yes, this is, yeah, this is a GP versus CPU computation.

00:36:49.000 --> 00:36:50.000
Okay, I missed that earlier, yeah, okay.

00:36:50.000 --> 00:36:55.000
Um, yeah, sorry, you can see here, um, so this compa… it gives us a compute pipeline.

00:36:55.000 --> 00:36:58.000
Oh, come on, that light's turned off again. Um…

00:36:58.000 --> 00:37:02.000
the GPU, um, it says here it renders in 80 milliseconds… oh, it's 7 milliseconds.

00:37:02.000 --> 00:37:03.000
Make some sudden movements, you know? That's how you make the light… turn the light on.

00:37:03.000 --> 00:37:10.000
Whereas CPU takes almost a full second to actually calculate it.

00:37:10.000 --> 00:37:11.000
Okay.

00:37:11.000 --> 00:37:14.000
So, yeah, basically, the more I can add more layers, probably, and it would…

00:37:14.000 --> 00:37:18.000
probably gonna… oh yeah, it has taken a while. Ooh, it's freezing.

00:37:18.000 --> 00:37:21.000
I maybe shouldn't have done that. Um…

00:37:21.000 --> 00:37:25.000
There I go. 5 seconds on the CPU, 78 mils on the GPU,

00:37:25.000 --> 00:37:30.000
And this is running in the browser, too. There's not even a native one. If I was running native, it can go even faster.

00:37:30.000 --> 00:37:32.000
Um…

00:37:32.000 --> 00:37:35.000
Mm-hmm.

00:37:35.000 --> 00:37:38.000
Excellent, thank you. Let's go to the chat.

00:37:38.000 --> 00:37:40.000
And, uh, would you want to read, or should I read?

00:37:40.000 --> 00:37:43.000
I can open chat here, there we go.

00:37:43.000 --> 00:37:44.000
Okay.

00:37:44.000 --> 00:37:46.000
Um…

00:37:46.000 --> 00:37:48.000
Okay, come on…

00:37:48.000 --> 00:37:54.000
Oh, sorry, my audio is low, sorry. Um…

00:37:54.000 --> 00:37:56.000
Uh…

00:37:56.000 --> 00:38:00.000
So, scrolling through, uh…

00:38:00.000 --> 00:38:03.000
Are you sure it isn't in Rust?

00:38:03.000 --> 00:38:06.000
running your triangles assembly is longer.

00:38:06.000 --> 00:38:09.000
Yeah, it definitely would be. Um…

00:38:09.000 --> 00:38:15.000
Let's see… how can we experiment on my Mac? Is there any easy way locally to do it?

00:38:15.000 --> 00:38:18.000
Um, so…

00:38:18.000 --> 00:38:21.000
I would say, um…

00:38:21.000 --> 00:38:27.000
And with cargo, I mean, if you've done a rush programming, you can get cargo on your machine, and you can do…

00:38:27.000 --> 00:38:30.000
Um, uh…

00:38:30.000 --> 00:38:35.000
Unfortunately, I don't think there's an easy way to, like, compile Rust in your browser or something, um…

00:38:35.000 --> 00:38:40.000
I was looking at that, actually, because I wanted to see if I could do, like, a cool demo of, like, oh, compiling code in the presentation.

00:38:40.000 --> 00:38:44.000
But, um, can't quite do that. Um…

00:38:44.000 --> 00:38:49.000
Because, you know, doing cargo, you can get WGPU, you can use Winit as well to get it to…

00:38:49.000 --> 00:38:52.000
render a native window, and…

00:38:52.000 --> 00:38:57.000
And you're off to the races. Um…

00:38:57.000 --> 00:39:00.000
Let's see, let me draw the rest of the owl… yeah, it very feels like that.

00:39:00.000 --> 00:39:04.000
Or I kind of went, oh, here's a triangle, and then go try to draw in directly.

00:39:04.000 --> 00:39:08.000
Can WGPU and Rust run as a native app,

00:39:08.000 --> 00:39:10.000
Um, on EG Windows and…

00:39:10.000 --> 00:39:17.000
Um, yeah, so I would say, like, when it will render a native window, um,

00:39:17.000 --> 00:39:20.000
I get renders, um…

00:39:20.000 --> 00:39:22.000
Okay, so yeah, it'll, you know…

00:39:22.000 --> 00:39:24.000
If you do anything like… Sorry.

00:39:24.000 --> 00:39:27.000
It's telling me my words. Um…

00:39:27.000 --> 00:39:29.000
it kind of depends, I guess, like, on, like,

00:39:29.000 --> 00:39:36.000
What do you mean by native? You know, it can… it can create a native window, like, with, like, you know, like, the native windowing API of your OS,

00:39:36.000 --> 00:39:38.000
But, like, if you want to use it, like, in, like, a…

00:39:38.000 --> 00:39:45.000
say, like, a tool calculation, if you want to integrate it with, like, SwiftUI on Mac OS, or, like, the…

00:39:45.000 --> 00:39:47.000
Windows, like, Fluent, or, like,

00:39:47.000 --> 00:39:50.000
Linux GTK.

00:39:50.000 --> 00:39:52.000
Um, I'm not…

00:39:52.000 --> 00:39:57.000
Entirely sure. Like, I think it depends… it depends on, like, what those toolkits have.

00:39:57.000 --> 00:39:58.000
to integrate with them.

00:39:58.000 --> 00:40:01.000
I haven't quite looked into that. I kind of just did a very bare-bones, like…

00:40:01.000 --> 00:40:03.000
like, eagerly application.

00:40:03.000 --> 00:40:05.000
So, um…

00:40:05.000 --> 00:40:11.000
it doesn't really do any, like, native toolkit stuff, it just kind of does its own ego toolkit.

00:40:11.000 --> 00:40:13.000
Um, but…

00:40:13.000 --> 00:40:17.000
I had to take a look around, like, the cargo crates and see, like…

00:40:17.000 --> 00:40:19.000
what's out there. There might be some of them.

00:40:19.000 --> 00:40:23.000
Um, I know there's a lot of useful craziness, I know, like, there's, like,

00:40:23.000 --> 00:40:26.000
WGPU, like, is used for, um…

00:40:26.000 --> 00:40:28.000
the, uh…

00:40:28.000 --> 00:40:30.000
the Z editor.

00:40:30.000 --> 00:40:33.000
Um, they use, like, a… they have, like, a native…

00:40:33.000 --> 00:40:38.000
The whole, like, appeal of Zed is they have, like, their own GPUI framework,

00:40:38.000 --> 00:40:40.000
that renders a UI with a GPU.

00:40:40.000 --> 00:40:43.000
I believe he used a WGPU, and…

00:40:43.000 --> 00:40:47.000
it's not like a native UI, technically, but it's a hardware-accelerated

00:40:47.000 --> 00:40:50.000
GUI.

00:40:50.000 --> 00:40:52.000
that fuel's very snappy.

00:40:52.000 --> 00:40:55.000
Um…

00:40:55.000 --> 00:40:58.000
Yeah, where you ask for devices capabilities will be interesting.

00:40:58.000 --> 00:41:01.000
Do other GPU APIs do the same thing?

00:41:01.000 --> 00:41:05.000
Um, I am not…

00:41:05.000 --> 00:41:07.000
Sure. I…

00:41:07.000 --> 00:41:11.000
because my main experience with other GPU, uh, APIs…

00:41:11.000 --> 00:41:15.000
has been, um…

00:41:15.000 --> 00:41:18.000
with… for the class I took, it was mainly with, like, CUDA.

00:41:18.000 --> 00:41:25.000
And I'm like, I haven't really interacted with many other APIs other than CUDA and, like,

00:41:25.000 --> 00:41:29.000
WGPU. So, I didn't get to look into it yourself, and…

00:41:29.000 --> 00:41:33.000
It would be really nice to have an extraction like this for multi-versioning for, like…

00:41:33.000 --> 00:41:37.000
AVX512 and other, like, parallel operations, and like…

00:41:37.000 --> 00:41:43.000
things that might not be on every device, but you can kind of request a device that has certain capabilities.

00:41:43.000 --> 00:41:46.000
Um, yeah, it is very quirky.

00:41:46.000 --> 00:41:50.000
Um…

00:41:50.000 --> 00:41:54.000
graphics debugger support.

00:41:54.000 --> 00:41:59.000
I am not sure about that either, actually. So, as I said, I'm not actually, like, a deeply, like, expert, I just want, like, a…

00:41:59.000 --> 00:42:01.000
I have not actually…

00:42:01.000 --> 00:42:03.000
tried out a graphic debugger.

00:42:03.000 --> 00:42:09.000
But I believe that, considering this does use, like, you know, native, um…

00:42:09.000 --> 00:42:11.000
GPU, um…

00:42:11.000 --> 00:42:13.000
operations, like, it should be able to, like,

00:42:13.000 --> 00:42:17.000
If a graphic device supports, like, say, like, you know, Vulkan and your…

00:42:17.000 --> 00:42:21.000
You're on, like, run around booking or something, it should be able to hook into there, right? I'd imagine.

00:42:21.000 --> 00:42:29.000
might be a little lower level than, like, the WGPU itself, but…

00:42:29.000 --> 00:42:32.000
Um, there might also be some, like, further, like,

00:42:32.000 --> 00:42:37.000
geography can do. I know, uh, WGPU is, uh…

00:42:37.000 --> 00:42:39.000
quite good in terms of, like, its error-catching, um…

00:42:39.000 --> 00:42:46.000
Like, I know, like, I had an issue or anything, and I accidentally, like, I misconfigured a buffer, and when it tried to load it, I got… it wasn't at compile time,

00:42:46.000 --> 00:42:55.000
Was it runtime, but it's still, like, it didn't, like, do it out of bounds right, it just went like, hey, this binding's wrong, try again. I'm like, oh, okay, got it.

00:42:55.000 --> 00:42:58.000
Then I fixed that. Um…

00:42:58.000 --> 00:43:03.000
So, yeah, I didn't have to use a debugger for what I did, so I'm not sure, but…

00:43:03.000 --> 00:43:05.000
take a look around, I'd say.

00:43:05.000 --> 00:43:07.000
Um, ooh!

00:43:07.000 --> 00:43:10.000
When UI Fluent is in Windows RS now. Okay.

00:43:10.000 --> 00:43:16.000
Um… yeah, as long as I'm looking… looking native, um…

00:43:16.000 --> 00:43:19.000
Yeah, I… I know I've used, um…

00:43:19.000 --> 00:43:23.000
I fell apart. I use, um, the Realm for, um…

00:43:23.000 --> 00:43:26.000
crate to do, like, Linux native, like, like, uh…

00:43:26.000 --> 00:43:29.000
GTK slash, like, little by the way to, um…

00:43:29.000 --> 00:43:36.000
Uh, looking stuff, and I'm not sure how well it integrates with WGPU. I have not tried. They might have a similar, like…

00:43:36.000 --> 00:43:38.000
canvas-like element where you can, like…

00:43:38.000 --> 00:43:41.000
render to it, um…

00:43:41.000 --> 00:43:43.000
But I'm not sure.

00:43:43.000 --> 00:43:53.000
And yeah, eagerly's perfect, little utilities and game UI stuff, yeah.

00:43:53.000 --> 00:43:54.000
Yeah.

00:43:54.000 --> 00:43:57.000
Okay, I think we ran out of questions, right?

00:43:57.000 --> 00:43:59.000
Yeah, I scrolled through the entire chat, as far as I know.

00:43:59.000 --> 00:44:02.000
Yeah, yeah. Okay.

00:44:02.000 --> 00:44:04.000
Speak now and forever hold your peace, you know?

00:44:04.000 --> 00:44:07.000
Okay, uh…

00:44:07.000 --> 00:44:10.000
Well, thank you very much, again.

00:44:10.000 --> 00:44:14.000
to everyone who was here, and uh…

00:44:14.000 --> 00:44:16.000
participated and asked questions.

00:44:16.000 --> 00:44:20.000
Or was it silent and listened in air?

00:44:20.000 --> 00:44:25.000
Uh, and thank you very much, Liam, for giving this presentation and answering all the questions.

00:44:25.000 --> 00:44:26.000
Yeah.

00:44:26.000 --> 00:44:28.000
And thank you, everyone who was watching the video.

00:44:28.000 --> 00:44:32.000
So now is the time to like the video and follow the channel, just to make sure.

00:44:32.000 --> 00:44:33.000
And, um…

00:44:33.000 --> 00:44:35.000
Ring that bell and all that.

00:44:35.000 --> 00:44:47.000
Yeah, yeah, yeah, yeah, yeah, yeah, I have to remember what else I need to take. Yes, those people who are here, you can stay around, and after the video… after I stop the recording, you're welcome to stay, and we can have a free chat.

00:44:47.000 --> 00:44:50.000
Uh… off the record.

00:44:50.000 --> 00:45:00.000
Exactly. Uh, so thank you very much, and see you at some session later on.

