# myriad formats

This reference documents the three JSON file formats `myriad` reads in one place for multi-agent orchestrators, research harnesses, and external tooling. The specification of record stands in `CONTRACT.md` under the consolidated version 1 formats section (`FIX-MY-15`). Where this reference and `CONTRACT.md` differ, `CONTRACT.md` governs.

Every format below stands at version 1 in `CONTRACT.md` (`FIX-MY-15`). No file carries a `"version"` field or any numeric field at version 1, and every loader ignores unknown extra fields not named by the contract. Under `PORT-3` and `FIX-MY-15`, the JSON parser enforces a container depth boundary of `9997` open containers, rejects integer literals past `4300` digits, and rejects unpaired UTF-16 surrogates and line-break characters (`\n` and `\r`) in text fields.

## Exploration format (`myriad-exploration` at version 1)

Read by `map`, `check`, `claims`, and `census` (`MY-1` through `MY-6`, `FIX-MY-15`). An exploration records one branching investigation of a single open-ended question as an acyclic forest of branches.

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `question` | string | required | Non-empty single-line question text |
| `branches` | array of objects | required | Zero or more branch records forming a cycle-free forest |
| `branches[].id` | string | required | Unique non-empty single-line branch identifier |
| `branches[].parent` | string or `null` | required | `null` for a root branch, or the `id` of another branch in `branches` |
| `branches[].prompt` | string | required | Non-empty single-line sub-question (no two `open` branches may share the same `prompt` under `MY-3`) |
| `branches[].state` | string | required | Exactly one of `open`, `taken`, or `abandoned` |
| `branches[].finding` | string | conditional | Required and non-empty when `state` is `taken`, forbidden otherwise |
| `branches[].reason` | string | conditional | Required and non-empty when `state` is `abandoned`, forbidden otherwise |
| `branches[].claim` | string | optional | Non-empty single-line worker or agent label holding dibs on this branch (`MY-5`) |

```json
{
  "question": "Which week does the harbour ice survey run",
  "branches": [
    {
      "id": "tide",
      "parent": null,
      "prompt": "Which week gives the lowest spring tide at the outer wall",
      "state": "taken",
      "finding": "The second week of February holds the lowest daytime tide",
      "claim": "hydrography lane"
    },
    {
      "id": "vessel",
      "parent": "tide",
      "prompt": "Is the survey launch free during the second week of February",
      "state": "open",
      "claim": "fleet lane"
    },
    {
      "id": "january",
      "parent": null,
      "prompt": "Can the crew run the soundings in late January instead",
      "state": "abandoned",
      "reason": "The calibration rig arrives on the first of February"
    }
  ]
}
```

## Atlas format (`myriad-atlas` at version 1)

Read by `atlas`, `index`, and `changelog` (`MY-7` through `MY-12`, `FIX-MY-15`). An atlas groups multiple named explorations under a single survey title.

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `title` | string | required | Non-empty single-line atlas title |
| `entries` | array of objects | required | Zero or more labelled exploration entries with unique `label` strings |
| `entries[].label` | string | required | Unique non-empty single-line entry handle |
| `entries[].exploration` | object | required | Valid inline exploration object under the rules above |

```json
{
  "title": "Winter questions for a small harbour",
  "entries": [
    {
      "label": "ice survey",
      "exploration": {
        "question": "Which week does the harbour ice survey run",
        "branches": [
          {
            "id": "tide",
            "parent": null,
            "prompt": "Which week gives the lowest spring tide",
            "state": "open"
          }
        ]
      }
    }
  ]
}
```

## Lineage format (`myriad-lineage` at version 1)

Read by `lineage` and the thirteen temporal chain commands (`returns`, `standing`, `absence`, `arrivals`, `departures`, `matched`, `runs`, `presence`, `standings`, `company`, `sets`, `rosters`, and `fellows`) (`MY-13`, `MY-14`, `FIX-MY-15`). A lineage orders two or more atlases captured across successive checkpoints in time.

| Field | Type | Required | Rule |
| --- | --- | --- | --- |
| `atlases` | array of objects | required | Two or more valid atlas objects in chronological step order |

```json
{
  "atlases": [
    {
      "title": "Harbour survey, week 1",
      "entries": [
        {
          "label": "ice survey",
          "exploration": {
            "question": "Which week does the harbour ice survey run",
            "branches": []
          }
        }
      ]
    },
    {
      "title": "Harbour survey, week 2",
      "entries": [
        {
          "label": "ice survey",
          "exploration": {
            "question": "Which week does the harbour ice survey run",
            "branches": [
              {
                "id": "tide",
                "parent": null,
                "prompt": "Check tide tables",
                "state": "taken",
                "finding": "Second week of February"
              }
            ]
          }
        }
      ]
    }
  ]
}
```
