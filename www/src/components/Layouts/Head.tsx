import React from 'react';
import { Link } from 'react-router-dom';

interface AppTitle {
  title: string,
};

class Header extends React.Component<AppTitle> {
  public state: AppTitle = {
    title: "Title"
  };
  constructor(props: AppTitle) {
    super(props);
    const title: string = this.props.title;
    this.state.title = title;
  };

  public render(){
    return (
      <div className="card-parent" id="top-view">
        <header>
          <div className="card-children">
            <div className="page-top">
              {this.state.title}
            </div>
          </div>
        </header>
      </div>
    );
  };
};

export default Header;
