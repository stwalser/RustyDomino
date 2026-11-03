# Improvements

- Choose how many rounds should be played or until which number of points

- print_points — tie-breaking for leader is ambiguous (lines 164-173)
- Queue::is_playable — unwrap() on expected_start_stone (line 87)
- Repeated panic messages in player.rs
- wait_for_press_of in client.rs — raw mode not restored on panic
- Client and Player both maintain stone lists
