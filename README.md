# braindump
Because using email drafts to store text gets old.

## What is it?

Short-term storage for text. That's it. If you've ever found yourself writing a GMail draft to take something down and make it accessible for later -- that's the use case.

## Why? Why not use a note application?

Simple:

  - Something like Evernote is way overkill for my personal needs.
  - Google Keep is free and integrates accross devices but I don't like the interface. Plus it can take a while to load.
  - The amount of features is small enough that it's great as a learning project.

I've set a personal goal for myself to develop a small utility (or utilities) that I personally use and that are perfectly tailored to my personal use cases and/or tastes.
So far I've done this with a simple Window Switcher App Thing, because I hate the Windows taskbar, but it's not yet published. This just addresses another personal use case of mine.

## Why not just keep using GMail drafts?

Because it hides the visibility of real drafts. And why not build something? This is totally 2015 where everyone should code more, yo.

## Technical goals

The following are major goals by which the success of the project will be measured:

  - Speed. Speed speed speed. The page should load as quickly as possible, and capturing text should be as painless as possible.
    The latency from head-to-storage should be minimal. That's it.
  - Zero-friction data capture. No animations. No fancy styling. No bullshit.
  - Iterop. The idea of GMail drafts is that they show up everywhere. This should be the same.
  - Auth. For Your Eyes Only.

And these are some minor goals that will make it a bit better to use:

  - Support some basic formatting (markdown, perhaps?) and automatic URL linking
  - Mobile-specific layout

And some long-term goals:

  - Pure REST API for integration with anything (eg. Android Widget)

## Tech Specs

The idea is to use Rust on the server. I picked the first web framework that I could get compiling - [iron](https://github.com/iron/iron).
I also checked [arewewebyet](http://arewewebyet.com/) and saw that there is a working driver for Postgres, so I'll probably end up using that as a back-end.

For the front-end, I have no idea. I'm not too hot on all the hipster JS frameworks that are available these days. I like the idea of React for dynamic views, so I'll probably go with that just to learn more about it.
As for hitting the back-end, I have no idea. Something simple and rock solid. Backbone?

## Contributing

Is probably not a good idea right now. I don't know how much I'll work on this, and the entire tech stack is new to me, so things are going to break a lot.

## License and Copyright

This work is currently Copyright by me. Don't steal it and sell it. If you want to do something, just open and issue and ask.
