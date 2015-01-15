#[macro_export]
macro_rules! query_map {
($conn:ident, $query:expr, $params:expr, $T:ident, $($member:ident),* ) => {
		{		
			let stmt = $conn.prepare($query)
					.unwrap();
				
			let result = stmt.query($params).unwrap()
				.map(|row:postgres::Row| $T {
						$( $member: row.get(stringify!($member)) ),*
					});
			result.collect::<Vec<$T>>()			
		}
	}
}

