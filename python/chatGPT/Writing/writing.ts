import * as React from 'react';

interface JournalEntry {
  date: Date;
  entry: string;
}

interface JournalState {
  entries: JournalEntry[];
  currentEntry: string;
  currentStreak: number;
  points: number;
}

class Journal extends React.Component<{}, JournalState> {
  constructor(props: {}) {
    super(props);
    this.state = {
      entries: [],
      currentEntry: '',
      currentStreak: 0,
      points: 0
    };
  }

  handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    this.setState({ currentEntry: event.target.value });
  }

  handleSubmit(event: React.FormEvent<HTMLFormElement>) {
    event.preventDefault();
    const entry: JournalEntry = {
      date: new Date(),
      entry: this.state.currentEntry
    };
    this.setState({
      entries: [...this.state.entries, entry],
      currentEntry: '',
      currentStreak: this.state.currentStreak + 1,
      points: this.state.points + (this.state.currentEntry.length >= 200 ? 1 : 0)
    });
    if (this.state.currentStreak === 4) {
      this.setState({ points: this.state.points + 50 });
    }
  }

  render() {
    return (
      <div>
        <h1>Journal</h1>
        <p>Current Streak: {this.state.currentStreak}</p>
        <p>Points: {this.state.points}</p>
        <form onSubmit={this.handleSubmit}>
          <label>
            New Entry:
            <input
              type="text"
              value={this.state.currentEntry}
              onChange={this.handleChange}
            />
          </label>
          <input type="submit" value="Submit" />
        </form>
      </div>
    );
  }
}
