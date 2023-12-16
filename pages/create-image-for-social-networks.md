---
title: Create image for social networks
timestamp: 2023-12-05T11:00:01
published: true
description: The Open Graph pro
tags:
    - Open Graph
    - image
---

If you share any of the articles posted on [DEV.to](https://dev.to/) to Facebook or Twitter they will automatically embed an image in your post.
This is accomplished by having a set of meta-fields in the HTML of each post on DEV.to.

The critical fields are from [The Open Graph protocol](https://ogp.me/):

```
  <meta property="og:image" content="" />
  <meta name="twitter:image:src" content="">
```

The `content` for both of them including URL to an image. That image is what's being shared on the social networks.

Originally I wanted to show how to generate such image using Rust, but in the meantime I already created a project that
will help me, and hopefully others to create such images.

See the [Banner Builder](https://banner-builder.code-maven.com/) project.

## Images provided by other sites.

I've downloaded one of those images from [DEV.to](https://dev.to/) and used the `file` command of Linux on it. This was the result:

```
PNG image data, 1000 x 500, 8-bit colormap, non-interlaced
```

At another site the respective images was

```
PNG image data, 800 x 400, 8-bit/color RGBA, non-interlaced
```

The image of yet another site has this:

```
JPEG image data, JFIF standard 1.01, resolution (DPI), density 120x120, segment length 16, Exif Standard: [TIFF image data, big-endian, direntries=0], baseline, precision 8, 500x300, components 3
```

I realized that the title of the article might bee too long and we might need to [break it into multiple lines](/wrap-text).

We might want to add a small image of the author or the logo of the web site.

We might want to add some other logos. For example Rust if the article is about Rust.


## Conclusion

Try to share this, or any other article from the Rust Maven site on a social network and let me know if you can see the image.

I saw that this works on Facebook, but on LinkedIn only a thumbnail of the image was displayed.

I have not tried it on Reddit or other social networks.


