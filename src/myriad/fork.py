"""The fork helper for myriad.

Branch generation in slice 2 is recorded, not live. This module
builds the exploration skeleton a live forker would emit, so the
shape of a fork is pinned without any live call. The obligations
this module carries are MY-4 in CONTRACT.md, in the full form
section.
"""


def _holds_unpaired_surrogate(text):
    """Return whether a string fails UTF-8 encoding.

    The same encoding test the loader in myriad.explorations uses,
    so an input the loader would name as malformed is refused here
    before a skeleton is built on it.
    """
    try:
        text.encode("utf-8")
    except UnicodeEncodeError:
        return True
    return False


def _holds_line_break(text):
    """Return whether a string holds a newline or a carriage return.

    The loader names such a value as malformed under FIX-MY-6, so a
    skeleton built on one could never load clean, and the helper
    refuses it before building, under FIX-MY-8.
    """
    return "\n" in text or "\r" in text


def fork_skeleton(question, prompts):
    """Build an exploration skeleton from a question and prompts.

    The skeleton holds one root branch per prompt, every branch in
    state open, with ids in the form b01, b02, and onward in prompt
    order. Raises ValueError on an empty question, on a question or
    a prompt holding an unpaired surrogate, on a question or a
    prompt holding a newline or a carriage return, on a prompt
    that is not a string, on an empty prompt, and on two prompts
    that are the same, because no sound skeleton exists for those
    inputs.
    """
    if not isinstance(question, str) or question == "":
        raise ValueError("the question must be a non empty string")
    if _holds_unpaired_surrogate(question):
        raise ValueError("the question holds an unpaired surrogate")
    if _holds_line_break(question):
        raise ValueError(
            "the question holds a newline or a carriage return"
        )
    branches = []
    seen = set()
    for index, prompt in enumerate(prompts, start=1):
        if not isinstance(prompt, str):
            raise ValueError("each prompt must be a string")
        if prompt == "":
            raise ValueError("each prompt must be a non empty string")
        if _holds_unpaired_surrogate(prompt):
            raise ValueError("each prompt must hold no unpaired surrogate")
        if _holds_line_break(prompt):
            raise ValueError(
                "each prompt must hold no newline or carriage return"
            )
        if prompt in seen:
            raise ValueError(f"two prompts are the same, {prompt}")
        seen.add(prompt)
        branches.append(
            {
                "id": f"b{index:02d}",
                "parent": None,
                "prompt": prompt,
                "state": "open",
            }
        )
    return {"question": question, "branches": branches}
