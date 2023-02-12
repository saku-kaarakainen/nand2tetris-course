library IEEE;
use IEEE.std_logic_1164.all;

entity xor_gate is
  port (
    a, b: in std_logic;
    y: out std_logic
  );
end entity xor_gate;

architecture behavioral of xor_gate is
  signal not_a: std_logic;
  signal not_b: std_logic;
  signal a_and_b: std_logic;
  signal not_a_and_not_b: std_logic;
begin
  not_gate_1: entity work.not_gate
    port map (
      a => a,
      y => not_a
    );
  
  not_gate_2: entity work.not_gate
    port map (
      a => b,
      y => not_b
    );
  
  and_gate_1: entity work.and_gate
    port map (
      a => a,
      b => b,
      y => a_and_b
    );
  
  and_gate_2: entity work.and_gate
    port map (
      a => not_a,
      b => not_b,
      y => not_a_and_not_b
    );
  
  or_gate: entity work.or_gate
    port map (
      a => a_and_b,
      b => not_a_and_not_b,
      y => y
    );
end architecture behavioral;
