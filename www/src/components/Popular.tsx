import React from 'react';
import * as wasm from 'math-net';

interface Grade {
  grade: number,
  book_id: number,
  title: string,
  num_posts: number,
}

class PupularCard extends React.Component {
  public grade_1: Grade = {
    grade: 1,
    book_id: 0,
    title: "default",
    num_posts: 0,
  };

  public grade_2: Grade = {
    grade: 2,
    book_id: 0,
    title: "default",
    num_posts: 0,
  };

  public grade_3: Grade = {
    grade: 3,
    book_id: 0,
    title: "default",
    num_posts: 0,
  };

  constructor(grade_1: Grade, grade_2: Grade, grade_3: Grade) {
    super(grade_1);
    super(grade_2);
    super(grade_3);

    const json = wasm.get_popular_books();
    const obj = JSON.parse(json);
    let json_grade_1 = obj[0];
    let json_grade_2 = obj[1];
    let json_grade_3 = obj[2];
    json_grade_1 = JSON.parse(json_grade_1);
    grade_1.book_id = json_grade_1.book_id;
    grade_1.title = json_grade_1.title;
    grade_1.num_posts = json_grade_1.num_posts;
    grade_2.book_id = json_grade_2.book_id;
    grade_2.title = json_grade_2.title;
    grade_2.num_posts = json_grade_2.num_posts;
    grade_3.book_id = json_grade_3.book_id;
    grade_3.title = json_grade_3.title;
    grade_3.num_posts = json_grade_3.num_posts;
    /*
    console.log(json_grade_1);
    console.log(json_grade_2);
    */
  };

  public render() {
    return (
      <div className="card">
        <PupularCardChildren grade={this.grade_1.grade} title={this.grade_1.title}
          book_id={this.grade_1.book_id} num_posts={this.grade_1.num_posts}
        />
        <PupularCardChildren grade={this.grade_2.grade} title={this.grade_2.title}
          book_id={this.grade_2.book_id} num_posts={this.grade_2.num_posts}
        />
        <PupularCardChildren grade={this.grade_3.grade} title={this.grade_3.title}
          book_id={this.grade_3.book_id} num_posts={this.grade_3.num_posts}
        />
      </div>
    )
  }
}

class PupularCardChildren extends React.Component<Grade> {
  public state: Grade = {
    grade: 0,
    book_id: 0,
    title: "default",
    num_posts: 0,
  };

  constructor(props: Grade) {
    super(props);
    const grade = this.props.grade;
    const book_id = this.props.book_id;
    const title = this.props.title;
    const num_posts = this.props.num_posts;

    this.state.grade = grade;
    this.state.book_id = book_id;
    this.state.title = title;
    this.state.num_posts = num_posts;
  };

  public render(){
    return (
      <div className="card">
        {this.state.grade}
        {this.state.book_id}
        {this.state.title}
        {this.state.num_posts}
      </div>
    );
  };
};

export default PupularCard;
