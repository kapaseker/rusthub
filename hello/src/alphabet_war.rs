use std::cmp::Ordering;

fn try_switch_team(battle_char: &mut [char], me: &[char], opponent: &[char], priest: usize, opponent_priest:&char) {

    if priest > 0 {
        if let Some(last) = battle_char.get(priest - 1) {
            let mut switch = priest <= 1 || if let Some(last_last) = battle_char.get(priest - 2) {
                *last_last != *opponent_priest
            } else {
                true
            };

            if switch {
                if let Some(p) = opponent.iter().position(|s| *s == *last) {
                    battle_char[priest - 1] = me[p];
                }
            }
        }
    }


    if let Some(next) = battle_char.get(priest + 1) {

        let switch = if let Some(next_next) = battle_char.get(priest + 2) {
            *next_next != *opponent_priest
        } else {
            true
        };

        if switch {
            if let Some(p) = opponent.iter().position(|s| *s == *next) {
                battle_char[priest + 1] = me[p];
            }
        }
    }
}

fn alphabet_war(fight: &str) -> &'static str {

    let l_team = ['s', 'b', 'p', 'w'];
    let l_priest = 't';

    let r_team = ['z', 'd', 'q', 'm'];
    let r_priest = 'j';

    let mut battle_char = fight.chars().collect::<Vec<char>>();
    let len = battle_char.len();

    for i in 0..len {

        let c = battle_char[i];
        let l = c == l_priest;
        let r = c == r_priest;

        if l {
            try_switch_team(&mut battle_char, &l_team, &r_team,i,&r_priest);
        } else if r {
            try_switch_team(&mut battle_char, &r_team, &l_team,i,&l_priest);
        }
    }

    let mut  sum = 0i32;

    for i in 0..len {
        let c = battle_char[i];
        if let Some(p) = l_team.iter().position(|s| *s == c) {
            sum += p as i32 + 1
        } else if let Some(p) = r_team.iter().position(|s| *s == c) {
            sum -= p as i32 + 1
        }
    }

    return match sum.cmp(&0) {
        Ordering::Less => "Right side wins!",
        Ordering::Equal => "Let's fight again!",
        Ordering::Greater => "Left side wins!",
    };
}
#[cfg(test)]
mod tests {
    use super::alphabet_war;

    fn dotest(s: &str, expected: &'static str) {
        let actual = alphabet_war(s);
        assert!(actual == expected,
                "With fight = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn dacezzdwmtpjddb_test() {
        dotest("dacezzdwmtpjddb","Left side wins!");
        // dotest("dwmtpjddb","Left side wins!");
    }

    #[test]
    fn random_test() {
        dotest("fsqtjtpzdetfdejmtajcztmjdwa","Right side wins!");
    }

    #[test]
    fn jtsw_test() {
        dotest("jzmzaspmdmwbtzjtmdttpjpdmemtmtstqzqqjmzstw", "Right side wins!");
    }

    #[test]
    fn z_tests() {
        dotest("z", "Right side wins!");
    }

    #[test]
    fn tz_test() {
        dotest("tz", "Left side wins!");
    }

    #[test]
    fn jz_test() {
        dotest("jz", "Right side wins!");
    }

    #[test]
    fn zt_test() {
        dotest("zt", "Left side wins!");
    }

    #[test]
    fn tzj_test() {
        dotest("tzj", "Right side wins!");
    }

    #[test]
    fn fixed_tests() {
        dotest("z", "Right side wins!");
        dotest("tz", "Left side wins!");
        dotest("jbdt", "Let's fight again!");
        dotest("jz", "Right side wins!");
        dotest("zt", "Left side wins!");
        dotest("sj", "Right side wins!");
        dotest("azt", "Left side wins!");
        dotest("tzj", "Right side wins!");
        dotest("wololooooo", "Left side wins!");
        dotest("zdqmwpbs", "Let's fight again!");
        dotest("ztztztzs", "Left side wins!");
    }
}