import csv


def get_host_league(team_name: str):
    d = {
        "ba": "Ballard",
        "ma": "Magnolia",
        "nc": "North Central",
        "ne": "Northeast",
        "nw": "Northwest",
        "qa": "Queen Anne",
        "ru": "RUG",
        "sl": "Shoreline",
    }
    abr = team_name[:2].lower()
    return d[abr]


def get_type(home_team, away_team):
    if home_team[:2] == away_team[:2]:
        return "Divisional"
    else:
        return "Interleague"


rows = []

with open("a.csv", "r") as f:
    reader = csv.reader(f)
    for row in reader:
        rows.append(row)

print(rows)
away_teams = rows[0]

with open("out.csv", "w") as f:
    writer = csv.writer(f)
    writer.writerow(["Date", "Day", "Home", "Away", "Host League", "Type"])
    for row in rows[1:]:
        date = row[0]
        day = row[1]
        for i in range(2, len(row)):
            home_team = row[i]
            if home_team and home_team != "bye":
                away_team = away_teams[i]
                host_league = get_host_league(home_team)
                type = get_type(home_team, away_team)
                writer.writerow([date, day, home_team, away_team, host_league, type])
