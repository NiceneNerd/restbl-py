# restbl

Library to manipulate the resource size table (RSTB/RESTBL) in *The Legend of
Zelda: Tears of the Kingdom*

## Example Usage

```python
from restbl import ResourceSizeTable

with open("ResourceSizeTable.product.112.rsizetable.zs", "rb") as file:
    table = ResourceSizeTable.from_binary(file.read())
    assert table.get_size("Actor/TwnObj_HatenoObj_A_12.engine__actor__ActorParam.bgyml") == 6184
    table.set_size("Actor/TwnObj_HatenoObj_A_12.engine__actor__ActorParam.bgyml", 666)
    assert table.get_size("Actor/TwnObj_HatenoObj_A_12.engine__actor__ActorParam.bgyml") == 666
```

## License

This project is licensed under the GPLv3+ license.
