





# A1 Paper






![alt text][img1]


Illustration of the A series paper sizes by [Bromskloss](http://commons.wikimedia.org/wiki/File:A_size_illustration2.svg)



Björn likes the square root of two very much. He likes it so much that he has decided to write down the first 10,000 digits of it on a single paper. He started doing this on an A4 paper, but ran out of space after writing down only 1250 digits. Being pretty good at math, he quickly figured out that he needs an A1 paper to fit all the digits. Björn doesn’t have an A1 paper, but he has smaller papers which he can tape together.

Taping two A2 papers together along their long side turns them into an A1 paper, two A3 papers give an A2 paper, and so on. Given the number of papers of different sizes that Björn has, can you figure out how much tape he needs to make an A1 paper? Assume that the length of tape needed to join together two sheets of papers is equal to their long side. An A2 paper is 2<sup>-5/4</sup> meters by 2<sup>-3/4</sup> meters and each consecutive paper size (A3, A4, ) have the same shape but half the area of the previous one.

## Input

The first line of input contains a single integer 2<= n <= 30, the A-size of the smallest papers Björn has. The second line contains n-1 integers giving the number of sheets he has of each paper size starting with A2 and ending with An. Björn doesn’t have more than 10<sup>9</sup> sheets of any paper size.

## Output

If Björn has enough paper to make an A1 paper, output a single floating point number, the smallest total length of tape needed in meters. Otherwise output “impossible”. The output number should have an absolute error of at most 10<sup>-5</sup>.

<table class="sample" summary="sample data">

<tbody>

<tr>

<th>Sample Input 1</th>

<th>Sample Output 1</th>

</tr>

<tr>

<td>

<pre>4
1 0 5
</pre>

</td>

<td>

<pre>1.60965532263
</pre>

</td>

</tr>

</tbody>

</table>

<table class="sample" summary="sample data">

<tbody>

<tr>

<th>Sample Input 2</th>

<th>Sample Output 2</th>

</tr>

<tr>

<td>

<pre>3
0 3
</pre>

</td>

<td>

<pre>impossible
</pre>

</td>

</tr>

</tbody>

</table>



[img1]: .\img1.png
