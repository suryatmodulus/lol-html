macro_rules! ch_sequence_arm_pattern {

    // Sequences
    //--------------------------------------------------------------------
    ( | $scope_vars:tt |> "--", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'-', b'-' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "DOCTYPE", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'D', b'O', b'C', b'T', b'Y', b'P', b'E' ], $($rest_args)*
        );
    };

    ( | $scope_vars:tt |> "[CDATA[", $($rest_args:tt)* )  => {
        ch_sequence_arm_pattern!(
            @first |$scope_vars|> [ b'[', b'C', b'D', b'A', b'T', b'A', b'[' ], $($rest_args)*
        );
    };


    // Character comparison expression
    //--------------------------------------------------------------------
    ( @cmp_exp $ch:ident, $exp_ch:expr ) => ( $ch == $exp_ch );
    ( @cmp_exp $ch:ident, $exp_ch:expr, ignore_case ) => ( $ch == $exp_ch || $ch == $exp_ch ^ 0x20 );


    // Match block expansion
    //--------------------------------------------------------------------
    ( @match_block $ch:expr, $exp_ch:expr, $body:tt, $($case_mod:ident)* ) => {
        match $ch {
            Some(ch) if ch_sequence_arm_pattern!(@cmp_exp ch, $exp_ch $(, $case_mod)*) => {
               $body
            },
            _ => ()
        }
    };


    // Expand check for the first character
    //--------------------------------------------------------------------
    ( @first | [$self:tt, $ch:ident] |>
        [ $exp_ch:expr, $($rest_chs:tt)* ], $actions:tt, $($case_mod:ident)*
    ) => {
        ch_sequence_arm_pattern!(@match_block $ch, $exp_ch, {
            ch_sequence_arm_pattern!(
                @iter |[$self, $ch]|> 1, [ $($rest_chs)* ], $actions, $($case_mod)*
            );
        }, $($case_mod)*);
    };


    // Recursively expand checks for the remaining characters
    //--------------------------------------------------------------------
    ( @iter | [$self:tt, $ch:ident] |>
        $depth:expr, [ $exp_ch:expr, $($rest_chs:tt)* ], $actions:tt, $($case_mod:ident)*
    ) => {
        ch_sequence_arm_pattern!(@match_block $self.buffer.peek_at($self.pos + $depth), $exp_ch, {
            ch_sequence_arm_pattern!(
                @iter |[$self, $ch]|> $depth + 1, [ $($rest_chs)* ], $actions, $($case_mod)*
            );
        }, $($case_mod)*);
    };

    // NOTE: end of recursion
    ( @iter | [$self:tt, $ch:ident] |>
        $depth:expr, [$exp_ch:expr], ( $($actions:tt)* ), $($case_mod:ident)*
    ) => {
        ch_sequence_arm_pattern!(@match_block $self.buffer.peek_at($self.pos + $depth), $exp_ch, {
            $self.pos += $depth;
            action_list!(|$self|> $($actions)*);
            return;
        }, $($case_mod)*);
    };
}