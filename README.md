# PyTarRs

```
pip install pytarrs
```

 Python tar file iterator with tar-rs. Made for one of my other projects, 4x faster than native python. 
 - Multi-threadable (not sendable between threads)
 - No GIL for the actual reading

```python
from pytarrs import PyReader

tar = PyReader('foo.tar')
for x in tar:
    pass
```
