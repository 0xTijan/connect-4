use std::fmt;


#[derive(Clone, Debug)]
pub struct BitBoard {
    pub player_mask: u128,  // bitboard za igralčeve žetone
    pub ai_mask: u128,      // bitboard za AI žetone
    heights: Vec<u8>,
    pub rows: u8,
    pub cols: u8,
    pub connect: u8,        // zathevana število žetonov za zmago
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Piece {
    Empty,
    Player,
    AI,
}

impl BitBoard {
    // ustvari novo BitBoard z danimi parametri
    pub fn new(rows: u8, cols: u8, connect: u8) -> Self {
        Self {
            player_mask: 0, // bitboard=0 - ni žetonov
            ai_mask: 0,
            heights: vec![0; cols as usize],
            rows,
            cols,
            connect
        }
    }

    // vrne indeks bitov ki je določen s stolpcem in vrstico [0, rows * cols)] - kje v u128 se nahaja bit (celica)
    // biti so razporejeni od leve proti desni, od spodaj navzgor v stolpcu, z enim bitom presledka med stolpci
    fn bit_index(&self, row: u8, col: u8) -> u8 {
        col * (self.rows + 1) + row
    }

    // vrne bitmasko z enim bitom nastavljenim na 1 na poziciji, ki jo določa bit_index (row, col)
    fn bit(&self, row: u8, col: u8) -> u128 {
        1u128 << self.bit_index(row, col)
    }

    // preveri, če je stolpec še prazen
    pub fn is_valid_location(&self, col: u8) -> bool {
        self.heights[col as usize] < self.rows
    }

    // vrne seznam vseh veljavnih stolpcev (kjer lahko žeton spustimo)
    pub fn get_valid_locations(&self) -> Vec<u8> {
        (0..self.cols).filter(|&c| self.is_valid_location(c)).collect()
    }

    // vrne nov BitBoard z dodanim žetonom v stolpec
    pub fn drop_piece(&self, col: u8, piece: Piece) -> Option<Self> {
        // preveri, če je stolpec veljaven (če ni poln)
        if !self.is_valid_location(col) {
            return None;
        }

        // shrani višino stolpca, da veš, kje bo žeton padel - vrstica
        let row = self.heights[col as usize];
        // shrani bitmasko, ki ustreza tej poziciji (lokacija kjer bi žeton padel)
        let bit_to_set = self.bit(row, col);

        let mut new_board = self.clone();
        // povečaj višino stolpca, ker bo padel žeton
        new_board.heights[col as usize] += 1;         

        // nastavi bit v ustreznem bitboardu glede na vrsto žetona (igralec ali AI)
        match piece {
            Piece::Player => new_board.player_mask |= bit_to_set,
            Piece::AI => new_board.ai_mask |= bit_to_set,
            _ => {}
        }

        Some(new_board)
    }

    // vrne vrsto žetona na dani poziciji (vrstica, stolpec)
    pub fn get_piece(&self, row: u8, col: u8) -> Piece {
        let bit = self.bit(row, col);

        if self.player_mask & bit != 0 {
            Piece::Player
        } else if self.ai_mask & bit != 0 {
            Piece::AI
        } else {
            Piece::Empty
        }
    }

    // preveri, če je BitBoard poln (vsi stolpci so polni)
    pub fn is_full(&self) -> bool {
        self.heights.iter().all(|&h| h >= self.rows)
    }

    // preveri, če je določen žeton zmagal
    pub fn check_win(&self, piece: Piece) -> bool {
        // izberi masko glede na vrsto žetona
        let mask = match piece {
            Piece::Player => self.player_mask,
            Piece::AI => self.ai_mask,
            _ => 0,
        };

        let row_stride = self.rows + 1; // 1 bit presledek
        // možne smeri za preverjanje zmage
        let directions = [
            1,                  // navpično
            row_stride,         // vodoravno (preskakovanje stolpcev)
            row_stride - 1,     // diagonalno / (en gor, en desno)
            row_stride + 1      // diagonalno \ (en dol, en desno)
        ];
        
        // preveri vsako smer
        for dir in directions {
            let mut current = mask;

            // večkrateni shift in AND operacije na maski za preverjanje zaporedja žetonov - ponovi za connect-1 krat
            // primer: connect 4 => 3 krat shift in AND
            for _ in 0..self.connect - 1 {
                // shiftaj masko v smeri dir enkrat
                current = current & (current >> dir);
            }

            // če so prisotni biti v maski, potem je zmagovalna kombinacija
            if current != 0 {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for r in (0..self.rows).rev() {
            for c in 0..self.cols {
                let symbol = match self.get_piece(r, c) {
                    Piece::Empty => '.',
                    Piece::Player => 'X',
                    Piece::AI => 'O',
                };
                write!(f, " {} ", symbol)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "{}", "-".repeat((self.cols as usize) * 3))?;

        for c in 0..self.cols {
            write!(f, " {} ", c)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
