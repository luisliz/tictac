import 'package:flutter/material.dart';

class LandingView extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Welcome to Our App'),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Text(
              'Welcome to Our App',
              style: Theme.of(context).textTheme.headline4,
            ),
            RaisedButton(
              onPressed: () {
                // TODO: Navigate to Sign In View
              },
              child: Text('Sign In'),
            ),
            RaisedButton(
              onPressed: () {
                // TODO: Navigate to Sign Up View
              },
              child: Text('Sign Up'),
            ),
          ],
        ),
      ),
    );
  }
}
