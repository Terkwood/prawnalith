# aquariums in the sky, prawns in heaven, cloud miserliness

By now you've installed docker and insist on running redis
manually. Everything you know is wrong. The world is a beautiful
place and we can all have a bite to eat after a long life
in a pretty aquarium.

But first, defeat the dreaded TRANSPARENT HUGE PAGES warning
on Google Compute Engine when running `redis`:

```
sudo sh -c 'echo never > /sys/kernel/mm/transparent_hugepage/enabled'
```
