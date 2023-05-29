use sqlx::Database;

use crate::Deletable;

use super::{
	ContactAdapter,
	EmployeeAdapter,
	ExpensesAdapter,
	JobAdapter,
	LocationAdapter,
	OrganizationAdapter,
	TimesheetAdapter,
};

/// A complete implementation of the [`winvoice_adapter::schema`].
pub trait Adapter
{
	/// The [`Database`] that is used for this adapter.
	type Db: Database;

	/// The adapter for [`Contact`](winvoice_schema::Contact)s
	type Contact: Deletable<Db = Db> + ContactAdapter;

	/// The adapter for [`Employee`](winvoice_schema::Employee)s
	type Employee: Deletable<Db = Db> + EmployeeAdapter;

	/// The adapter for [`Expense`](winvoice_schema::Expense)s
	type Expenses: Deletable<Db = Db> + ExpensesAdapter;

	/// The adapter for [`Job`](winvoice_schema::Job)s
	type Job: Deletable<Db = Db> + JobAdapter;

	/// The adapter for [`Location`](winvoice_schema::Location)s
	type Location: Deletable<Db = Db> + LocationAdapter;

	/// The adapter for [`Organization`](winvoice_schema::Organization)s
	type Organization: Deletable<Db = Db> + OrganizationAdapter;

	/// The adapter for [`Timesheet`](winvoice_schema::Timesheet)s
	type Timesheet: Deletable<Db = Db> + TimesheetAdapter;
}
