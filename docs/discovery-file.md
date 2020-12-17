# Discovery File

The `defaults` section will add those defaults into each discovery item if not defined.

`discovery_items` is an array of flat json objects that will become accessible via the ${discovery.\<value\>} replacement attributes.

```json
{
  "defaults": {
    "user": "ec2-user",
    "key": "~/.ssh/some-key.pem"
  },
  "discovery_items": [
    {
      "name": "test-ec2",
      "host": "13.239.27.119",
      "team": "batman"
    }
  ]
}
```
