// Example code from website below:
// https://code.tutsplus.com/tutorials/adding-swipe-gestures-to-recyclerviews--cms-32427

package com.example.swipeexample

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import androidx.fragment.app.Fragment
import com.example.swipeexample.dummy.DummyContent

class MainActivity : AppCompatActivity() , ItemFragment.OnListFragmentInteractionListener {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }
    override fun onListFragmentInteraction(
        item: DummyContent.DummyItem?) {
        // leave empty
    }




    }
