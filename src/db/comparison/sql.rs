//! SQL queries for comparisons.

// Use https://www.sqlstyle.guide.

/// SQL for getting a personal-best run for a game category.
pub(super) const RUN_PB: &str = "
SELECT run_id
     , run.timestamp AS date
     , SUM(time_ms)  AS total
     -- These bits exist to fill in parts of the run summary that are implicit
     -- in the fact we're looking for a PB.
     , 1 AS rank
     , 1 AS is_completed
  FROM run
       INNER JOIN run_split      USING (run_id)
       INNER JOIN run_split_time USING (run_split_id)
 WHERE game_category_id = :game_category
   AND is_completed = 1
 GROUP BY run_id
 ORDER BY total ASC
 LIMIT 1;";

/// SQL for getting a split PB set for a game category.
pub(super) const SPLIT_PBS: &str = "
SELECT s.short AS short, total
  FROM split_pb
       INNER JOIN segment_split    AS ss USING (split_id)
       INNER JOIN category_segment AS cs USING (segment_id)
       INNER JOIN split            AS s  USING (split_id)
 WHERE game_category_id = :game_category
 GROUP BY split_id, cs.position, ss.position
 ORDER BY cs.position, ss.position;";

/// SQL for getting the sum of best for a game category.
pub(super) const SUM_OF_BEST: &str = "
SELECT SUM(total) AS total
  FROM split_pb
 WHERE game_category_id = :game_category;";
