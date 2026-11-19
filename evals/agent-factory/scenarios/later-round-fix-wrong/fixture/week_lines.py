def week_lines(rows, day):
    """The report of the week that holds `day`: a line for each project
    with hours in it, in alphabetical order, then the total."""
    monday, sunday = week_of(day)
    totals = {}
    for when, project, hours in rows:
        if monday <= when <= sunday:
            totals[project] = totals.get(project, 0) + hours
    if not totals:
        return [f"no hours in the week of {monday}"]
    lines = [f"{project} {fmt(hours)}" for project, hours in sorted(totals.items())]
    return lines + [f"total {fmt(sum(totals.values()))}"]
