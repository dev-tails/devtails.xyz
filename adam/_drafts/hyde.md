Hyde is a replacement for Jekyll built in Rust

Goals:
- Speed
- Learn and teach rust
- Simplify adding static content to static site blog


# Simplify adding static content to static site blog

One of my biggest annoyances with Jekyll is how it handles including images. Web apps have made it extremely simple to drag and drop or paste a new image into a document. With Jekyll, I find myself jumping through so many hoops just to get an image to show up. 

1. Take screenshot
2. Move screenshot to top level `/assets` folder
3. Rename screenshot to something useful
4. Manually type markdown syntax to include this image in post

The image is now highly disconnected from the post it actually pertains to.  If I remove a post, there's a good chance I don't know what files relate to it and just leave them around forever.  The assets folder slowly accumulates until it is more and more difficult to browse and deal with.

What I would instead want is to drop an image file directly into a web text editor and have it store that image file in a folder directly next to the post it will be displayed in. 

```
posts/
  2022-06-17-first-post/
    2022-06-17-first-post.md
    assets/
      img1.png
      img2.png
```

# How it works

Initially the plan would be to have this be as low config as possible.  A person should be able to run `hyde` in any folder and it will interpret the folder structure from there as a hyde project.  The process itself will have an API component that handles making changes to the local file system.  It will then open the browser to the hyde dashboard.  From here a user can Create Post or Browse Existing Posts.

Ideally, a user could create a new draft from a single click of an icon on their phone (or one day just say "create new draft").  That way when someone sparks inspiration on the go, they can quickly get a draft started, without committing to finishing it all.
