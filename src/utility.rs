pub fn char_frequency(mut chars: Vec<char>) -> Vec<char> {
    let frequency_order = [
        'e', 't', 'a', 'o', 'i', 'n', 's', 'h', 'r', 'd', 'l', 'c',
        'u', 'm', 'w', 'f', 'g', 'y', 'p', 'b', 'v', 'k', 'j', 'x', 'q', 'z',
    ];

    let mut freq_rank = [0usize; 26];
    for (rank, ch) in frequency_order.iter().enumerate() {
        freq_rank[*ch as usize - b'a' as usize] = rank;
    }

    chars.sort_by_key(|&ch| {
        if ch >= 'a' && ch <= 'z' {
            freq_rank[ch as usize - b'a' as usize]
        } else {
            usize::MAX 
        }
    });

    chars
}
