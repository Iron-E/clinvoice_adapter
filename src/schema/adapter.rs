use super::{
	ContactAdapter,
	EmployeeAdapter,
	ExpensesAdapter,
	JobAdapter,
	LocationAdapter,
	OrganizationAdapter,
	TimesheetAdapter,
};
use crate::{Deletable, Initializable};

/// A complete implementation of the [`winvoice_adapter::schema`].
pub trait Adapter: Initializable
{
	/// The adapter for [`Contact`](winvoice_schema::Contact)s
	type Contact: Deletable<Db = Self::Db> + ContactAdapter;

	/// The adapter for [`Employee`](winvoice_schema::Employee)s
	type Employee: Deletable<Db = Self::Db> + EmployeeAdapter;

	/// The adapter for [`Expense`](winvoice_schema::Expense)s
	type Expenses: Deletable<Db = Self::Db> + ExpensesAdapter;

	/// The adapter for [`Job`](winvoice_schema::Job)s
	type Job: Deletable<Db = Self::Db> + JobAdapter;

	/// The adapter for [`Location`](winvoice_schema::Location)s
	type Location: Deletable<Db = Self::Db> + LocationAdapter;

	/// The adapter for [`Organization`](winvoice_schema::Organization)s
	type Organization: Deletable<Db = Self::Db> + OrganizationAdapter;

	/// The adapter for [`Timesheet`](winvoice_schema::Timesheet)s
	type Timesheet: Deletable<Db = Self::Db> + TimesheetAdapter;
}
