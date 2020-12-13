# Decorator File

The decorator file allows you to apply decorations to discoveries. The `defaults` section will apply attributes to the provided decorations if not set.

The `decorations` attribute requires an array of objects with a `matches` and `variables` section.

- `matches` accepts a flat object with the key of the targetted attribute and the value of a regex statement.
- `variables` is a flat object with key pair values.

Supply the path to your decorator file with the `NR_DECORATOR_FILE` environment variable or with the `-d` or `-decorator-file` flag.

```json
{
  "defaults": {
    "guestUser": "123",
    "guestPass": "abc",
    "dcUser": "administrator@vsphere.local",
    "dcPass": "N1mbu55!"
  },
  "decorations": [
    {
      "matches": {
        "vmName": "\\S+"
      },
      "variables": {
        "hello": "world",
        "something": "123",
        "guestUser": "kav",
        "guestPass": "n1mbu5!"
      }
    }
  ]
}
```
